<h1 align="center">formatx</h1>

[![Crate Downloads](https://img.shields.io/crates/d/formatx?logo=rust&style=flat-square)](https://crates.io/formatx)
[![Crate Version](https://img.shields.io/crates/v/formatx?logo=rust&style=flat-square)](https://crates.io/crates/formatx)
[![Build Status](https://img.shields.io/github/actions/workflow/status/clitic/formatx/build.yml?logo=github&style=flat-square)](https://github.com/clitic/formatx/actions)
[![Docs Status](https://img.shields.io/docsrs/formatx?logo=docsdotrs&style=flat-square)](https://docs.rs/formatx)
[![Crate License](https://img.shields.io/crates/l/formatx?style=flat-square)](https://crates.io/crates/formatx)
[![Repo Size](https://img.shields.io/github/repo-size/clitic/formatx?logo=github&style=flat-square)](https://github.com/clitic/formatx)

`formatx` lets you format strings at runtime using the same syntax as [`std::fmt`] (`{}`, `{:?}`, `{name}`, etc.), but with runtime template strings instead of compile-time literals with **zero** dependencies.

<div align="center">
  <img src="https://raw.githubusercontent.com/clitic/formatx/refs/heads/main/docs/images/benchmark.svg" width="700px">
</div>

## Getting Started

Add this to your `Cargo.toml` file.

```toml
[dependencies]
formatx = "0.3"
```

Or add from command line.

```bash
cargo add formatx
```

See [docs](https://docs.rs/formatx) and [examples](https://github.com/clitic/formatx/tree/main/examples) to know how to use it.

## Examples

### Using [`formatx!`](https://docs.rs/formatx/latest/formatx/macro.formatx.html)

Works just like [`format!`](https://doc.rust-lang.org/std/macro.format.html), but accepts runtime template strings.

```rust
use formatx::formatx;

let template = "{} scored {score:.1}% in {}";
let result = formatx!(template, "Alice", "maths", score = 95.678).unwrap();
assert_eq!(result, "Alice scored 95.7% in maths");
```

> [!NOTE]
> Extra arguments that aren't referenced by any placeholder are silently ignored in both `formatx!` and `formatxl!`.

### Template Reuse

Parse once, render many times with [`Template`](https://docs.rs/formatx/latest/formatx/struct.Template.html).

```rust
use formatx::Template;

let template = Template::new("{name} has {n} items").unwrap();

let r1 = template.render()
    .named("name", &"Alice")
    .named("n", &3)
    .finish()
    .unwrap();

let r2 = template.render()
    .named("name", &"Bob")
    .named("n", &7)
    .finish()
    .unwrap();

assert_eq!(r1, "Alice has 3 items");
assert_eq!(r2, "Bob has 7 items");
```

## Supported Syntax

`formatx` supports most of the [`std::fmt`] formatting syntax:

| Feature | Example | Supported |
|---|---|---|
| Implicit positional | `{}` | ✅ |
| Explicit positional | `{0} {1}` | ✅ |
| Named arguments | `{name}` | ✅ |
| Mixed positional | `{1} {} {0} {}` | ✅ |
| Debug | `{:?}`, `{:#?}` | ✅ |
| Debug hex | `{:x?}`, `{:X?}` | ✅ |
| Width | `{:10}` | ✅ |
| Precision | `{:.5}` | ✅ |
| Fill and align | `{:-<10}`, `{:^10}`, `{:*>10}` | ✅ |
| Sign | `{:+}` | ✅ |
| Alternate | `{:#}` | ✅ |
| Zero-pad | `{:05}` | ✅ |
| `$`-parameter width/precision | `{:width$}`, `{:.prec$}` | ✅ |
| Star precision | `{:.*}` | ✅ |
| Escaped braces | `{{` `}}` | ✅ |
| LowerHex | `{:x}` | ❌ |
| UpperHex | `{:X}` | ❌ |
| Octal | `{:o}` | ❌ |
| Binary | `{:b}` | ❌ |
| LowerExp | `{:e}` | ❌ |
| UpperExp | `{:E}` | ❌ |
| Pointer | `{:p}` | ❌ |

> [!NOTE]
> Only types implementing [`Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html) + [`Debug`](https://doc.rust-lang.org/std/fmt/trait.Debug.html) are supported. Other [formatting traits](https://doc.rust-lang.org/std/fmt/#formatting-traits) (`LowerHex`, `Binary`, `Octal`, etc.) are not supported and will return [`Error::UnsupportedTrait`](https://docs.rs/formatx/latest/formatx/enum.Error.html#variant.UnsupportedTrait).

> [!NOTE]
> Local variable interpolation is not supported since template strings are parsed at runtime.
>
>   ```rust
>   let people = "Rustaceans";
>   // This will NOT interpolate `people` - use named args instead:
>   formatx!("Hello {people}!", people = people).unwrap();
>   ```

## Status

[![Packaging status](https://repology.org/badge/vertical-allrepos/rust%3Aformatx.svg)](https://repology.org/project/rust%3Aformatx/versions)

## License

Dual Licensed

- [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0) ([LICENSE-APACHE](LICENSE-APACHE))
- [MIT license](https://opensource.org/licenses/MIT) ([LICENSE-MIT](LICENSE-MIT))

[`std::fmt`]: https://doc.rust-lang.org/std/fmt/#syntax