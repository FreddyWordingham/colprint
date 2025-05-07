//! Defines the format specification for individual columns.
//!
//! This module contains the `ColumnFormat` struct, which encapsulates the formatting rules
//! for a single column, including the type of formatting to use (`Display`, `Debug`, or `PrettyDebug`),
//! an optional width constraint, and an optional separator to print after the column.
//!
//! `ColumnFormat` instances are typically created internally by parsing format strings
//! and are used by the `ColumnFormatter` to control the output appearance.

use crate::format_type::FormatType;

/// Describes the format for a single column.
pub struct ColumnFormat {
    /// The type of formatting to use
    pub format_type: FormatType,
    /// Optional width for the column
    pub width: Option<usize>,
    /// Optional separator to print after this column
    pub separator: Option<String>,
}
