//! The [`Template`] struct - parse once, inspect, render many times.

use crate::{
    ast::{Argument, FormatString, Segment},
    error::Error,
    parser,
    renderer::Renderer,
};
use alloc::vec::Vec;

/// A parsed format string that can be rendered many times with different arguments.
///
/// # Examples
///
/// ```
/// use formatx::Template;
///
/// let template = Template::new("{name} scored {score:.1}%").unwrap();
/// assert!(template.contains("name"));
///
/// let result = template.render()
///     .named("name", "Alice")
///     .named("score", &95.678)
///     .finish()
///     .unwrap();
/// assert_eq!(result, "Alice scored 95.7%");
/// ```
#[derive(Debug)]
pub struct Template<'a> {
    pub(crate) parsed: FormatString<'a>,
}

impl<'a> Template<'a> {
    /// Parse a format string into a reusable template.
    ///
    /// Returns `Err` if the format string is malformed (unmatched braces, invalid specs, etc.).
    pub fn new<S: AsRef<str> + ?Sized>(source: &'a S) -> Result<Self, Error> {
        let parsed = parser::parse(source.as_ref())?;
        Ok(Self { parsed })
    }

    /// Create a [`Renderer`] to format this template with arguments.
    ///
    /// The renderer collects arguments and produces the formatted output.
    pub fn render(&self) -> Renderer<'_> {
        Renderer::new(self)
    }

    /// Returns `true` if the template contains a placeholder with the given name.
    pub fn contains(&self, name: &str) -> bool {
        self.parsed.segments.iter().any(|seg| {
            if let Segment::Placeholder(p) = seg
                && let Argument::Named(n) = &p.argument
            {
                return *n == name;
            }
            false
        })
    }

    /// Returns the names of all named placeholders in the template.
    pub fn placeholders(&self) -> Vec<&str> {
        self.parsed
            .segments
            .iter()
            .filter_map(|seg| {
                if let Segment::Placeholder(p) = seg
                    && let Argument::Named(n) = &p.argument
                {
                    return Some(*n);
                }
                None
            })
            .collect::<Vec<_>>()
    }
}

impl<'a> TryFrom<&'a str> for Template<'a> {
    type Error = Error;

    fn try_from(source: &'a str) -> Result<Self, Self::Error> {
        Self::new(source)
    }
}
