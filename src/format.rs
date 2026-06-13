//! Format engine - applies [`FormatSpec`] to produce formatted output.
//!
//! Two paths:
//! - **Fast path**: plain `{}` or `{:?}` -`write!` directly to output.
//! - **Full path**: delegates sign/alternate/precision/width to `std::fmt` via `write!`,
//!   then handles fill+align manually when custom fill is specified.

use crate::{ast::*, error::Error, value::FormatValue};
use std::fmt::{Debug, Write};

// Display with all options including width and zero-pad
macro_rules! fmt_display_full {
    ($buf:expr, $arg:expr, $sign:expr, $alt:expr, $zero:expr, $width:expr, $prec:expr) => {
        match ($sign, $alt, $zero, $width, $prec) {
            (false, false, _, None, None) => write!($buf, "{}", $arg),
            (true, false, _, None, None) => write!($buf, "{:+}", $arg),
            (false, true, _, None, None) => write!($buf, "{:#}", $arg),
            (true, true, _, None, None) => write!($buf, "{:+#}", $arg),
            (false, false, _, None, Some(p)) => write!($buf, "{:.prec$}", $arg, prec = p),
            (true, false, _, None, Some(p)) => write!($buf, "{:+.prec$}", $arg, prec = p),
            (false, true, _, None, Some(p)) => write!($buf, "{:#.prec$}", $arg, prec = p),
            (true, true, _, None, Some(p)) => write!($buf, "{:+#.prec$}", $arg, prec = p),
            (false, false, false, Some(w), None) => write!($buf, "{:width$}", $arg, width = w),
            (true, false, false, Some(w), None) => write!($buf, "{:+width$}", $arg, width = w),
            (false, true, false, Some(w), None) => write!($buf, "{:#width$}", $arg, width = w),
            (true, true, false, Some(w), None) => write!($buf, "{:+#width$}", $arg, width = w),
            (false, false, false, Some(w), Some(p)) => {
                write!($buf, "{:width$.prec$}", $arg, width = w, prec = p)
            }
            (true, false, false, Some(w), Some(p)) => {
                write!($buf, "{:+width$.prec$}", $arg, width = w, prec = p)
            }
            (false, true, false, Some(w), Some(p)) => {
                write!($buf, "{:#width$.prec$}", $arg, width = w, prec = p)
            }
            (true, true, false, Some(w), Some(p)) => {
                write!($buf, "{:+#width$.prec$}", $arg, width = w, prec = p)
            }
            (false, false, true, Some(w), None) => write!($buf, "{:0width$}", $arg, width = w),
            (true, false, true, Some(w), None) => write!($buf, "{:+0width$}", $arg, width = w),
            (false, true, true, Some(w), None) => write!($buf, "{:#0width$}", $arg, width = w),
            (true, true, true, Some(w), None) => write!($buf, "{:+#0width$}", $arg, width = w),
            (false, false, true, Some(w), Some(p)) => {
                write!($buf, "{:0width$.prec$}", $arg, width = w, prec = p)
            }
            (true, false, true, Some(w), Some(p)) => {
                write!($buf, "{:+0width$.prec$}", $arg, width = w, prec = p)
            }
            (false, true, true, Some(w), Some(p)) => {
                write!($buf, "{:#0width$.prec$}", $arg, width = w, prec = p)
            }
            (true, true, true, Some(w), Some(p)) => {
                write!($buf, "{:+#0width$.prec$}", $arg, width = w, prec = p)
            }
        }
    };
}

