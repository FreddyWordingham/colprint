//! Provides the main `colprint!` macro for printing data in columns.
//!
//! This module contains the implementation of the `colprint!` macro, which is the primary
//! interface for users of this crate. The macro processes format strings to determine column
//! layouts and formatting options, then delegates to the `ColumnFormatter` for actual rendering.
//!
//! The macro supports various formatting options including regular Display format (`{}`),
//! Debug format (`{:?}`), and Pretty Debug format (`{:#?}`), with optional width specifications
//! and custom separators between columns.

/// Macro for printing items in columns using a format string.
///
/// The format string specifies how each item should be displayed:
/// - `{}` for regular Display
/// - `{:?}` for Debug
/// - `{:#?}` for pretty Debug
///
/// You can also specify a width for each column by adding a colon and a number after the format:
/// - `{:80}` for Display with width 80
/// - `{:?:60}` for Debug with width 60
/// - `{:#?:100}` for pretty Debug with width 100
///
/// Any text between format specifications will be used as column separators:
/// - `{} | {}` will print a pipe with spaces between columns
/// - `{}  {}` will print two spaces between columns
/// - `{:?} -> {:#?}` will print an arrow between columns
///
/// # Examples
///
/// ```
/// // Basic usage with Display
/// colprint!("{}{}", item1, item2);
///
/// // Using Debug format with separators
/// colprint!("{:?} | {:?}", item1, item2);
///
/// // Using pretty Debug with specific widths and separators
/// colprint!("{:#?:80} || {:#?:60}", item1, item2);
///
/// // Mixed formats with decorative separators
/// colprint!("{} -> {:?} => {:#?}", item1, item2, item3);
/// ```
#[macro_export]
macro_rules! colprint {
    ($fmt:expr, $($item:expr),* $(,)?) => {
        {
            let fmt_str = $fmt;
            let mut items = Vec::new();
            let mut format_specs = Vec::new();

            // Extract all format specifiers (e.g., "{}", "{:?}", "{:#?}")
            let mut in_format = false;
            let mut start = 0;

            for (i, c) in fmt_str.char_indices() {
                if c == '{' && !in_format {
                    // Start of a format specifier
                    start = i;
                    in_format = true;
                } else if c == '}' && in_format {
                    // End of a format specifier
                    let end = i + 1;
                    let fmt_spec = &fmt_str[start..end];
                    format_specs.push(fmt_spec);
                    in_format = false;
                }
            }

            // Create FormattableItems based on format specs
            let mut idx = 0;
            $(
                if idx < format_specs.len() {
                    let spec = format_specs[idx];
                    if spec.contains(":#?") || spec.contains(":?") {
                        items.push($crate::FormattableItem::DebugItem(&$item));
                    } else {
                        items.push($crate::FormattableItem::DisplayItem(&$item));
                    }
                    idx += 1;
                }
            )*

            // Create and use the formatter
            let formatter = $crate::ColumnFormatter::new(fmt_str, items);
            println!("{}", formatter);
        }
    };
}
