//! Runtime string formatting with [`std::fmt`] syntax.
//!
//! `formatx` lets you format strings at runtime using the same syntax as
//! [`std::fmt`] - `{}`, `{:?}`, `{name}`, `{:+#08.2}`, etc. - but with
//! runtime template strings instead of compile-time literals.
//!
//! # Quick Start
//!
//! ```
//! use formatx::formatx;
//!
//! let template = "{name} scored {score:.1}%";
//! let result = formatx!(template, name = "Alice", score = 95.678).unwrap();
//! assert_eq!(result, "Alice scored 95.7%");
//! ```
//!
//! # Template Reuse
//!
//! Parse once, render many times:
//!
//! ```
//! use formatx::Template;
//!
//! let template = Template::new("{name} has {n} items").unwrap();
//! let r1 = template.render().named("name", &"Alice").named("n", &3).finish().unwrap();
//! let r2 = template.render().named("name", &"Bob").named("n", &7).finish().unwrap();
//! ```

mod ast;
mod error;
mod format;
mod macros;
mod parser;
mod renderer;
mod template;
mod value;

pub use error::Error;
pub use renderer::Renderer;
pub use template::Template;
pub use value::FormatValue;

// Re-export FormatType for error matching
pub use ast::FormatType;