macro_rules! fmt_debug_full {
    ($buf:expr, $arg:expr, $sign:expr, $alt:expr, $zero:expr, $width:expr, $prec:expr) => {
        match ($sign, $alt, $zero, $width, $prec) {
            (false, false, _, None, None) => write!($buf, "{:?}", $arg),
            (true, false, _, None, None) => write!($buf, "{:+?}", $arg),
            (false, true, _, None, None) => write!($buf, "{:#?}", $arg),
            (true, true, _, None, None) => write!($buf, "{:+#?}", $arg),
            (false, false, _, None, Some(p)) => write!($buf, "{:.prec$?}", $arg, prec = p),
            (true, false, _, None, Some(p)) => write!($buf, "{:+.prec$?}", $arg, prec = p),
            (false, true, _, None, Some(p)) => write!($buf, "{:#.prec$?}", $arg, prec = p),
            (true, true, _, None, Some(p)) => write!($buf, "{:+#.prec$?}", $arg, prec = p),
            (false, false, false, Some(w), None) => write!($buf, "{:width$?}", $arg, width = w),
            (true, false, false, Some(w), None) => write!($buf, "{:+width$?}", $arg, width = w),
            (false, true, false, Some(w), None) => write!($buf, "{:#width$?}", $arg, width = w),
            (true, true, false, Some(w), None) => write!($buf, "{:+#width$?}", $arg, width = w),
            (false, false, false, Some(w), Some(p)) => {
                write!($buf, "{:width$.prec$?}", $arg, width = w, prec = p)
            }
            (true, false, false, Some(w), Some(p)) => {
                write!($buf, "{:+width$.prec$?}", $arg, width = w, prec = p)
            }
            (false, true, false, Some(w), Some(p)) => {
                write!($buf, "{:#width$.prec$?}", $arg, width = w, prec = p)
            }
            (true, true, false, Some(w), Some(p)) => {
                write!($buf, "{:+#width$.prec$?}", $arg, width = w, prec = p)
            }
            (false, false, true, Some(w), None) => write!($buf, "{:0width$?}", $arg, width = w),
            (true, false, true, Some(w), None) => write!($buf, "{:+0width$?}", $arg, width = w),
            (false, true, true, Some(w), None) => write!($buf, "{:#0width$?}", $arg, width = w),
            (true, true, true, Some(w), None) => write!($buf, "{:+#0width$?}", $arg, width = w),
            (false, false, true, Some(w), Some(p)) => {
                write!($buf, "{:0width$.prec$?}", $arg, width = w, prec = p)
            }
            (true, false, true, Some(w), Some(p)) => {
                write!($buf, "{:+0width$.prec$?}", $arg, width = w, prec = p)
            }
            (false, true, true, Some(w), Some(p)) => {
                write!($buf, "{:#0width$.prec$?}", $arg, width = w, prec = p)
            }
            (true, true, true, Some(w), Some(p)) => {
                write!($buf, "{:+#0width$.prec$?}", $arg, width = w, prec = p)
            }
        }
    };
}

// Display core (no width - for manual padding path)
macro_rules! fmt_display_core {
    ($buf:expr, $arg:expr, $sign:expr, $alt:expr, $prec:expr) => {
        match ($sign, $alt, $prec) {
            (false, false, None) => write!($buf, "{}", $arg),
            (true, false, None) => write!($buf, "{:+}", $arg),
            (false, true, None) => write!($buf, "{:#}", $arg),
            (true, true, None) => write!($buf, "{:+#}", $arg),
            (false, false, Some(p)) => write!($buf, "{:.prec$}", $arg, prec = p),
            (true, false, Some(p)) => write!($buf, "{:+.prec$}", $arg, prec = p),
            (false, true, Some(p)) => write!($buf, "{:#.prec$}", $arg, prec = p),
            (true, true, Some(p)) => write!($buf, "{:+#.prec$}", $arg, prec = p),
        }
    };
}

macro_rules! fmt_debug_core {
    ($buf:expr, $arg:expr, $sign:expr, $alt:expr, $prec:expr) => {
        match ($sign, $alt, $prec) {
            (false, false, None) => write!($buf, "{:?}", $arg),
            (true, false, None) => write!($buf, "{:+?}", $arg),
            (false, true, None) => write!($buf, "{:#?}", $arg),
            (true, true, None) => write!($buf, "{:+#?}", $arg),
            (false, false, Some(p)) => write!($buf, "{:.prec$?}", $arg, prec = p),
            (true, false, Some(p)) => write!($buf, "{:+.prec$?}", $arg, prec = p),
            (false, true, Some(p)) => write!($buf, "{:#.prec$?}", $arg, prec = p),
            (true, true, Some(p)) => write!($buf, "{:+#.prec$?}", $arg, prec = p),
        }
    };
}

