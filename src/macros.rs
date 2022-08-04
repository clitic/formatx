/// Creates a `String` using interpolation of runtime expressions at runtime.
///
/// The first argument `formatx!` receives is a format string.
/// The power of the formatting string is in the `{}`s contained.
///
/// Additional parameters passed to `formatx!` replace the `{}`s within the
/// formatting string in the order given unless named or positional parameters
/// are used; see [std::fmt](std::fmt) for more information.
///
/// A common use for `formatx!` is concatenation and interpolation of strings at runtime.
/// The same convention is used with [print!](std::print) and [write!](core::write) macros,
/// depending on the intended destination of the string.
///
/// To convert a single value to a string, use the `to_string` method. This
/// will use the [Display](core::fmt::Display) formatting trait.
///
/// # Panics
///
/// `formatx!` panics if a formatting trait implementation returns an error.
/// This indicates an incorrect implementation
/// since `fmt::Write for String` never returns an error itself.
/// Additonally `formatx!` returns a `Result` type which can be resolved later.
///
/// # Examples
///
/// Any type could be formatted if it implements [Display](std::fmt::Display) + [Debug](std::fmt::Debug) traits.
///
/// ```
/// use formatx::formatx;
///
/// formatx!("test").unwrap().text().unwrap();
/// formatx!("hello {}", "world!").unwrap();
/// formatx!("x = {}, y = {y}", 10, y = 30).unwrap();
/// ```
#[macro_export]
macro_rules! formatx {
    ($template: expr) => {
        $crate::Template::new($template)
    };

    ($template: expr, $($values: tt)*) => {{
        let template = $crate::Template::new($template);

        if let Err(err) = template {
            Err(err)
        } else {
            let mut template = template.unwrap();
            $crate::formatx_internal!(template, $($values)*);
            template.text()
        }
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! formatx_internal {
    ($template: expr, $name: ident = $value: expr) => {
        $template.replace(stringify!($name), $value);
    };

    ($template: expr, $value: expr) => (
        $template.replace_positional($value);
    );

    ($template: expr, $name: ident = $value: expr, $($values: tt)*) => {
        $template.replace(stringify!($name), $value);
        $crate::formatx_internal!($template, $($values)*);
    };

    ($template: expr, $value:expr, $($values: tt)*) => {
    	$template.replace_positional($value);
        $crate::formatx_internal!($template, $($values)*);
    };
}

#[cfg(test)]
mod tests {
    use crate::formatx;

    macro_rules! formatx_test {
        ($template: expr, $($values: tt)*) => {
            assert_eq!(formatx!($template, $($values)*).unwrap(), format!($template, $($values)*))
        }
    }

    #[test]
    fn text() {
        assert_eq!(formatx!("Hello").unwrap().text().unwrap(), format!("Hello"));
    }

    #[test]
    fn positional() {
        formatx_test!("Hello, {}!", "world");
        formatx_test!("The number is {}", 1);
        formatx_test!("{} {}", 1, 2);
    }

    #[test]
    fn named() {
        formatx_test!("{value}", value = 4);
        formatx_test!("{argument}", argument = "test");
        formatx_test!("{a} {c} {b}", a = "a", b = 'b', c = 3);
    }

    #[test]
    fn intermixed() {
        formatx_test!("{name} {}", 1, name = 2);
    }

    #[test]
    fn width() {
        formatx_test!("Hello {:5}!", "x");
        formatx_test!("Hello {:05}!", -5);
    }

    #[test]
    fn zero_width() {
        formatx_test!("{:04}", 42);
        formatx_test!("Hello {:05}!", 5);
    }

    #[test]
    fn align() {
        formatx_test!("Hello {:<5}!", "x");
        formatx_test!("Hello {:-<5}!", "x");
        formatx_test!("Hello {:^5}!", "x");
        formatx_test!("Hello {:>5}!", "x");
    }

    #[test]
    fn sign() {
        formatx_test!("Hello {:+}!", 5)
    }

    #[test]
    fn precision() {
        formatx_test!("Hello {0} is {1:.5}", "x", 0.01)
    }

    #[test]
    fn escaping() {
        assert_eq!(
            formatx!("Hello {{}}").unwrap().text().unwrap(),
            format!("Hello {{}}")
        );
        assert_eq!(
            formatx!("{{ Hello").unwrap().text().unwrap(),
            format!("{{ Hello")
        );
    }

    #[test]
    fn debug() {
        formatx_test!("{} {:?}", 3, 4);
        formatx_test!("{} {:?}", 'a', 'b');
        formatx_test!("{} {:?}", "foo\n", "bar\n");
    }
}
