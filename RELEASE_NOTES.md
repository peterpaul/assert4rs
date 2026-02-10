# Release Notes

## 0.2.0

### New assertions

- **Result** — `is_ok`, `is_err`, `unwrap`, `unwrap_err`
- **Ordering** — `is_gt`, `is_ge`, `is_lt`, `is_le`
- **String** — `starts_with`, `ends_with`, `contains`
- **Vec** — `is_empty`, `has_length`
- **`satisfies(predicate)`** — custom predicate assertion for any `Debug` type

### Improvements

- Replaced trait-based API with inherent `impl` blocks — no more trait imports needed
- Updated to Rust edition 2021

## 0.1.0

Initial release with core fluent assertion API.

- **`Assert::that(actual)`** entry point with method chaining
- **Equality** — `is`, `is_not`
- **Option** — `is_some`, `is_none`, `unwrap`
- **Vec** — `contains`, `get`
- **`map`** — transform the actual value for further assertions
