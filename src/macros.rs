//! Public macros for runtime string formatting.

/// Format a runtime string in **strict mode**.
///
/// Returns [`Result`](core::result::Result)`<`[`String`](alloc::string::String)`, `[`Error`](crate::Error)`>`.
/// Produces [`Error::MissingArgument`](crate::Error::MissingArgument) if any placeholder references an argument that was not provided.
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
    ($($args:tt)*) => {
        $crate::_formatx_impl!(finish, $($args)*)
    };
}

/// Format a runtime string in **lenient mode**.
///
/// Returns [`Result`](core::result::Result)`<`[`String`](alloc::string::String)`, `[`Error`](crate::Error)`>`.
/// Missing arguments are replaced with an empty string `""` instead of producing an error.
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
    ($($args:tt)*) => {
        $crate::_formatx_impl!(finish_lenient, $($args)*)
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! _formatx_impl {
    // Template only, no arguments.
    ($finish:ident, $template:expr $(,)?) => {
        match $crate::Template::new(&$template) {
            Ok(t) => t.render().$finish(),
            Err(e) => Err(e),
        }
    };
    // Template with arguments.
    ($finish:ident, $template:expr, $($args:tt)*) => {
        match $crate::Template::new(&$template) {
            Ok(t) => {
                let mut r = t.render();
                $crate::_formatx_impl!(@arg r, $($args)*);
                r.$finish()
            }
            Err(e) => Err(e),
        }
    };
    // Named argument (last or trailing comma).
    (@arg $r:expr, $name:ident = $value:expr $(,)?) => {
        let v = &$value;
        $r.named(stringify!($name), v);
    };
    // Positional argument (last or trailing comma).
    (@arg $r:expr, $value:expr $(,)?) => {
        let v = &$value;
        $r.arg(v);
    };
    // Named argument followed by more arguments.
    (@arg $r:expr, $name:ident = $value:expr, $($rest:tt)*) => {
        let v = &$value;
        $r.named(stringify!($name), v);
        $crate::_formatx_impl!(@arg $r, $($rest)*);
    };
    // Positional argument followed by more arguments.
    (@arg $r:expr, $value:expr, $($rest:tt)*) => {
        let v = &$value;
        $r.arg(v);
        $crate::_formatx_impl!(@arg $r, $($rest)*);
    };
}
