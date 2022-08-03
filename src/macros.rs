/// Creates a `String` using interpolation of runtime expressions at runtime.
///
/// The first argument `formatx!` receives is a format string.
/// The power of the formatting string is in the `{}`s contained.
///
/// Additional parameters passed to `formatx!` replace the `{}`s within the
/// formatting string in the order given unless named or positional parameters
/// are used; see [`std::fmt`] for more information.
///
/// A common use for `formatx!` is concatenation and interpolation of strings at runtime.
/// The same convention is used with [`print!`] and [`write!`] macros,
/// depending on the intended destination of the string.
///
/// To convert a single value to a string, use the `to_string` method. This
/// will use the [`Display`] formatting trait.
///
/// [`std::fmt`]: ../std/fmt/index.html
/// [`print!`]: ../std/macro.print.html
/// [`write!`]: core::write
/// [`Display`]: core::fmt::Display
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
/// Any type could be formatted if it implements `std::fmt::Display` + `std::fmt::Debug` traits.
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

    #[test]
    fn precision() {
        assert_eq!(
            formatx!("{:.2}", 99.9999).unwrap(),
            format!("{:.2}", 99.9999),
        );
    }
}
