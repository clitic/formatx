//! Single-pass parser for `std::fmt`-style format strings.
//!
//! Produces a [`FormatString`] AST from a `&str` source.

use crate::{ast::*, error::Error};

/// Parse a format string into a [`FormatString`] AST.
///
/// This is a single-pass parser using `char_indices()`.
/// It handles all `std::fmt` syntax: positional, named, and implicit arguments,
/// fill/align, sign, `#`, `0`, width, precision (including `$`-params and `.*`),
/// and format types.
pub fn parse(source: &str) -> Result<FormatString, Error> {
    let mut segments = Vec::new();
    let bytes = source.as_bytes();
    let len = bytes.len();
    let mut pos = 0;
    let mut implicit_counter: usize = 0;

    while pos < len {
        match bytes[pos] {
            b'{' => {
                if pos + 1 < len && bytes[pos + 1] == b'{' {
                    // Escaped `{{`
                    segments.push(Segment::EscapedOpen);
                    pos += 2;
                } else {
                    // Start of placeholder `{...}`
                    let start = pos;
                    pos += 1; // skip `{`
                    let (placeholder, end) =
                        parse_placeholder(source, pos, start, &mut implicit_counter)?;
                    segments.push(Segment::Placeholder(placeholder));
                    pos = end;
                }
            }
            b'}' => {
                if pos + 1 < len && bytes[pos + 1] == b'}' {
                    // Escaped `}}`
                    segments.push(Segment::EscapedClose);
                    pos += 2;
                } else {
                    return Err(Error::Parse {
                        span: Span { start: pos, end: pos + 1 },
                        message: "unmatched `}`".to_string(),
                    });
                }
            }
            _ => {
                // Literal text -collect until we hit `{` or `}`
                let start = pos;
                while pos < len && bytes[pos] != b'{' && bytes[pos] != b'}' {
                    pos += 1;
                }
                segments.push(Segment::Literal(Span { start, end: pos }));
            }
        }
    }

    Ok(FormatString { segments })
}

/// Parse the inside of a `{...}` placeholder, starting right after the `{`.
/// Returns `(Placeholder, end_pos)` where `end_pos` is right after the closing `}`.
fn parse_placeholder(
    source: &str,
    mut pos: usize,
    brace_start: usize,
    implicit_counter: &mut usize,
) -> Result<(Placeholder, usize), Error> {
    let bytes = source.as_bytes();
    let len = bytes.len();

    // Parse argument (before `:` or `}`)
    let argument = parse_argument(source, &mut pos, implicit_counter)?;

    // Parse format spec (after `:`)
    let spec = if pos < len && bytes[pos] == b':' {
        pos += 1; // skip `:`
        parse_format_spec(source, &mut pos, implicit_counter)?
    } else {
        FormatSpec::default()
    };

    // Expect closing `}`
    if pos >= len || bytes[pos] != b'}' {
        return Err(Error::Parse {
            span: Span { start: brace_start, end: pos.min(len) },
            message: "unmatched `{`".to_string(),
        });
    }
    pos += 1; // skip `}`

    let placeholder = Placeholder {
        argument,
        spec,
        span: Span { start: brace_start, end: pos },
    };

    Ok((placeholder, pos))
}

/// Parse the argument part of a placeholder (before `:` or `}`).
fn parse_argument(
    source: &str,
    pos: &mut usize,
    implicit_counter: &mut usize,
) -> Result<Argument, Error> {
    let bytes = source.as_bytes();
    let len = bytes.len();

    if *pos >= len {
        return Err(Error::Parse {
            span: Span { start: *pos, end: *pos },
            message: "unexpected end of format string".to_string(),
        });
    }

    // `}` or `:` immediately → implicit
    if bytes[*pos] == b'}' || bytes[*pos] == b':' {
        *implicit_counter += 1;
        return Ok(Argument::Implicit);
    }

    // Try to parse a number (positional index)
    let start = *pos;
    if bytes[*pos].is_ascii_digit() {
        while *pos < len && bytes[*pos].is_ascii_digit() {
            *pos += 1;
        }
        // Must be followed by `}` or `:` -not `$` (that's a count param, not here)
        if *pos < len && (bytes[*pos] == b'}' || bytes[*pos] == b':') {
            let num_str = &source[start..*pos];
            let index = num_str.parse::<usize>().map_err(|_| Error::Parse {
                span: Span { start, end: *pos },
                message: format!("invalid positional argument: `{num_str}`"),
            })?;
            return Ok(Argument::Positional(index));
        }
        // Not a valid positional -reset and try as identifier
        *pos = start;
    }

    // Try to parse an identifier (named argument)
    if bytes[*pos].is_ascii_alphabetic() || bytes[*pos] == b'_' {
        let name_start = *pos;
        while *pos < len && (bytes[*pos].is_ascii_alphanumeric() || bytes[*pos] == b'_') {
            *pos += 1;
        }
        if *pos < len && (bytes[*pos] == b'}' || bytes[*pos] == b':') {
            return Ok(Argument::Named(Span { start: name_start, end: *pos }));
        }
        // Reset if not valid
        *pos = start;
    }

    Err(Error::Parse {
        span: Span { start, end: *pos + 1 },
        message: "invalid placeholder argument".to_string(),
    })
}

