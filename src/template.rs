//! The [`Template`] struct -parse once, inspect, render many times.

use std::fmt;
use std::str::FromStr;

use crate::ast::{Argument, FormatString, Segment, Span};
use crate::error::Error;
use crate::parser;
use crate::renderer::Renderer;

/// An owned, parsed format string that can be rendered many times with different arguments.
///
/// `Template` owns the source string and stores the parsed AST with `Span` byte offsets.
/// No lifetime parameters -store it in structs, return it from functions, put it in
/// collections freely.
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
pub struct Template {
    source: String,
    parsed: FormatString,
}

impl Template {
    /// Parse a format string into a reusable template.
    ///
    /// Returns `Err` if the format string is malformed (unmatched braces, invalid specs, etc.).
    pub fn new<S: Into<String>>(source: S) -> Result<Self, Error> {
        let source = source.into();
        let parsed = parser::parse(&source)?;
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

impl fmt::Display for Template {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.source)
    }
}

impl fmt::Debug for Template {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Template")
            .field("source", &self.source)
            .finish()
    }
}

impl FromStr for Template {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}
