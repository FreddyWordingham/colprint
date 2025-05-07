//! Implementation of the core column formatting logic.
//!
//! This module provides the `ColumnFormatter` struct, which is responsible for:
//! - Parsing format strings into column specifications.
//! - Applying the appropriate formatting to each item.
//! - Handling column width calculations (both automatic and specified).
//! - Aligning and printing items in columns with proper separators.
//! - Managing multi-line content in columns.
//!
//! The `ColumnFormatter` serves as the engine behind the `colprint!` macro, translating
//! high-level formatting directives into properly formatted columnar output.

use std::{
    cmp::min,
    fmt::{self, Display, Formatter, Result as FmtResult},
    io::{self, Write},
};

use crate::{FormattableItem, column_format::ColumnFormat, format_part::FormatPart, format_type::FormatType};

/// A formatter for creating columnar output.
pub struct ColumnFormatter<'a> {
    /// The kind of format for each column.
    formats: Vec<ColumnFormat>,
    /// The items to format.
    items: Vec<FormattableItem<'a>>,
}

impl<'a> ColumnFormatter<'a> {
    /// Construct a new `ColumnFormatter` instance.
    #[must_use]
    #[inline]
    pub fn new(format_str: &str, items: Vec<FormattableItem<'a>>) -> Self {
        Self {
            formats: Self::parse_format_string(format_str),
            items,
        }
    }

    /// Parse a format string like "{} | {:?} | {:#?:80}" into column formats.
    #[expect(clippy::single_call_fn, reason = "This function makes initialisation logic cleaner.")]
    fn parse_format_string(format_str: &str) -> Vec<ColumnFormat> {
        let mut formats = Vec::new();
        let mut parts = Vec::new();

        // First, split the format string into parts (format specifiers and separators)
        let mut in_format = false;
        let mut start_byte_idx = 0;

        // Use char_indices to safely navigate UTF-8 characters
        for (i, c) in format_str.char_indices() {
            if c == '{' && !in_format {
                // Start of a format specifier
                if i > start_byte_idx {
                    // There's a separator before this format specifier
                    if let Some(separator) = format_str.get(start_byte_idx..i) {
                        parts.push(FormatPart::Separator(separator));
                    }
                }
                start_byte_idx = i;
                in_format = true;
            } else if c == '}' && in_format {
                // End of a format specifier
                in_format = false;
                let end_byte_idx = i + c.len_utf8(); // Properly account for character length

                // Check for width specification after the format
                let mut width_end_byte_idx = end_byte_idx;
                let format_bytes = format_str.as_bytes();

                // Safely check for colon
                if end_byte_idx < format_str.len() && format_bytes[end_byte_idx] == b':' {
                    width_end_byte_idx = end_byte_idx + 1;

                    // Safely collect width digits
                    while width_end_byte_idx < format_str.len() && format_bytes[width_end_byte_idx].is_ascii_digit() {
                        width_end_byte_idx += 1;
                    }

                    // Ensure we're at UTF-8 boundaries before slicing
                    let start_str = format_str.get(start_byte_idx..end_byte_idx).unwrap_or_default();
                    let width_str = format_str.get(end_byte_idx + 1..width_end_byte_idx).unwrap_or_default();

                    parts.push(FormatPart::Format(start_str, Some(width_str)));
                } else {
                    let format_slice = format_str.get(start_byte_idx..end_byte_idx).unwrap_or_default();
                    parts.push(FormatPart::Format(format_slice, None));
                }

                start_byte_idx = width_end_byte_idx;
            }
        }

        // Add any trailing separator
        if start_byte_idx < format_str.len() {
            if let Some(trailing) = format_str.get(start_byte_idx..) {
                parts.push(FormatPart::Separator(trailing));
            }
        }

        // Now process the parts to create column formats
        for (i, part) in parts.iter().enumerate() {
            if let FormatPart::Format(fmt_str, width_str) = *part {
                // Determine format type
                let format_type = if fmt_str.contains(":#?") {
                    FormatType::PrettyDebug
                } else if fmt_str.contains(":?") {
                    FormatType::Debug
                } else {
                    FormatType::Display
                };

                // Parse width if specified
                let width = width_str.and_then(|w| w.parse::<usize>().ok());

                // Check for separator after this format
                let separator = if i + 1 < parts.len() {
                    if let FormatPart::Separator(sep) = parts[i + 1] {
                        Some(sep.to_owned())
                    } else {
                        None
                    }
                } else {
                    None
                };

                formats.push(ColumnFormat {
                    format_type,
                    width,
                    separator,
                });
            }
        }

        formats
    }