/// Parse the format spec after `:` -fill, align, sign, `#`, `0`, width, `.precision`, type.
fn parse_format_spec(
    source: &str,
    pos: &mut usize,
    implicit_counter: &mut usize,
) -> Result<FormatSpec, Error> {
    let bytes = source.as_bytes();
    let len = bytes.len();
    let mut spec = FormatSpec::default();

    if *pos >= len || bytes[*pos] == b'}' {
        return Ok(spec);
    }

    // Fill and align -2-char lookahead:
    // If char at pos+1 is an align char, then char at pos is fill.
    // Otherwise, if char at pos is an align char, use default fill.
    let (fill, align) = parse_fill_align(source, pos);
    spec.fill = fill;
    spec.align = align;

    if *pos >= len || bytes[*pos] == b'}' {
        return Ok(spec);
    }

    // Sign
    if bytes[*pos] == b'+' {
        spec.sign = Some(Sign::Plus);
        *pos += 1;
    } else if bytes[*pos] == b'-' {
        spec.sign = Some(Sign::Minus);
        *pos += 1;
    }

    if *pos >= len || bytes[*pos] == b'}' {
        return Ok(spec);
    }

    // Alternate `#`
    if bytes[*pos] == b'#' {
        spec.alternate = true;
        *pos += 1;
    }

    if *pos >= len || bytes[*pos] == b'}' {
        return Ok(spec);
    }

    // Zero-pad `0` -only if followed by a digit or `}` or `.` or type char
    // (a bare `0` before width is zero-padding, not width)
    if bytes[*pos] == b'0' {
        // Peek ahead: if next char is a digit, this is zero-pad prefix
        // If next char is `}` or `.` or a type char, this could be width=0 or zero-pad
        // In std::fmt, `{:0}` is zero-pad with no width, `{:05}` is zero-pad with width 5
        let next = if *pos + 1 < len { bytes[*pos + 1] } else { b'}' };
        if next.is_ascii_digit() || next == b'}' || next == b'.' || is_type_char(next) {
            spec.zero_pad = true;
            *pos += 1;
        }
    }

    if *pos >= len || bytes[*pos] == b'}' {
        return Ok(spec);
    }

    // Width
    spec.width = parse_count(source, pos)?;

    if *pos >= len || bytes[*pos] == b'}' {
        return Ok(spec);
    }

    // Precision (`.`)
    if bytes[*pos] == b'.' {
        *pos += 1; // skip `.`

        if *pos >= len {
            return Err(Error::Parse {
                span: Span { start: *pos - 1, end: *pos },
                message: "expected precision after `.`".to_string(),
            });
        }

        // `.*` -star precision
        if bytes[*pos] == b'*' {
            *pos += 1;
            spec.precision = Some(Precision::Star);
            // Star consumes the next implicit positional arg
            *implicit_counter += 1;
        } else {
            // Count-based precision
            if let Some(count) = parse_count(source, pos)? {
                spec.precision = Some(Precision::Count(count));
            } else {
                return Err(Error::Parse {
                    span: Span { start: *pos - 1, end: *pos },
                    message: "expected precision value after `.`".to_string(),
                });
            }
        }
    }

    if *pos >= len || bytes[*pos] == b'}' {
        return Ok(spec);
    }

    // Format type
    spec.format_type = parse_format_type(source, pos)?;

    Ok(spec)
}

/// Parse fill character and alignment.
fn parse_fill_align(source: &str, pos: &mut usize) -> (Option<char>, Option<Align>) {
    let bytes = source.as_bytes();
    let len = bytes.len();

    if *pos >= len || bytes[*pos] == b'}' {
        return (None, None);
    }

    // 2-char lookahead: fill + align
    // The fill can be any character (including multi-byte UTF-8).
    let rest = &source[*pos..];
    let mut chars = rest.chars();

    if let Some(first) = chars.next() {
        if let Some(second) = chars.next()
            && let Some(align) = to_align(second)
        {
            // first is fill, second is align
            *pos += first.len_utf8() + second.len_utf8();
            return (Some(first), Some(align));
        }
        // 1-char: just align, no fill
        if let Some(align) = to_align(first) {
            *pos += first.len_utf8();
            return (None, Some(align));
        }
    }

    (None, None)
}

