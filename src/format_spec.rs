use std::fmt::{Debug, Display};

use crate::error::Error;

#[derive(Debug, Clone)]
pub struct FormatSpec {
    pub original: String,
    align: Option<(Option<String>, String)>,
    sign: Option<String>,
    hashtag: bool,
    zero: bool,
    width: Option<usize>,
    precision: Option<usize>,
    types: Option<String>,
}

impl FormatSpec {
    pub fn new(mut spec: String) -> Self {
        let original = spec.clone();

        let spec_copy = spec.clone();
        let align = if let Some(align) = spec_copy.get(0..1) {
            if align == "<" || align == "^" || align == ">" {
                spec.replace_range(0..1, "");
                Some((None, align.to_owned()))
            } else if let Some(align) = spec_copy.get(1..2) {
                if align == "<" || align == "^" || align == ">" {
                    spec.replace_range(0..2, "");
                    Some((
                        Some(spec_copy.get(0..1).unwrap().to_owned()),
                        align.to_owned(),
                    ))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        let spec_copy = spec.clone();
        let sign = if let Some(sign) = spec_copy.get(0..1) {
            if sign == "+" || sign == "-" {
                spec.replace_range(0..1, "");
                Some(sign.to_owned())
            } else {
                None
            }
        } else {
            None
        };

        let hashtag = if let Some(hastag) = spec.get(0..1) {
            if hastag == "#" {
                spec.replace_range(0..1, "");
                true
            } else {
                false
            }
        } else {
            false
        };

        let zero = if let Some(zero) = spec.get(0..1) {
            if zero == "0" {
                spec.replace_range(0..1, "");
                true
            } else {
                false
            }
        } else {
            false
        };

        let width = crate::utils::usize_token(&spec, 0);

        if let Some(x) = width {
            spec = spec.trim_start_matches(&x.to_string()).to_owned();
        }

        let precision = if let Some(point) = spec.get(0..1) {
            if point == "." {
                crate::utils::usize_token(&spec, 1)
            } else {
                None
            }
        } else {
            None
        };

        if let Some(x) = precision {
            spec = spec.trim_start_matches(&format!(".{}", x)).to_owned();
        }

        let types = if spec == "" {
            None
        } else {
            Some(spec.split(" ").nth(0).unwrap().to_owned())
        };

        Self {
            original,
            align,
            sign,
            hashtag,
            zero,
            width,
            precision,
            types,
        }
    }

    pub fn format<T: Display + Debug>(&self, value: T) -> String {
        let mut fmtval = format!("{}", value);

        if let Some(precision) = self.precision {
            if self.zero && self.hashtag {
                fmtval = format!("{:#0.1$}", value, precision);
            } else if self.hashtag {
                fmtval = format!("{:#.1$}", value, precision);
            } else {
                fmtval = format!("{:.1$}", value, precision);
            }
        }

        if let Some(types) = &self.types {
            if self.hashtag {
                if types == "?" {
                    return format!("{:#?}", value);
                } else if types == "x?" {
                    return format!("{:#x?}", value);
                } else if types == "X?" {
                    return format!("{:#X?}", value);
                }
            } else {
                if types == "?" {
                    return format!("{:?}", value);
                } else if types == "x?" {
                    return format!("{:x?}", value);
                } else if types == "X?" {
                    return format!("{:X?}", value);
                }
            }
        }

        if let Some(sign) = &self.sign {
            if sign == "+" && !self.zero {
                if crate::utils::is_number_and_positive(&fmtval) {
                    fmtval = "+".to_owned() + &fmtval;
                }
            }
        }

        match &self.align {
            Some((Some(fill), align)) => {
                fmtval = fmtval.trim().to_owned();
                let chars_count = fmtval.chars().count();
                let width = self.width.unwrap_or(0);

                if width > chars_count {
                    if align == "<" {
                        fmtval = fmtval + &fill.repeat(width - chars_count);
                    } else if align == "^" {
                        let factor = if chars_count % 2 == 0 { 0 } else { 1 };
                        let start = fill.repeat((width - chars_count - factor) / 2);
                        let end = fill.repeat((width - chars_count + factor) / 2);
                        fmtval = start + &fmtval + &end;
                    } else if align == ">" {
                        fmtval = fill.repeat(width - chars_count) + &fmtval;
                    }
                }
            }
            Some((None, align)) => {
                if align == "<" {
                    fmtval = format!("{:<1$}", fmtval, self.width.unwrap_or(0));
                } else if align == "^" {
                    fmtval = format!("{:^1$}", fmtval, self.width.unwrap_or(0));
                } else if align == ">" {
                    fmtval = format!("{:>1$}", fmtval, self.width.unwrap_or(0));
                }
            }
            None => (),
        }

        if let Some(width) = self.width {
            if crate::utils::is_number(&fmtval) {
                let chars_count = fmtval.chars().count();

                if self.zero && width > chars_count {
                    if let Some(sign) = &self.sign {
                        if sign == "+" {
                            if crate::utils::is_number_and_positive(&fmtval) {
                                fmtval =
                                    "+".to_owned() + &"0".repeat(width - chars_count - 1) + &fmtval;
                            } else {
                                fmtval = "-".to_owned()
                                    + &"0".repeat(width - chars_count)
                                    + &fmtval[1..];
                            }
                        } else if sign == "-" {
                            fmtval =
                                "-".to_owned() + &"0".repeat(width - chars_count) + &fmtval[1..];
                        }
                    } else {
                        if fmtval.starts_with("-") {
                            fmtval =
                                "-".to_owned() + &"0".repeat(width - chars_count) + &fmtval[1..];
                        } else {
                            fmtval = "0".repeat(width - chars_count) + &fmtval;
                        }
                    }
                } else if width > chars_count {
                    fmtval = " ".repeat(width - chars_count) + &fmtval;
                }
            } else {
                if self.zero && self.hashtag {
                    fmtval = format!("{:#01$}", fmtval, width);
                } else if self.zero {
                    fmtval = format!("{:01$}", fmtval, width);
                } else if self.hashtag {
                    fmtval = format!("{:#1$}", fmtval, width);
                } else {
                    fmtval = format!("{:1$}", fmtval, width);
                }
            }
        }

        fmtval
    }

    pub fn unsupported(&self) -> Result<(), Error> {
        if self.original.contains("$") {
            return Err(Error::new_ufs(
                "parameter setting through $ sign argument is not supported",
            ));
        }

        if self.original.contains(".*") {
            return Err(Error::new_ufs("asterisk .* formatting is not supported"));
        }

        if self.original.contains("o") {
            return Err(Error::new_ufs("o formatting is not supported"));
        }

        if self.original.contains("x") {
            return Err(Error::new_ufs("x formatting is not supported"));
        }

        if self.original.contains("X") {
            return Err(Error::new_ufs("X formatting is not supported"));
        }

        if self.original.contains("p") {
            return Err(Error::new_ufs("p formatting is not supported"));
        }

        if self.original.contains("b") {
            return Err(Error::new_ufs("b formatting is not supported"));
        }

        if self.original.contains("e") {
            return Err(Error::new_ufs("e formatting is not supported"));
        }

        if self.original.contains("E") {
            return Err(Error::new_ufs("E formatting is not supported"));
        }

        Ok(())
    }
}
