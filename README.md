<h1 align="center">formatx.rs</h1>

<p align="center">
  <a href="https://crates.io/crates/formatx">
    <img src="https://img.shields.io/crates/d/formatx?style=flat-square">
  </a>
  <a href="https://crates.io/crates/formatx">
    <img src="https://img.shields.io/crates/v/formatx?style=flat-square">
  </a>
  <a href="https://github.com/clitic/formatx.rs">
    <img src="https://img.shields.io/github/workflow/status/clitic/formatx.rs/Rust?logo=github&style=flat-square">
  </a>
  <a href="https://docs.rs/formatx/latest/formatx">
    <img src="https://img.shields.io/docsrs/formatx?logo=docsdotrs&style=flat-square">
  </a>
  <a href="https://github.com/clitic/formatx.rs#license">
    <img src="https://img.shields.io/crates/l/formatx?style=flat-square">
  </a>
  <a href="https://github.com/clitic/formatx.rs">
    <img src="https://img.shields.io/github/repo-size/clitic/formatx?style=flat-square">
  </a>
  <a href="https://github.com/clitic/formatx.rs">
    <img src="https://img.shields.io/tokei/lines/github/clitic/formatx.rs?logo=github&style=flat-square">
  </a>
</p>

<p align="center">A macro for formatting non literal strings at runtime in Rust.</p>

This library aims for formatting **strings** and **numbers** rather than an generic type. Syntax for the format string is derived from [std::fmt](https://doc.rust-lang.org/std/fmt/#syntax).

## Getting Started

Add this to your Cargo.toml file.

```toml
[dependencies]
formatx = "0.1"
```

Or add from command line.

```bash
$ cargo add formatx@0.1
```

See [docs](https://docs.rs/formatx/latest/formatx) and [examples](https://github.com/clitic/formatx.rs/tree/main/examples) to 
know how to use it.

## Example

SOURCE: [format! with non literal string](https://users.rust-lang.org/t/format-with-non-literal-string/2057)

```rust
use formatx::formatx;

fn message(language: &str, name: &str, number: i32) -> String {
    let s = match language {
        "french" => "Bonjour {}, le nombre est {}",
        "spanish" => "Hola {}, el numero es {}",
        _ => "Hi {}, the number is {}",
    };
    formatx!(s, name, number).unwrap()
}

fn main() {
    println!("{}", message("french", "Léa", 1));
    println!("{}", message("spanish", "Sofia", 2));
    println!("{}", message("english", "Ashley", 3));
}
```

OUTPUT

```
Bonjour Léa, le nombre est 1
Hola Sofia, el numero es 2
Hi Ashley, the number is 3
```

## Limitations

> **Examples given below will always panic.**

1. Only types which implements [Display](https://doc.rust-lang.org/std/fmt/trait.Display.html) + [Debug](https://doc.rust-lang.org/std/fmt/trait.Debug.html) traits are supported. Other [formatting-traits](https://doc.rust-lang.org/std/fmt/#formatting-traits) aren't supported.

2. Local variable interpolation isn't supported.

```rust
let people = "Rustaceans";
formatx!("Hello {people}!").unwrap().text().unwrap();
```

3. Intermingling the two types of [positional](https://doc.rust-lang.org/std/fmt/#positional-parameters) specifiers isn't supported. 

```rust
formatx!("{1} {} {0} {}", 1, 2).unwrap();
```

4. Parameter setting through `$` sign argument isn't supported.

```rust
formatx!("{:width$}!", "x", width = 5).unwrap();
```

5. An asterisk `.*` can't be used to set [precision](https://doc.rust-lang.org/std/fmt/#precision).

```rust
formatx!("{:.*}", 5, 0.01).unwrap();
```

## Alternatives

1. [strfmt](https://github.com/vitiral/strfmt)
2. [runtime-fmt](https://github.com/SpaceManiac/runtime-fmt)

## License

Dual Licensed

- [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0) ([LICENSE-APACHE](LICENSE-APACHE))
- [MIT license](https://opensource.org/licenses/MIT) ([LICENSE-MIT](LICENSE-MIT))