/// Render a parsed [`FormatString`] into `output` using the provided arguments.
pub fn render(
    output: &mut String,
    source: &str,
    parsed: &FormatString,
    args: &[&dyn FormatValue],
    named: &[(&str, usize)],
    strict: bool,
) -> Result<(), Error> {
    let mut implicit_pos: usize = 0;

    for segment in &parsed.segments {
        match segment {
            Segment::Literal(span) => {
                output.push_str(&source[span.start..span.end]);
            }
            Segment::EscapedOpen => output.push('{'),
            Segment::EscapedClose => output.push('}'),
            Segment::Placeholder(placeholder) => {
                // For `.*`, the precision arg is consumed BEFORE the value arg.
                let resolved_precision = resolve_precision(
                    &placeholder.spec.precision,
                    source,
                    args,
                    named,
                    &mut implicit_pos,
                )?;

                // Now resolve the argument index
                let arg_index =
                    resolve_argument(&placeholder.argument, source, &mut implicit_pos, named);

                let arg = match arg_index {
                    Some(idx) if idx < args.len() => Some(args[idx]),
                    Some(_) | None => None,
                };

                if arg.is_none() {
                    if strict {
                        let name = match &placeholder.argument {
                            Argument::Implicit => format!("{}", implicit_pos - 1),
                            Argument::Positional(idx) => format!("{idx}"),
                            Argument::Named(span) => source[span.start..span.end].to_string(),
                        };
                        return Err(Error::MissingArgument {
                            name,
                            span: placeholder.span,
                        });
                    }
                    continue;
                }

                let arg = arg.unwrap();
                let resolved_width =
                    resolve_count_value(&placeholder.spec.width, source, args, named)?;

                check_format_type(placeholder.spec.format_type, placeholder.span)?;

                // Fast path: default spec with Display
                if placeholder.spec.is_default()
                    && placeholder.spec.format_type == FormatType::Display
                {
                    write!(output, "{}", arg).map_err(Error::Format)?;
                    continue;
                }
                // Fast path: default spec with Debug
                if placeholder.spec.is_default() && is_debug_type(placeholder.spec.format_type) {
                    format_debug_fast(output, arg, placeholder.spec.format_type)?;
                    continue;
                }

                // If custom fill or align is specified -> manual padding
                if placeholder.spec.fill.is_some() || placeholder.spec.align.is_some() {
                    let mut buf = String::new();
                    format_core(&mut buf, arg, &placeholder.spec, resolved_precision)?;
                    apply_padding(output, &buf, &placeholder.spec, resolved_width);
                } else {
                    // Let std::fmt handle width + native alignment
                    format_full(
                        output,
                        arg,
                        &placeholder.spec,
                        resolved_width,
                        resolved_precision,
                    )?;
                }
            }
        }
    }

    Ok(())
}

// Argument/count resolution

fn resolve_argument(
    argument: &Argument,
    source: &str,
    implicit_pos: &mut usize,
    named: &[(&str, usize)],
) -> Option<usize> {
    match argument {
        Argument::Implicit => {
            let idx = *implicit_pos;
            *implicit_pos += 1;
            Some(idx)
        }
        Argument::Positional(idx) => Some(*idx),
        Argument::Named(span) => {
            let name = &source[span.start..span.end];
            named.iter().find(|(n, _)| *n == name).map(|(_, idx)| *idx)
        }
    }
}

fn resolve_count_value(
    count: &Option<Count>,
    source: &str,
    args: &[&dyn FormatValue],
    named: &[(&str, usize)],
) -> Result<Option<usize>, Error> {
    let Some(count) = count else { return Ok(None) };
    match count {
        Count::Literal(n) => Ok(Some(*n)),
        Count::Param(param) => {
            let idx = match param {
                CountParam::Positional(idx) => *idx,
                CountParam::Named(span) => {
                    let name = &source[span.start..span.end];
                    named
                        .iter()
                        .find(|(n, _)| *n == name)
                        .map(|(_, idx)| *idx)
                        .ok_or_else(|| Error::Parse {
                            span: *span,
                            message: format!("missing count argument: `{name}`"),
                        })?
                }
            };
            if idx >= args.len() {
                return Err(Error::Parse {
                    span: Span { start: 0, end: 0 },
                    message: format!("count argument index {idx} out of range"),
                });
            }
            let formatted = format!("{}", args[idx]);
            formatted
                .parse::<usize>()
                .map(Some)
                .map_err(|_| Error::Parse {
                    span: Span { start: 0, end: 0 },
                    message: format!("count argument `{formatted}` is not a valid usize"),
                })
        }
    }
}

fn resolve_precision(
    precision: &Option<Precision>,
    source: &str,
    args: &[&dyn FormatValue],
    named: &[(&str, usize)],
    implicit_pos: &mut usize,
) -> Result<Option<usize>, Error> {
    let Some(prec) = precision else {
        return Ok(None);
    };
    match prec {
        Precision::Count(count) => resolve_count_value(&Some(count.clone()), source, args, named),
        Precision::Star => {
            let idx = *implicit_pos;
            *implicit_pos += 1;
            if idx >= args.len() {
                return Err(Error::Parse {
                    span: Span { start: 0, end: 0 },
                    message: "not enough arguments for `.*` precision".to_string(),
                });
            }
            let formatted = format!("{}", args[idx]);
            formatted
                .parse::<usize>()
                .map(Some)
                .map_err(|_| Error::Parse {
                    span: Span { start: 0, end: 0 },
                    message: format!("`.*` precision argument `{formatted}` is not a valid usize"),
                })
        }
    }
}

