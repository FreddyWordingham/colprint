//! Provides conversion functionality to `FormattableItem`.
//!
//! This module defines the `ToFormattableItem` trait, which provides methods to convert
//! any type that implements both `Display` and `Debug` into a `FormattableItem`.
//!
//! The trait offers two conversion methods:
//! - `to_display_item`: Converts the item to a `FormattableItem::DisplayItem`
//! - `to_debug_item`: Converts the item to a `FormattableItem::DebugItem`
//!
//! This trait is implemented for all types that implement both `Display` and `Debug`,
//! making it easy for the `colprint!` macro to wrap items appropriately based on
//! the specified format.

use std::fmt::{Debug, Display};

use crate::FormattableItem;

/// Helper trait to convert any type to `FormattableItem`.
pub trait ToFormattableItem {
    fn to_display_item(&self) -> FormattableItem<'_>;
    fn to_debug_item(&self) -> FormattableItem<'_>;
}

impl<T: Display + Debug> ToFormattableItem for T {
    #[inline]
    fn to_display_item(&self) -> FormattableItem<'_> {
        FormattableItem::DisplayItem(self)
    }

    #[inline]
    fn to_debug_item(&self) -> FormattableItem<'_> {
        FormattableItem::DebugItem(self)
    }
}
