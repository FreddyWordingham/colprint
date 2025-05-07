//! Defines the available formatting types for columns.
//!
//! This module contains the `FormatType` enum, which represents the different
//! ways an item can be formatted:
//! - `Display`: Standard formatting using the `Display` trait.
//! - `Debug`: Debug formatting using the `Debug` trait with `{:?}` format.
//! - `PrettyDebug`: Pretty debug formatting using the `Debug` trait with `{:#?}` format.
//!
//! The format type is determined by the format specifier used in the format string
//! and controls how items are rendered in the output.

/// Different formatting types.
#[derive(Debug, PartialEq, Eq)]
pub enum FormatType {
    /// Custom formatting.
    Display,
    /// Debug formatting with `:?`.
    Debug,
    /// Pretty debug formatting with `:#?`.
    PrettyDebug,
}
