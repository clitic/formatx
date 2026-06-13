//! The [`FormatValue`] marker trait.

use std::fmt::{Debug, Display};

/// Marker trait for values that can be formatted at runtime.
///
/// Blanket-implemented for all `T: Display + Debug`, which covers the vast
/// majority of Rust types (`i32`, `f64`, `String`, `&str`, `bool`, `char`,
/// custom types with `#[derive(Debug)]` and a `Display` impl, etc.).
///
/// Since `FormatValue` is a supertrait of both `Display` and `Debug`,
/// `&dyn FormatValue` implements both traits via supertrait upcasting
/// (stable since Rust 1.76). No proxy types are needed.
pub trait FormatValue: Display + Debug {}

impl<T: Display + Debug> FormatValue for T {}
