//! Typed AST for parsed format strings.

use alloc::vec::Vec;

/// A parsed format string, split into segments.
#[derive(Debug)]
pub struct FormatString<'a> {
    pub segments: Vec<Segment<'a>>,
}

/// A single piece of a format string.
#[derive(Debug)]
pub enum Segment<'a> {
    /// Literal text slice from the source.
    Literal(&'a str),
    /// Escaped open brace `{{` -> `{`.
    EscapedOpen,
    /// Escaped close brace `}}` -> `}`.
    EscapedClose,
    /// A `{...}` placeholder.
    Placeholder(Placeholder<'a>),
}

/// A single `{...}` placeholder with its argument reference and format spec.
#[derive(Debug)]
pub struct Placeholder<'a> {
    /// Which argument this placeholder refers to.
    pub argument: Argument<'a>,
    /// The format specification after the `:`.
    pub spec: FormatSpec<'a>,
    /// Byte offset of the opening `{` in the source (for error reporting).
    pub offset: usize,
}

/// How a placeholder references its argument.
#[derive(Debug)]
pub enum Argument<'a> {
    /// `{}` - uses the next implicit positional index.
    Implicit,
    /// `{0}`, `{1}` - explicit positional index.
    Positional(usize),
    /// `{name}` - named argument, resolved to a string slice.
    Named(&'a str),
}

/// The full format specification after `:` inside a placeholder.
#[derive(Debug)]
pub struct FormatSpec<'a> {
    pub fill: Option<char>,
    pub align: Option<Align>,
    pub sign: Option<Sign>,
    pub alternate: bool,
    pub zero_pad: bool,
    pub width: Option<Count<'a>>,
    pub precision: Option<Precision<'a>>,
    pub format_type: FormatType,
}

impl FormatSpec<'_> {
    /// Returns a default spec (no formatting options).
    pub const fn default() -> FormatSpec<'static> {
        FormatSpec {
            fill: None,
            align: None,
            sign: None,
            alternate: false,
            zero_pad: false,
            width: None,
            precision: None,
            format_type: FormatType::Display,
        }
    }

    /// Returns `true` if this spec has no formatting options at all —
    /// meaning we can take the fast path and `write!` directly.
    pub fn is_default(&self) -> bool {
        self.fill.is_none()
            && self.align.is_none()
            && self.sign.is_none()
            && !self.alternate
            && !self.zero_pad
            && self.width.is_none()
            && self.precision.is_none()
            && self.format_type == FormatType::Display
    }
}

/// Text alignment.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Align {
    Left,
    Center,
    Right,
}

/// Sign display mode.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Sign {
    Plus,
    Minus,
}

/// A width or precision value - either a literal number or a parameter reference.
#[derive(Debug)]
pub enum Count<'a> {
    /// A literal integer, e.g. `10` in `{:10}`.
    Literal(usize),
    /// A reference to another argument, e.g. `width$` in `{:width$}`.
    Param(CountParam<'a>),
}

/// A parameter reference for width/precision.
#[derive(Debug)]
pub enum CountParam<'a> {
    /// `{:0$}` - positional argument index.
    Positional(usize),
    /// `{:width$}` - named argument, resolved to a string slice.
    Named(&'a str),
}

/// Precision specification.
#[derive(Debug)]
pub enum Precision<'a> {
    /// `.5` or `.prec$` - a count value.
    Count(Count<'a>),
    /// `.*` - precision is the next implicit positional argument.
    Star,
}

/// The format trait to use.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FormatType {
    /// `{}` -`Display`
    Display,
    /// `{:?}` -`Debug`
    Debug,
    /// `{:x?}` -`Debug` with lowercase hex integers
    DebugLowerHex,
    /// `{:X?}` -`Debug` with uppercase hex integers
    DebugUpperHex,
    // Parsed but rejected at format time:
    /// `{:o}`
    Octal,
    /// `{:x}`
    LowerHex,
    /// `{:X}`
    UpperHex,
    /// `{:b}`
    Binary,
    /// `{:e}`
    LowerExp,
    /// `{:E}`
    UpperExp,
    /// `{:p}`
    Pointer,
}
