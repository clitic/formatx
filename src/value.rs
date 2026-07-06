//! The [`FormatValue`] marker trait and argument storage.

use core::fmt::{Debug, Display};

/// Marker trait for values that can be formatted at runtime.
///
/// Blanket-implemented for all `T: Display + Debug` (including unsized types
/// such as `str`), which covers the vast majority of Rust types (`i32`, `f64`,
/// `String`, `&str`, `bool`, `char`, custom types with `#[derive(Debug)]` and a
/// `Display` impl, etc.).
pub trait FormatValue: Display + Debug {}

impl<T: Display + Debug + ?Sized> FormatValue for T {}

/// A format argument - either a trait object reference or a `&str` slice.
///
/// Two variants handle the `str` unsized coercion gap: `&T` where `T: Sized`
/// can coerce to `&dyn FormatValue` directly, but `&str` cannot since `str`
/// is unsized. The `Str` variant stores the `&str` directly so that callers
/// can pass string literals without extra `&` indirection.
pub enum FormatArg<'a> {
    /// A reference to any sized `FormatValue`.
    Ref(&'a dyn FormatValue),
    /// A string slice (avoids the unsized coercion issue).
    Str(&'a str),
}

impl<'a> FormatArg<'a> {
    pub(crate) fn value(&self) -> &dyn FormatValue {
        match self {
            Self::Ref(v) => *v,
            Self::Str(s) => s,
        }
    }
}

impl<'a, T: FormatValue> From<&'a T> for FormatArg<'a> {
    fn from(value: &'a T) -> Self {
        Self::Ref(value)
    }
}

impl<'a> From<&'a str> for FormatArg<'a> {
    fn from(value: &'a str) -> Self {
        Self::Str(value)
    }
}