/// Convert a character to an [`Align`] variant.
fn to_align(c: char) -> Option<Align> {
    match c {
        '<' => Some(Align::Left),
        '^' => Some(Align::Center),
        '>' => Some(Align::Right),
        _ => None,
    }
}

/// Parse a count value: literal integer or `name$` / `index$` parameter.
fn parse_count(source: &str, pos: &mut usize) -> Result<Option<Count>, Error> {
    let bytes = source.as_bytes();
    let len = bytes.len();

    if *pos >= len || bytes[*pos] == b'}' {
        return Ok(None);
    }

    let start = *pos;

    // Try a number
    if bytes[*pos].is_ascii_digit() {
        while *pos < len && bytes[*pos].is_ascii_digit() {
            *pos += 1;
        }
        // Check for `$` suffix → parameter reference
        if *pos < len && bytes[*pos] == b'$' {
            let num_str = &source[start..*pos];
            let index = num_str.parse::<usize>().map_err(|_| Error::Parse {
                span: Span { start, end: *pos },
                message: format!("invalid count parameter: `{num_str}`"),
            })?;
            *pos += 1; // skip `$`
            return Ok(Some(Count::Param(CountParam::Positional(index))));
        }
        // Plain literal number
        let num_str = &source[start..*pos];
        let value = num_str.parse::<usize>().map_err(|_| Error::Parse {
            span: Span { start, end: *pos },
            message: format!("invalid count: `{num_str}`"),
        })?;
        return Ok(Some(Count::Literal(value)));
    }

    // Try an identifier followed by `$`
    if bytes[*pos].is_ascii_alphabetic() || bytes[*pos] == b'_' {
        let name_start = *pos;
        while *pos < len && (bytes[*pos].is_ascii_alphanumeric() || bytes[*pos] == b'_') {
            *pos += 1;
        }
        if *pos < len && bytes[*pos] == b'$' {
            *pos += 1; // skip `$`
            return Ok(Some(Count::Param(CountParam::Named(Span {
                start: name_start,
                end: *pos - 1, // exclude `$`
            }))));
        }
        // Not a count param -reset
        *pos = start;
    }

    Ok(None)
}

/// Parse the format type suffix (e.g. `?`, `x?`, `x`, `o`, `b`, etc.).
fn parse_format_type(source: &str, pos: &mut usize) -> Result<FormatType, Error> {
    let bytes = source.as_bytes();
    let len = bytes.len();

    if *pos >= len || bytes[*pos] == b'}' {
        return Ok(FormatType::Display);
    }

    let start = *pos;
    let ty = match bytes[*pos] {
        b'?' => { *pos += 1; FormatType::Debug }
        b'o' => { *pos += 1; FormatType::Octal }
        b'b' => { *pos += 1; FormatType::Binary }
        b'e' => { *pos += 1; FormatType::LowerExp }
        b'E' => { *pos += 1; FormatType::UpperExp }
        b'p' => { *pos += 1; FormatType::Pointer }
        b'x' => {
            *pos += 1;
            if *pos < len && bytes[*pos] == b'?' {
                *pos += 1;
                FormatType::DebugLowerHex
            } else {
                FormatType::LowerHex
            }
        }
        b'X' => {
            *pos += 1;
            if *pos < len && bytes[*pos] == b'?' {
                *pos += 1;
                FormatType::DebugUpperHex
            } else {
                FormatType::UpperHex
            }
        }
        c => {
            return Err(Error::Parse {
                span: Span { start, end: *pos + 1 },
                message: format!("unknown format type: `{}`", c as char),
            });
        }
    };

    Ok(ty)
}

/// Check if a byte is a format type character.
fn is_type_char(b: u8) -> bool {
    matches!(b, b'?' | b'o' | b'x' | b'X' | b'b' | b'e' | b'E' | b'p')
}

#[cfg(test)]
mod tests {
    use super::*;

    fn resolve(source: &str, span: Span) -> &str {
        &source[span.start..span.end]
    }

    #[test]
    fn empty_string() {
        let result = parse("").unwrap();
        assert!(result.segments.is_empty());
    }

    #[test]
    fn literal_only() {
        let source = "hello world";
        let result = parse(source).unwrap();
        assert_eq!(result.segments.len(), 1);
        if let Segment::Literal(span) = &result.segments[0] {
            assert_eq!(resolve(source, *span), "hello world");
        } else {
            panic!("expected Literal");
        }
    }

    #[test]
    fn escaped_braces() {
        let result = parse("{{}}").unwrap();
        assert_eq!(result.segments.len(), 2);
        assert!(matches!(result.segments[0], Segment::EscapedOpen));
        assert!(matches!(result.segments[1], Segment::EscapedClose));
    }

