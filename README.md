# assert4rs

A fluent assertion library for Rust. Write expressive, chainable test assertions that read like natural language — and get failure messages that tell you exactly what went wrong.

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

Type-specific assertions are available for `Option`, `Result`, `String`, `Vec`, arrays, slices, `HashSet`, and `HashMap`:

```rust
use assert4rs::Assert;
use std::collections::{HashMap, HashSet};

Assert::that(HashSet::from([1, 2, 3])).contains(&2);
Assert::that(HashMap::from([("a", 1)])).contains_key(&"a").get(&"a").unwrap().is(1);
```

## Error reporting

Good failure messages are the point of this crate — you shouldn't have to add a `println!` to figure out what actually went wrong. Every assertion reports:

- **The right location.** Panics point at your test's `file:line`, never at assert4rs's internals.
- **What was asserted, and on what.** Give an assertion a label with `.named(...)` when you're checking several things in one test and want to know at a glance which one failed.
- **A diff, not just a dump.** Equality failures (`.is()`) show *where* two values differ, not just their raw `Debug` output side by side.

```rust
use assert4rs::Assert;

Assert::that(25).named("user.age").is(30);
```

```text
Assertion failed for `user.age`: `(actual == expected)`
  Actual:   `25`
  Expected: `30`
             ^ differs at byte 0 ('2' vs '3')
```

### Structural diffs for collections

`.is()` on a `Vec`/`HashMap`/`HashSet` still works, but its diff is text-based — for collections, `.is_eq_to(...)` gives a diff that names actual missing/extra elements instead:

```rust
use assert4rs::Assert;

Assert::that(vec![1, 9, 2, 3]).is_eq_to(vec![1, 2, 3]);
```

```text
Assertion failed: `(actual.is_eq_to(expected))`
  Actual:   `[1, 9, 2, 3]`
  Expected: `[1, 2, 3]`
  Extra:    `[9]`
```

The inserted `9` is named directly — no need to eyeball two lists for the difference. `is_eq_to` is also available on arrays, slices, and `HashSet`, and on `HashMap` it additionally reports changed values for keys present on both sides:

```rust
use assert4rs::Assert;
use std::collections::HashMap;

let actual = HashMap::from([("a", 1), ("b", 2)]);
let expected = HashMap::from([("a", 1), ("b", 99), ("c", 3)]);
Assert::that(actual).is_eq_to(expected);
```

```text
Assertion failed: `(actual.is_eq_to(expected))`
  Actual:   `{"a": 1, "b": 2}`
  Expected: `{"a": 1, "b": 99, "c": 3}`
  Missing keys: `["c"]`
  Changed: `{"b": 2 != 99}`
```

(`HashMap`/`HashSet` don't have a guaranteed iteration order, so the `Actual:`/`Expected:` dump lines may print entries in a different order between runs — the `Missing keys:`/`Extra keys:`/`Changed:` lines are always sorted and stable.)

## License

MIT
