# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0] - 2026-07-19

### Added

- `Assert::named(label)` to attach a custom label to an assertion, shown in the panic message if it fails.
- A diff pointer on `.is()` equality failures, showing the first byte at which the actual and expected `Debug` output differ.
- `#[track_caller]` on every panicking assertion method, so panics report the caller's file:line instead of pointing inside assert4rs.
- `Assert<HashSet<T>>` — `contains`, `is_empty`, `has_length`.
- `Assert<HashMap<K, V>>` — `contains_key`, `get`, `is_empty`, `has_length`.
- `is_eq_to` on `Vec<T>`, arrays, slices, `HashSet<T>`, and `HashMap<K, V>` — a structural equality check that reports missing/extra elements (or missing/extra/changed keys for maps) instead of a raw text diff.

## [0.2.1] - 2026-03-25

Administrative release with documentation such as

- README.md
- RELEASE_NOTES.md

## [0.2.0] - 2026-02-10

### Added

- **Result** — `is_ok`, `is_err`, `unwrap`, `unwrap_err`
- **Ordering** — `is_gt`, `is_ge`, `is_lt`, `is_le`
- **String** — `starts_with`, `ends_with`, `contains`
- **Vec** — `is_empty`, `has_length`
- **`satisfies(predicate)`** — custom predicate assertion for any `Debug` type

### Changed

- Replaced trait-based API with inherent `impl` blocks — no more trait imports needed
- Updated to Rust edition 2021

## [0.1.0] - 2020-12-18

Initial release with core fluent assertion API.

- **`Assert::that(actual)`** entry point with method chaining
- **Equality** — `is`, `is_not`
- **Option** — `is_some`, `is_none`, `unwrap`
- **Vec** — `contains`, `get`
- **`map`** — transform the actual value for further assertions

[Unreleased]: https://github.com/peterpaul/assert4rs/compare/0.3.0...HEAD
[0.3.0]: https://github.com/peterpaul/assert4rs/compare/0.2.2...0.3.0
[0.2.1]: https://github.com/peterpaul/assert4rs/compare/0.2.0...0.2.1
[0.2.0]: https://github.com/peterpaul/assert4rs/compare/0.1.0...0.2.0
[0.1.0]: https://github.com/peterpaul/assert4rs/releases/tag/0.1.0
