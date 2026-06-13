use formatx::formatx;

fn main() {
    // Basic usage with named arguments
    let template = "{name} has {count} items";
    let result = formatx!(template, name = "Alice", count = 42).unwrap();
    println!("{result}");

    // Positional arguments
    let result = formatx!("{} + {} = {}", 1, 2, 3).unwrap();
    println!("{result}");

    // Mixed positional and named
    let greeting = "Hello, {name}! You scored {0}%";
    let result = formatx!(greeting, 95, name = "Bob").unwrap();
    println!("{result}");

    // Debug formatting
    let result = formatx!("{:?}", "a string").unwrap();
    println!("{result}");
}
