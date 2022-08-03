use formatx::Template;

fn main() {
    let mut template = Template::new("{percentage color=true:.2} => {percentage color=false:.0}%").unwrap();

    template.replace_with_callback("percentage", 99.9999, |formatted_value, placeholder| {
        if let Some(color) = placeholder.attr("color") {
            if color == "true" {
                return "\x1b[31m".to_owned() + &formatted_value + "\x1b[0m";
            }
        }

        formatted_value
    });

    println!("{}", template.text().unwrap());
}
