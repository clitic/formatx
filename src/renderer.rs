//! The [`Renderer`] builder - collects arguments and produces formatted output.

use crate::{error::Error, format, template::Template, value::FormatArg};
use alloc::{string::String, vec::Vec};

/// A builder for rendering a [`Template`] with arguments.
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
    template: &'a Template<'a>,
    args: Vec<FormatArg<'a>>,
    named: Vec<(&'a str, usize)>,
}

impl<'a> Renderer<'a> {
    /// Create a new renderer for the given template.
    pub(crate) fn new(template: &'a Template<'a>) -> Self {
        Self {
            template,
            args: Vec::new(),
            named: Vec::new(),
        }
    }

    /// Add a positional argument.
    pub fn arg<V: Into<FormatArg<'a>>>(&mut self, value: V) -> &mut Self {
        self.args.push(value.into());
        self
    }

    /// Add a named argument.
    pub fn named<V: Into<FormatArg<'a>>>(&mut self, name: &'a str, value: V) -> &mut Self {
        let index = self.args.len();
        self.args.push(value.into());
        self.named.push((name, index));
        self
    }

    /// Produce the formatted output.
    ///
    /// Returns `Err(Error::MissingArgument)` if any placeholder references an
    /// argument that was not provided.
    pub fn finish(&self) -> Result<String, Error> {
        self.render_inner(true)
    }

    /// Produce the formatted output.
    ///
    /// Missing arguments are replaced with an empty string `""` instead of
    /// producing an error.
    pub fn finish_lenient(&self) -> Result<String, Error> {
        self.render_inner(false)
    }

    fn render_inner(&self, strict: bool) -> Result<String, Error> {
        let mut output = String::new();
        format::render(
            &mut output,
            &self.template.parsed,
            &self.args,
            &self.named,
            strict,
        )?;
        Ok(output)
    }
}
