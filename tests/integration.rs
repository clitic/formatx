use formatx::{Error, FormatType, Template, formatx, formatxl};

macro_rules! assert_fmt {
    ($spec:literal $(, $arg:expr)* $(,)?) => {
        assert_eq!(
            formatx!($spec $(, $arg)*).unwrap(),
            format!($spec $(, $arg)*),
            "mismatch for spec {:?}",
            $spec,
        );
    };
}

#[test]
fn display_basic() {
    assert_fmt!("{}", 42);
    assert_fmt!("{}", "hello");
    assert_fmt!("{}", 3.14);
    assert_fmt!("{}", true);
    assert_fmt!("{}", 'X');
}

#[test]
fn display_sign() {
    assert_fmt!("{:+}", 42);
    assert_fmt!("{:+}", -42);
}

#[test]
fn display_alternate() {
    assert_fmt!("{:#}", 42);
}

#[test]
fn display_precision() {
    assert_fmt!("{:.5}", 3.14);
    assert_fmt!("{:.0}", 3.14);
    assert_fmt!("{:.2}", "hello");
}

#[test]
fn display_width() {
    assert_fmt!("{:10}", 42);
    assert_fmt!("{:10}", "hi");
}

#[test]
fn display_fill_align() {
    assert_fmt!("{:-<10}", "hi");
    assert_fmt!("{:->10}", "hi");
    assert_fmt!("{:-^10}", "hi");
    assert_fmt!("{:*>10}", 42);
}

#[test]
fn display_zero_pad() {
    assert_fmt!("{:05}", 42);
    assert_fmt!("{:+08}", 42);
    assert_fmt!("{:+08}", -42);
}

#[test]
fn display_combined() {
    assert_fmt!("{:+#.5}", 3.14);
}

#[test]
fn debug_basic() {
    assert_fmt!("{:?}", 42);
    assert_fmt!("{:?}", "hello");
    assert_fmt!("{:?}", true);
}

#[test]
fn debug_pretty() {
    assert_fmt!("{:#?}", "hello");
    assert_fmt!("{:#?}", 42);
}

#[test]
fn debug_hex() {
    assert_fmt!("{:x?}", 255);
    assert_fmt!("{:X?}", 255);
}

#[test]
fn positional_args() {
    assert_fmt!("{0} {1}", "a", "b");
    assert_fmt!("{0} {0}", "repeat");
}

#[test]
fn mixed_positional_implicit() {
    assert_fmt!("{1} {} {0} {}", 1, 2);
}

#[test]
fn named_args() {
    assert_eq!(
        formatx!("{name} is {age}", name = "Alice", age = 30).unwrap(),
        "Alice is 30",
    );
}

#[test]
fn escaped_braces() {
    assert_eq!(formatx!("{{}}").unwrap(), "{}");
    assert_eq!(formatx!("{{ }}").unwrap(), "{ }");
    assert_fmt!("{{{}}}", 42);
}

#[test]
fn no_placeholders() {
    assert_eq!(formatx!("hello world").unwrap(), "hello world");
}

#[test]
fn empty_string() {
    assert_eq!(formatx!("").unwrap(), "");
}

#[test]
fn width_from_positional_param() {
    assert_fmt!("{1:0$}", 10, "hi");
}

#[test]
fn precision_from_named_param() {
    assert_eq!(
        formatx!("{:.prec$}", 3.14159, prec = 2).unwrap(),
        format!("{:.prec$}", 3.14159, prec = 2),
    );
}

#[test]
fn star_precision() {
    assert_fmt!("{:.*}", 3, 3.14159);
}

#[test]
fn strict_missing_named() {
    let err = formatx!("{name} {missing}", name = "Alice").unwrap_err();
    assert!(matches!(err, Error::MissingArgument { ref name, .. } if name == "missing"));
}

#[test]
fn strict_missing_positional() {
    assert!(formatx!("{0} {1}", "only_one").is_err());
}

#[test]
fn lenient_missing_named() {
    assert_eq!(
        formatxl!("{greeting} {name}!", greeting = "Hello").unwrap(),
        "Hello !"
    );
}

#[test]
fn lenient_missing_positional() {
    assert_eq!(formatxl!("{} {} {}", "a", "b").unwrap(), "a b ");
}

#[test]
fn lenient_all_present() {
    assert_eq!(formatxl!("{} {}", "a", "b").unwrap(), "a b");
}

#[test]
fn unsupported_hex_format() {
    let err = formatx!("{:x}", 42).unwrap_err();
    assert!(matches!(
        err,
        Error::UnsupportedTrait {
            format_type: FormatType::LowerHex,
            ..
        }
    ));
}

#[test]
fn unsupported_binary_format() {
    assert!(formatx!("{:b}", 42).is_err());
}

#[test]
fn parse_error_unmatched_brace() {
    assert!(formatx!("{").is_err());
}

#[test]
fn template_reuse() {
    let t = Template::new("{name} is {age}").unwrap();
    assert_eq!(
        t.render()
            .named("name", &"Alice")
            .named("age", &30)
            .finish()
            .unwrap(),
        "Alice is 30"
    );
    assert_eq!(
        t.render()
            .named("name", &"Bob")
            .named("age", &25)
            .finish()
            .unwrap(),
        "Bob is 25"
    );
}

#[test]
fn template_contains() {
    let t = Template::new("{name} scored {score}").unwrap();
    assert!(t.contains("name"));
    assert!(t.contains("score"));
    assert!(!t.contains("missing"));
}

#[test]
fn template_placeholders() {
    let t = Template::new("{name} {score} {name}").unwrap();
    assert_eq!(t.placeholders(), vec!["name", "score", "name"]);
}

#[test]
fn template_from_str() {
    let t: Template = "{:?}".parse().unwrap();
    assert_eq!(t.render().arg(&42).finish().unwrap(), "42");
}

#[test]
fn unicode_fill() {
    assert_fmt!("{:★>10}", "hi");
}

#[test]
fn zero_width() {
    assert_fmt!("{:0}", 42);
}

#[test]
fn unused_args_allowed() {
    assert_eq!(formatx!("{}", "used", "unused").unwrap(), "used");
}
