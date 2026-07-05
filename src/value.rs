//! The [`FormatValue`] marker trait and zero-allocation argument storage.

use core::fmt::{Debug, Display};

/// Marker trait for values that can be formatted at runtime.
///
/// Blanket-implemented for all `T: Display + Debug` (including unsized types
/// such as `str`), which covers the vast majority of Rust types (`i32`, `f64`,
/// `String`, `&str`, `bool`, `char`, custom types with `#[derive(Debug)]` and a
/// `Display` impl, etc.).
pub trait FormatValue: Display + Debug {}

impl<T: Display + Debug + ?Sized> FormatValue for T {}

pub enum FormatArg<'a> {
    /// Any `Sized` value borrowed as a trait object.
    Dyn(&'a dyn FormatValue),
    /// A string slice, stored by value.
    Str(&'a str),
}

impl<'a> FormatArg<'a> {
    /// Borrow this argument as a `&dyn FormatValue` for formatting.
    #[inline]
    pub(crate) fn as_value(&self) -> &dyn FormatValue {
        match self {
            FormatArg::Dyn(value) => *value,
            FormatArg::Str(s) => s,
        }
    }
}

/// Conversion into a [`FormatArg`] without allocation.
pub trait IntoFormatArg<'a> {
    /// Convert `self` into a [`FormatArg`].
    fn into_format_arg(self) -> FormatArg<'a>;
}

impl<'a> IntoFormatArg<'a> for &'a str {
    #[inline]
    fn into_format_arg(self) -> FormatArg<'a> {
        FormatArg::Str(self)
    }
}

impl<'a, T: Display + Debug> IntoFormatArg<'a> for &'a T {
    #[inline]
    fn into_format_arg(self) -> FormatArg<'a> {
        FormatArg::Dyn(self)
    }
}
