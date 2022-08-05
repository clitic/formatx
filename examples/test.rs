use formatx::formatx;

macro_rules! formatx_test {
    ($template: expr, $($values: tt)*) => {
        assert_eq!(formatx!($template, $($values)*).unwrap(), format!($template, $($values)*))
    }
}

fn main() {
    formatx_test!("Hello {:05}!", -5);
    formatx_test!("Hello {:+05}!", -5);
    formatx_test!("Hello {:+05}!", 5);
}
