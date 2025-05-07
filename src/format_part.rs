//! Defines parsing elements for format string processing.
//!
//! This module contains the `FormatPart` enum, which is used during the format string
//! parsing process to represent different parts of the format string:
//! - Format specifications like `{}`, `{:?}`, or `{:#?}` with optional width parameters
//! - Separator text between columns
//!
//! These elements are used internally by the `ColumnFormatter` when parsing format
//! strings to create the appropriate column formats.

/// Helper enum for parsing format strings.
pub enum FormatPart<'a> {
    /// Format specifier with optional width
    Format(&'a str, Option<&'a str>),
    /// Separator between columns
    Separator(&'a str),
}
