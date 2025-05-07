//! Provides a wrapper for items that can be formatted.
//!
//! This module contains the `FormattableItem` enum, which serves as a type-erased
//! wrapper for items that implement either the `Display` or `Debug` traits. It allows
//! the `ColumnFormatter` to store and format heterogeneous collections of items.
//!
//! The enum variants correspond to the different formatting capabilities:
//! - `DisplayItem`: Wraps an item that implements the `Display` trait
//! - `DebugItem`: Wraps an item that implements the `Debug` trait
//!
//! This abstraction enables the `colprint!` macro to handle mixed formatting types
//! within a single output.

use std::fmt::{Debug, Display};

/// A wrapper that formats both Display and Debug trait objects.
#[non_exhaustive]
pub enum FormattableItem<'a> {
    DisplayItem(&'a dyn Display),
    DebugItem(&'a dyn Debug),
}
