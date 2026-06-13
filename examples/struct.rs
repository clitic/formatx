use formatx::formatx;

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let origin = Point { x: 0.0, y: 0.0 };
    let target = Point { x: 3.5, y: -2.1 };

    // Display
    let result = formatx!("From {} to {}", origin, target).unwrap();
    println!("{result}");

    // Debug
    let result = formatx!("Debug: {:?}", origin).unwrap();
    println!("{result}");

    // Pretty debug
    let result = formatx!("Pretty: {:#?}", origin).unwrap();
    println!("{result}");

    // Template reuse
    let template = formatx::Template::new("Point {name}: {point}").unwrap();
    let r1 = template.render()
        .named("name", &"Origin")
        .named("point", &origin)
        .finish()
        .unwrap();
    let r2 = template.render()
        .named("name", &"Target")
        .named("point", &target)
        .finish()
        .unwrap();
    println!("{r1}");
    println!("{r2}");
}