    /// Format items into columns and write to a buffer.
    #[expect(clippy::match_same_arms, reason = "Clippy /may/ be incorrect here.")]
    #[expect(clippy::pattern_type_mismatch, reason = "Priority of arms is important.")]
    fn format_columns(&self, writer: &mut impl Write) -> io::Result<()> {
        // Ensure we have the same number of formatters and items
        let num_items = min(self.formats.len(), self.items.len());

        if num_items == 0 {
            return Ok(());
        }

        // Format each item according to its format type
        let formatted_items: Vec<Vec<String>> = self
            .formats
            .iter()
            .zip(self.items.iter())
            .take(num_items)
            .map(|(fmt, item)| {
                let formatted = match (item, &fmt.format_type) {
                    (FormattableItem::DisplayItem(i), FormatType::Display) => {
                        format!("{i}")
                    }
                    (FormattableItem::DebugItem(i), FormatType::Debug) => {
                        format!("{i:?}")
                    }
                    (FormattableItem::DebugItem(i), FormatType::PrettyDebug) => {
                        format!("{i:#?}")
                    }
                    // Fallback cases - use what we have
                    (FormattableItem::DisplayItem(i), _) => {
                        format!("{i}")
                    }
                    (FormattableItem::DebugItem(i), FormatType::Display) => {
                        format!("{i:?}") // Use debug format as fallback
                    }
                };

                formatted.lines().map(ToOwned::to_owned).collect()
            })
            .collect();

        // Find the max number of lines
        let max_lines = formatted_items.iter().map(Vec::len).max().unwrap_or(0);

        // Calculate column widths (use specified width or auto-calculate)
        let column_widths: Vec<usize> = self
            .formats
            .iter()
            .take(num_items)
            .enumerate()
            .map(|(idx, fmt)| {
                // Use specified width or calculate based on content
                fmt.width.unwrap_or_else(|| {
                    formatted_items.get(idx).map_or(0, |item_lines| {
                        item_lines.iter().map(|line| line.chars().count()).max().unwrap_or(0)
                    })
                })
            })
            .collect();

        // For each line, concatenate the corresponding line from each item
        for line_idx in 0..max_lines {
            for (item_idx, item_lines) in formatted_items.iter().enumerate().take(num_items) {
                let column_width = *column_widths.get(item_idx).unwrap_or(&0);

                let line = if line_idx < item_lines.len() {
                    // Truncate or pad the line to fit the column width
                    let mut line = item_lines[line_idx].clone();
                    let line_len = line.chars().count();

                    if line_len > column_width {
                        // Truncate to column width (handling Unicode)
                        let mut chars = line.chars().collect::<Vec<_>>();
                        chars.truncate(column_width);
                        line = chars.into_iter().collect();
                    } else {
                        // Pad to column width
                        line.push_str(&" ".repeat(column_width - line_len));
                    }
                    line
                } else {
                    // Empty line if this item doesn't have this many lines
                    " ".repeat(column_width)
                };

                write!(writer, "{line}")?;

                // Add separator if not the last column
                if item_idx < num_items - 1 {
                    if let Some(separator) = &self.formats[item_idx].separator {
                        write!(writer, "{separator}")?;
                    }
                }
            }
            writeln!(writer)?;
        }

        Ok(())
    }
}

impl Display for ColumnFormatter<'_> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        // Buffer to collect the output
        let mut buffer = Vec::new();

        // Format the items into columns
        if self.format_columns(&mut buffer).is_err() {
            return Err(fmt::Error);
        }

        // Write the buffer to the formatter
        String::from_utf8(buffer).map_or(Err(fmt::Error), |s| write!(f, "{s}"))
    }
}
