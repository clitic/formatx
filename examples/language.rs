use formatx::formatxl;

fn main() {
    // i18n example: templates with possibly missing arguments
    let templates = [
        ("en", "Hello {name}, welcome to {place}!"),
        ("es", "Hola {name}, bienvenido a {place}!"),
        ("ja", "{name}さん、{place}へようこそ！"),
    ];

    for (lang, template) in templates {
        // Lenient mode: missing "place" won't error
        let result = formatxl!(template, name = "Alice").unwrap();
        println!("[{lang}] {result}");
    }
}
