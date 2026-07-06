# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `no_std` support — the crate now works in `no_std` environments (requires `alloc`). ([#13](https://github.com/clitic/formatx/pull/13))
- `&str` values can now be passed directly to `Renderer::arg()` and `Renderer::named()` without wrapping in `&`. ([#13](https://github.com/clitic/formatx/pull/13))
- `FormatArg` enum for zero-allocation argument storage with a dedicated `Str` variant for string slices.
- `IntoFormatArg` trait for ergonomic argument conversion.
- `FormatValue` is now implemented for unsized types (`?Sized` bound).

### Changed

- **Breaking:** `Template` now borrows the source string (`Template<'a>` with `&'a str`) instead of owning a `String`. This eliminates an allocation but requires the source to outlive the template.
- **Breaking:** `Template::new()` now takes `&'a str` directly instead of `S: Into<String>`.
- **Breaking:** `FromStr` impl for `Template` removed; replaced with `TryFrom<&'a str>`.
- `Renderer::arg()` and `Renderer::named()` now accept `impl IntoFormatArg<'a>` instead of `&'a (impl Display + Debug)`.
- `FormatSpec` and related AST types now derive `PartialEq`; `FormatSpec::is_default()` simplified.
- Introduced `NamedArgs` type alias for internal readability.

## [0.3.0] - 2026-06-15

### Added

- `formatxl!` macro for **lenient mode** - missing arguments are replaced with `""` instead of returning an error.
- Support for mixed positional and implicit arguments (`{1} {} {0} {}`).
- Support for `$`-parameter width and precision (`{:width$}`, `{:.prec$}`).
- Support for star precision (`{:.*}`).

### Changed

- New AST-based parser with span tracking for better error messages.
- Dual-path format engine: fast path for plain `{}` / `{:?}`, full path for complex specs.
- Manual padding for custom fill/align, delegating to `std::fmt` otherwise.
- Removed support for HTML-like attributes to maintain full parity with `std::fmt` syntax.

## [0.2.4] - 2024-05-04

### Fixed

- Suppport for trailing commas when calling `formatx!` macro. ([#11](https://github.com/clitic/formatx/pull/11))

## [0.2.3] - 2024-12-15

### Fixed

- clippy warnings. ([#6](https://github.com/clitic/formatx/pull/6))

## [0.2.2] - 2023-12-03

### Fixed

- Use `std::result::Result` in `formatx!` macro. ([#4](https://github.com/clitic/formatx/pull/4))

## [0.2.1] - 2023-02-03

### Fixed

- `formatx!` macro clippy warnings.

## [0.2.0] - 2022-12-06

### Added

- Better `formatx!` macro expansion.

### Changed

- `formatx!(template: expr)` macro returns `Result<String, formatx::Error>`.

## [0.1.4] - 2022-08-15

## [0.1.3] - 2022-08-05

## [0.1.2] - 2022-08-05

## [0.1.1] - 2022-08-05

## [0.1.0] - 2022-08-03

[Unreleased]: https://github.com/clitic/formatx/compare/0.3.0...HEAD
[0.3.0]: https://github.com/clitic/formatx/compare/0.2.4...0.3.0
[0.2.4]: https://github.com/clitic/formatx/compare/0.2.3...0.2.4
[0.2.3]: https://github.com/clitic/formatx/compare/0.2.2...0.2.3
[0.2.2]: https://github.com/clitic/formatx/compare/0.2.1...0.2.2
[0.2.1]: https://github.com/clitic/formatx/compare/0.2.0...0.2.1
[0.2.0]: https://github.com/clitic/formatx/compare/c672c19...0.2.0
[0.1.4]: https://github.com/clitic/formatx/compare/b2ee21f...c672c19
[0.1.3]: https://github.com/clitic/formatx/compare/39eb3ee...b2ee21f
[0.1.2]: https://github.com/clitic/formatx/compare/0f282e2...39eb3ee
[0.1.1]: https://github.com/clitic/formatx/compare/454cd82...0f282e2
[0.1.0]: https://github.com/clitic/formatx/compare/0a4cc2d...454cd82