fn check_format_type(format_type: FormatType, span: Span) -> Result<(), Error> {
    match format_type {
        FormatType::Display
        | FormatType::Debug
        | FormatType::DebugLowerHex
        | FormatType::DebugUpperHex => Ok(()),
        _ => Err(Error::UnsupportedTrait { format_type, span }),
    }
}

fn is_debug_type(format_type: FormatType) -> bool {
    matches!(
        format_type,
        FormatType::Debug | FormatType::DebugLowerHex | FormatType::DebugUpperHex
    )
}

fn format_debug_fast(
    output: &mut String,
    arg: &dyn FormatValue,
    format_type: FormatType,
) -> Result<(), Error> {
    let dbg: &dyn Debug = arg;
    match format_type {
        FormatType::Debug => write!(output, "{:?}", dbg)?,
        FormatType::DebugLowerHex => write!(output, "{:x?}", dbg)?,
        FormatType::DebugUpperHex => write!(output, "{:X?}", dbg)?,
        _ => unreachable!(),
    }
    Ok(())
}

/// let std::fmt handle width natively
fn format_full(
    output: &mut String,
    arg: &dyn FormatValue,
    spec: &FormatSpec,
    width: Option<usize>,
    precision: Option<usize>,
) -> Result<(), Error> {
    let sign_plus = matches!(spec.sign, Some(Sign::Plus));
    let alternate = spec.alternate;

    match spec.format_type {
        FormatType::Display => {
            fmt_display_full!(
                output,
                arg,
                sign_plus,
                alternate,
                spec.zero_pad,
                width,
                precision
            )?;
        }
        FormatType::Debug => {
            let dbg: &dyn Debug = arg;
            fmt_debug_full!(
                output,
                dbg,
                sign_plus,
                alternate,
                spec.zero_pad,
                width,
                precision
            )?;
        }
        FormatType::DebugLowerHex => {
            let dbg: &dyn Debug = arg;
            write!(output, "{:x?}", dbg)?; // simplified - width with x?/X? is rare
        }
        FormatType::DebugUpperHex => {
            let dbg: &dyn Debug = arg;
            write!(output, "{:X?}", dbg)?;
        }
        _ => unreachable!(),
    }

    Ok(())
}

// no width (for manual padding path)
fn format_core(
    buf: &mut String,
    arg: &dyn FormatValue,
    spec: &FormatSpec,
    precision: Option<usize>,
) -> Result<(), Error> {
    let sign_plus = matches!(spec.sign, Some(Sign::Plus));
    let alternate = spec.alternate;

    match spec.format_type {
        FormatType::Display => {
            fmt_display_core!(buf, arg, sign_plus, alternate, precision)?;
        }
        FormatType::Debug => {
            let dbg: &dyn Debug = arg;
            fmt_debug_core!(buf, dbg, sign_plus, alternate, precision)?;
        }
        FormatType::DebugLowerHex => {
            let dbg: &dyn Debug = arg;
            write!(buf, "{:x?}", dbg)?;
        }
        FormatType::DebugUpperHex => {
            let dbg: &dyn Debug = arg;
            write!(buf, "{:X?}", dbg)?;
        }
        _ => unreachable!(),
    }

    Ok(())
}

/// Manual padding
fn apply_padding(output: &mut String, raw: &str, spec: &FormatSpec, width: Option<usize>) {
    let Some(width) = width else {
        output.push_str(raw);
        return;
    };

    let char_count = raw.chars().count();
    if char_count >= width {
        output.push_str(raw);
        return;
    }

    let pad_total = width - char_count;
    let fill = spec.fill.unwrap_or(' ');
    let align = spec.align.unwrap_or(Align::Left);

    match align {
        Align::Left => {
            output.push_str(raw);
            for _ in 0..pad_total {
                output.push(fill);
            }
        }
        Align::Right => {
            for _ in 0..pad_total {
                output.push(fill);
            }
            output.push_str(raw);
        }
        Align::Center => {
            let left_pad = pad_total / 2;
            let right_pad = pad_total - left_pad;
            for _ in 0..left_pad {
                output.push(fill);
            }
            output.push_str(raw);
            for _ in 0..right_pad {
                output.push(fill);
            }
        }
    }
}