    #[test]
    fn implicit_args() {
        let result = parse("{} {}").unwrap();
        assert_eq!(result.segments.len(), 3);
        if let Segment::Placeholder(p) = &result.segments[0] {
            assert!(matches!(p.argument, Argument::Implicit));
        }
    }

    #[test]
    fn positional_args() {
        let result = parse("{0} {1}").unwrap();
        if let Segment::Placeholder(p) = &result.segments[0] {
            assert!(matches!(p.argument, Argument::Positional(0)));
        }
        if let Segment::Placeholder(p) = &result.segments[2] {
            assert!(matches!(p.argument, Argument::Positional(1)));
        }
    }

    #[test]
    fn named_args() {
        let source = "{name}";
        let result = parse(source).unwrap();
        if let Segment::Placeholder(p) = &result.segments[0] {
            if let Argument::Named(span) = &p.argument {
                assert_eq!(resolve(source, *span), "name");
            } else {
                panic!("expected Named");
            }
        }
    }

    #[test]
    fn format_spec_width_precision() {
        let result = parse("{:10.5}").unwrap();
        if let Segment::Placeholder(p) = &result.segments[0] {
            assert!(matches!(p.spec.width, Some(Count::Literal(10))));
            assert!(matches!(p.spec.precision, Some(Precision::Count(Count::Literal(5)))));
        }
    }

    #[test]
    fn format_spec_fill_align() {
        let result = parse("{:-<10}").unwrap();
        if let Segment::Placeholder(p) = &result.segments[0] {
            assert_eq!(p.spec.fill, Some('-'));
            assert_eq!(p.spec.align, Some(Align::Left));
            assert!(matches!(p.spec.width, Some(Count::Literal(10))));
        }
    }

    #[test]
    fn format_spec_sign_alternate_zero() {
        let result = parse("{:+#05}").unwrap();
        if let Segment::Placeholder(p) = &result.segments[0] {
            assert_eq!(p.spec.sign, Some(Sign::Plus));
            assert!(p.spec.alternate);
            assert!(p.spec.zero_pad);
            assert!(matches!(p.spec.width, Some(Count::Literal(5))));
        }
    }

    #[test]
    fn format_type_debug() {
        let result = parse("{:?}").unwrap();
        if let Segment::Placeholder(p) = &result.segments[0] {
            assert_eq!(p.spec.format_type, FormatType::Debug);
        }
    }

    #[test]
    fn format_type_pretty_debug() {
        let result = parse("{:#?}").unwrap();
        if let Segment::Placeholder(p) = &result.segments[0] {
            assert!(p.spec.alternate);
            assert_eq!(p.spec.format_type, FormatType::Debug);
        }
    }

    #[test]
    fn star_precision() {
        let result = parse("{:.*}").unwrap();
        if let Segment::Placeholder(p) = &result.segments[0] {
            assert!(matches!(p.spec.precision, Some(Precision::Star)));
        }
    }

    #[test]
    fn param_width_and_precision() {
        let source = "{:width$.prec$}";
        let result = parse(source).unwrap();
        if let Segment::Placeholder(p) = &result.segments[0] {
            if let Some(Count::Param(CountParam::Named(span))) = &p.spec.width {
                assert_eq!(resolve(source, *span), "width");
            } else {
                panic!("expected named width param");
            }
            if let Some(Precision::Count(Count::Param(CountParam::Named(span)))) = &p.spec.precision {
                assert_eq!(resolve(source, *span), "prec");
            } else {
                panic!("expected named precision param");
            }
        }
    }

    #[test]
    fn unmatched_open_brace() {
        assert!(parse("{").is_err());
    }

    #[test]
    fn unmatched_close_brace() {
        assert!(parse("}").is_err());
    }

    #[test]
    fn debug_hex_types() {
        let result = parse("{:x?}").unwrap();
        if let Segment::Placeholder(p) = &result.segments[0] {
            assert_eq!(p.spec.format_type, FormatType::DebugLowerHex);
        }
        let result = parse("{:X?}").unwrap();
        if let Segment::Placeholder(p) = &result.segments[0] {
            assert_eq!(p.spec.format_type, FormatType::DebugUpperHex);
        }
    }

    #[test]
    fn complex_mixed() {
        let source = "Hello {name}, your score is {0:+08.2}! {{escaped}}";
        let result = parse(source).unwrap();
        // Should have: Literal, Placeholder(name), Literal, Placeholder(0:+08.2),
        //              Literal, EscapedOpen, Literal("escaped"), EscapedClose
        assert!(result.segments.len() >= 5);
    }
}
