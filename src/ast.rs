//! Typed AST for parsed format strings.

/// Byte range in the source format string.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    /// Start byte offset (inclusive).
    pub start: usize,
    /// End byte offset (exclusive).
    pub end: usize,
}

/// A parsed format string, split into segments.
#[derive(Debug, Clone)]
pub struct FormatString {
    pub segments: Vec<Segment>,
}

/// A single piece of a format string.
#[derive(Debug, Clone)]
pub enum Segment {
    /// Literal text - byte range into the source.
    Literal(Span),
    /// Escaped open brace `{{` -> `{`.
    EscapedOpen,
    /// Escaped close brace `}}` -> `}`.
    EscapedClose,
    /// A `{...}` placeholder.
    Placeholder(Placeholder),
}

/// A single `{...}` placeholder with its argument reference and format spec.
#[derive(Debug, Clone)]
pub struct Placeholder {
    /// Which argument this placeholder refers to.
    pub argument: Argument,
    /// The format specification after the `:`.
    pub spec: FormatSpec,
    /// Byte span of the entire `{...}` in the source.
    pub span: Span,
}

/// How a placeholder references its argument.
#[derive(Debug, Clone)]
pub enum Argument {
    /// `{}` - uses the next implicit positional index.
    Implicit,
    /// `{0}`, `{1}` - explicit positional index.
    Positional(usize),
    /// `{name}` - named argument, stored as byte range.
    Named(Span),
}

/// The full format specification after `:` inside a placeholder.
#[derive(Debug, Clone)]
pub struct FormatSpec {
    pub fill: Option<char>,
    pub align: Option<Align>,
    pub sign: Option<Sign>,
    pub alternate: bool,
    pub zero_pad: bool,
    pub width: Option<Count>,
    pub precision: Option<Precision>,
    pub format_type: FormatType,
}

impl FormatSpec {
    /// Returns a default spec (no formatting options).
    pub const fn default() -> Self {
        Self {
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
    #[inline]
    pub fn is_default(&self) -> bool {
        self.fill.is_none()
            && self.align.is_none()
            && self.sign.is_none()
            && !self.alternate
            && !self.zero_pad
            && self.width.is_none()
            && self.precision.is_none()
    }
}

/// Text alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Align {
    Left,
    Center,
    Right,
}

/// Sign display mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sign {
    Plus,
    Minus,
}

/// A width or precision value - either a literal number or a parameter reference.
#[derive(Debug, Clone)]
pub enum Count {
    /// A literal integer, e.g. `10` in `{:10}`.
    Literal(usize),
    /// A reference to another argument, e.g. `width$` in `{:width$}`.
    Param(CountParam),
}

/// A parameter reference for width/precision.
#[derive(Debug, Clone)]
pub enum CountParam {
    /// `{:0$}` - positional argument index.
    Positional(usize),
    /// `{:width$}` - named argument, stored as byte range.
    Named(Span),
}

/// Precision specification.
#[derive(Debug, Clone)]
pub enum Precision {
    /// `.5` or `.prec$` - a count value.
    Count(Count),
    /// `.*` - precision is the next implicit positional argument.
    Star,
}

/// The format trait to use.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
