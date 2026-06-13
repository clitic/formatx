//! Public macros for runtime string formatting.

/// Format a runtime string - **strict mode** (default).
///
/// Returns `Err(Error::MissingArgument)` if any placeholder references an
/// argument that was not provided. This is the safe default that catches
/// missing arguments at runtime.
///
/// # Examples
///
/// ```
/// use formatx::formatx;
///
/// let template = "{name} has {count} items";
/// let result = formatx!(template, name = "Alice", count = 42).unwrap();
/// assert_eq!(result, "Alice has 42 items");
///
/// // Positional arguments:
/// let result = formatx!("{} + {} = {}", 1, 2, 3).unwrap();
/// assert_eq!(result, "1 + 2 = 3");
/// ```
#[macro_export]
macro_rules! formatx {
    ($template:expr $(,)?) => {
        (|| -> ::std::result::Result<::std::string::String, $crate::Error> {
            $crate::Template::new($template)?.render().finish()
        })()
    };
    ($template:expr, $($args:tt)*) => {
        (|| -> ::std::result::Result<::std::string::String, $crate::Error> {
            let t = $crate::Template::new($template)?;
            let mut r = t.render();
            $crate::_formatx_internal!(r, $($args)*);
            r.finish()
        })()
    };
}

/// Format a runtime string - **lenient mode**.
///
/// Missing arguments are replaced with an empty string `""` instead of
/// producing an error. Useful for i18n templates where not all placeholders
/// may be filled.
///
/// # Examples
///
/// ```
/// use formatx::formatxl;
///
/// let template = "{greeting} {name}!";
/// let result = formatxl!(template, greeting = "Hello").unwrap();
/// assert_eq!(result, "Hello !");
/// ```
#[macro_export]
macro_rules! formatxl {
    ($template:expr $(,)?) => {
        (|| -> ::std::result::Result<::std::string::String, $crate::Error> {
            $crate::Template::new($template)?.render().finish_lenient()
        })()
    };
    ($template:expr, $($args:tt)*) => {
        (|| -> ::std::result::Result<::std::string::String, $crate::Error> {
            let t = $crate::Template::new($template)?;
            let mut r = t.render();
            $crate::_formatx_internal!(r, $($args)*);
            r.finish_lenient()
        })()
    };
}

/// Internal helper macro for argument collection.
///
/// Recursively processes `name = value` (named) and `value` (positional) arguments.
#[macro_export]
#[doc(hidden)]
macro_rules! _formatx_internal {
    ($r:expr, $name:ident = $value:expr $(,)?) => {
        $r.named(stringify!($name), &$value);
    };
    ($r:expr, $value:expr $(,)?) => {
        $r.arg(&$value);
    };
    ($r:expr, $name:ident = $value:expr, $($rest:tt)*) => {
        $r.named(stringify!($name), &$value);
        $crate::_formatx_internal!($r, $($rest)*);
    };
    ($r:expr, $value:expr, $($rest:tt)*) => {
        $r.arg(&$value);
        $crate::_formatx_internal!($r, $($rest)*);
    };
}
