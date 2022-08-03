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
  <a href="https://docs.rs/kdam/latest/formatx.rs">
    <img src="https://img.shields.io/docsrs/formatx.rs?logo=docsdotrs&style=flat-square">
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

<!-- 
A crate for string formatting using runtime format strings.

This crate provides much the same facilities as `std::fmt`, with the
additional allowance for format strings which are not known until runtime.
Possible applications include internationalization, scripting, or other
customization.

The syntax for format strings and for macro invocations is equivalent to
that used by `std::fmt`, including support for positional and named
arguments. This crate shells out to the standard library implementations
for as much as possible to ensure feature parity. -->

<!-- - [How can I use a dynamic format string with the format! macro?](https://stackoverflow.com/questions/32572486/how-can-i-use-a-dynamic-format-string-with-the-format-macro) -->


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

See [docs](https://docs.rs/formatx) and [examples](https://github.com/clitic/formatx.rs/tree/main/examples) to 
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

## Alternatives

1. [strfmt](https://github.com/vitiral/strfmt)
2. [runtime-fmt](https://github.com/SpaceManiac/runtime-fmt)

## License

Dual licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)
