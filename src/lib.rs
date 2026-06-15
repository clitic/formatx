//! `formatx` lets you format strings at runtime using the same syntax as
//! [`std::fmt`] (`{}`, `{:?}`, `{name}`, etc.), but with runtime template
//! strings instead of compile-time literals with **zero** dependencies.
//!
//! # Using [`formatx!`]
//!
//! Works just like [`format!`], but accepts runtime template strings.
//!
//! ```
//! use formatx::formatx;
//!
//! let template = "{} scored {score:.1}% in {}";
//! let result = formatx!(template, "Alice", "maths", score = 95.678).unwrap();
//! assert_eq!(result, "Alice scored 95.7% in maths");
//! ```
//!
//! **Note:** Extra arguments that aren't referenced by any placeholder are
//! silently ignored in both [`formatx!`] and [`formatxl!`].
//!
//! # Template Reuse
//!
//! Parse once, render many times with [`Template`].
//!
//! ```
//! use formatx::Template;
//!
//! let template = Template::new("{name} has {n} items").unwrap();
//!
//! let r1 = template.render()
//!     .named("name", &"Alice")
//!     .named("n", &3)
//!     .finish()
//!     .unwrap();
//!
//! let r2 = template.render()
//!     .named("name", &"Bob")
//!     .named("n", &7)
//!     .finish()
//!     .unwrap();
//!
//! assert_eq!(r1, "Alice has 3 items");
//! assert_eq!(r2, "Bob has 7 items");
//! ```

mod ast;
mod error;
mod format;
mod macros;
mod parser;
mod renderer;
mod template;
mod value;

pub use ast::FormatType;
pub use error::Error;
pub use renderer::Renderer;
pub use template::Template;
pub use value::FormatValue;
