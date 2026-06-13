//! Error types for formatx.

use crate::ast::{FormatType, Span};
use std::fmt;

/// Errors that can occur during parsing or formatting.
#[derive(Debug)]
pub enum Error {
    /// The format string could not be parsed.
    Parse {
        span: Span,
        message: String,
    },
    /// A placeholder references an argument that was not provided.
    MissingArgument {
        name: String,
        span: Span,
    },
    /// A format type (e.g. `{:x}`) requires a trait we don't support.
    UnsupportedTrait {
        format_type: FormatType,
        span: Span,
    },
    /// An underlying `std::fmt::Error` occurred during formatting.
    Format(fmt::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse { message, span, .. } => {
                write!(f, "parse error at byte {}: {}", span.start, message)
            }
            Self::MissingArgument { name, .. } => {
                write!(f, "missing argument: `{name}`")
            }
            Self::UnsupportedTrait { format_type, .. } => {
                let trait_name = match format_type {
                    FormatType::Octal => "Octal",
                    FormatType::LowerHex => "LowerHex",
                    FormatType::UpperHex => "UpperHex",
                    FormatType::Binary => "Binary",
                    FormatType::LowerExp => "LowerExp",
                    FormatType::UpperExp => "UpperExp",
                    FormatType::Pointer => "Pointer",
                    _ => "Unknown",
                };
                write!(f, "unsupported format trait: `{trait_name}`")
            }
            Self::Format(e) => write!(f, "formatting error: {e}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        if let Self::Format(e) = self {
            Some(e)
        } else {
            None
        }
    }
}

impl From<fmt::Error> for Error {
    fn from(e: fmt::Error) -> Self {
        Self::Format(e)
    }
}
