//! The [`Renderer`] builder -collects arguments and produces formatted output.

use std::fmt::{Debug, Display};

use crate::error::Error;
use crate::format;
use crate::template::Template;
use crate::value::FormatValue;

/// A builder for rendering a [`Template`] with arguments.
///
/// Created by [`Template::render()`]. Collects positional and named arguments,
/// then produces the formatted output via [`finish()`](Renderer::finish) (strict)
/// or [`finish_lenient()`](Renderer::finish_lenient) (lenient).
///
/// # Examples
///
/// ```
/// use formatx::Template;
///
/// let template = Template::new("{} + {} = {}").unwrap();
/// let result = template.render()
///     .arg(&1)
///     .arg(&2)
///     .arg(&3)
///     .finish()
///     .unwrap();
/// assert_eq!(result, "1 + 2 = 3");
/// ```
pub struct Renderer<'a> {
    template: &'a Template,
    args: Vec<&'a dyn FormatValue>,
    named: Vec<(&'a str, usize)>,
}

impl<'a> Renderer<'a> {
    /// Create a new renderer for the given template.
    pub(crate) fn new(template: &'a Template) -> Self {
        Self {
            template,
            args: Vec::new(),
            named: Vec::new(),
        }
    }

    /// Add a positional argument.
    #[inline]
    pub fn arg(&mut self, value: &'a (impl Display + Debug)) -> &mut Self {
        self.args.push(value as &dyn FormatValue);
        self
    }

    /// Add a named argument.
    #[inline]
    pub fn named(&mut self, name: &'a str, value: &'a (impl Display + Debug)) -> &mut Self {
        let index = self.args.len();
        self.args.push(value as &dyn FormatValue);
        self.named.push((name, index));
        self
    }

    /// **Strict**: produce the formatted output.
    ///
    /// Returns `Err(Error::MissingArgument)` if any placeholder references an
    /// argument that was not provided.
    pub fn finish(&self) -> Result<String, Error> {
        self.render_inner(true)
    }

    /// **Lenient**: produce the formatted output.
    ///
    /// Missing arguments are replaced with an empty string `""` instead of
    /// producing an error.
    pub fn finish_lenient(&self) -> Result<String, Error> {
        self.render_inner(false)
    }

    fn render_inner(&self, strict: bool) -> Result<String, Error> {
        let source = self.template.source();
        let mut output = String::with_capacity(source.len());
        format::render(
            &mut output,
            source,
            self.template.parsed(),
            &self.args,
            &self.named,
            strict,
        )?;
        Ok(output)
    }
}
