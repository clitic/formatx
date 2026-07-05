//! The [`Template`] struct - parse once, inspect, render many times.

use crate::{
    ast::{Argument, FormatString, Segment, Span},
    error::Error,
    parser,
    renderer::Renderer,
};
use alloc::vec::Vec;
use core::fmt;

/// An owned, parsed format string that can be rendered many times with different arguments.
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
///     .named("name", &"Alice")
///     .named("score", &95.678)
///     .finish()
///     .unwrap();
/// assert_eq!(result, "Alice scored 95.7%");
/// ```
pub struct Template<'a> {
    source: &'a str,
    parsed: FormatString,
}

impl<'a> Template<'a> {
    /// Parse a format string into a reusable template.
    ///
    /// Returns `Err` if the format string is malformed (unmatched braces, invalid specs, etc.).
    pub fn new(source: &'a str) -> Result<Self, Error> {
        let parsed = parser::parse(source)?;
        Ok(Self { source, parsed })
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
                && let Argument::Named(span) = &p.argument
            {
                return self.resolve(*span) == name;
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
                    && let Argument::Named(span) = &p.argument
                {
                    return Some(self.resolve(*span));
                }
                None
            })
            .collect::<Vec<_>>()
    }

    /// Returns the original format string.
    pub fn source(&self) -> &str {
        &self.source
    }

    /// Returns a reference to the parsed AST.
    pub(crate) fn parsed(&self) -> &FormatString {
        &self.parsed
    }

    /// Resolve a [`Span`] to a string slice from the source.
    pub(crate) fn resolve(&self, span: Span) -> &str {
        &self.source[span.start..span.end]
    }
}

impl<'a> fmt::Display for Template<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.source)
    }
}

impl<'a> fmt::Debug for Template<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Template")
            .field("source", &self.source)
            .finish()
    }
}

impl<'a> From<&'a str> for Template<'a> {
    fn from(source: &'a str) -> Self {
        Self::new(source).unwrap()
    }
}
