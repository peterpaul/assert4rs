# assert4rs

A fluent assertion library for Rust. Write expressive, chainable test assertions that read like natural language.

```rust
use assert4rs::Assert;

Assert::that(vec![1, 2, 3])
    .contains(&2)
    .has_length(3);

Assert::that(String::from("hello world"))
    .starts_with("hello")
    .ends_with("world");

Assert::that(Some(42))
    .unwrap()
    .is_gt(0)
    .satisfies(|v| v % 2 == 0);
```

## Usage

Add to your `Cargo.toml`:

```toml
[dev-dependencies]
assert4rs = "0.2"
```

All assertions start with `Assert::that(value)` and can be chained fluently:

```rust
use assert4rs::Assert;

Assert::that("foo")
    .is("foo")
    .is_not("bar");
```

No trait imports needed — everything works through `Assert` alone.

## License

MIT
