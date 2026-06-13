//! The [`FormatValue`] marker trait.

use std::fmt::{Debug, Display};

/// Marker trait for values that can be formatted at runtime.
///
/// Blanket-implemented for all `T: Display + Debug`, which covers the vast
/// majority of Rust types (`i32`, `f64`, `String`, `&str`, `bool`, `char`,
/// custom types with `#[derive(Debug)]` and a `Display` impl, etc.).
pub trait FormatValue: Display + Debug {}

impl<T: Display + Debug> FormatValue for T {}
