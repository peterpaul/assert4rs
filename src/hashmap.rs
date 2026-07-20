use crate::Assert;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

/// DSL for [HashMap].
impl<K, V> Assert<HashMap<K, V>> {
    /// Assert that the actual map contains the given key.
    ///
    /// ```
    /// # use assert4rs::Assert;
    /// # use std::collections::HashMap;
    /// Assert::that(HashMap::from([("a", 1)])).contains_key(&"a");
    /// ```
    ///
    /// ```should_panic
    /// # use assert4rs::Assert;
    /// # use std::collections::HashMap;
    /// Assert::that(HashMap::from([("a", 1)])).contains_key(&"z");
    /// ```
    #[track_caller]
    pub fn contains_key(self, key: &K) -> Self
    where
        K: Eq + Hash + Debug,
        V: Debug,
    {
        assert!(
            self.actual.contains_key(key),
            "{}\n  Actual:   `{:?}`\n  Expected to contain key: `{:?}`\n  Missing key: `{:?}`",
            self.header("actual.contains_key(key)"),
            self.actual,
            key,
            key,
        );
        self
    }

    /// Returns an [Assert] for the value at `key`, or `None` if absent.
    /// Consumes the map entry (via [HashMap::remove]) so this works
    /// without requiring `V: Clone`, matching `vec.rs`'s `get` pattern.
    ///
    /// ```
    /// # use assert4rs::Assert;
    /// # use std::collections::HashMap;
    /// Assert::that(HashMap::from([("a", 1)])).get(&"a").unwrap().is(1);
    /// Assert::that(HashMap::from([("a", 1)])).get(&"z").is_none();
    /// ```
    pub fn get(mut self, key: &K) -> Assert<Option<V>>
    where
        K: Eq + Hash,
    {
        Assert::that(self.actual.remove(key))
    }

    /// Assert that the actual map is empty.
    ///
    /// ```
    /// # use assert4rs::Assert;
    /// # use std::collections::HashMap;
    /// Assert::that(HashMap::<&str, i32>::new()).is_empty();
    /// ```
    ///
    /// ```should_panic
    /// # use assert4rs::Assert;
    /// # use std::collections::HashMap;
    /// Assert::that(HashMap::from([("a", 1)])).is_empty();
    /// ```
    #[track_caller]
    pub fn is_empty(self) -> Self
    where
        K: Debug,
        V: Debug,
    {
        assert!(
            self.actual.is_empty(),
            "{}\n  Actual: `{:?}`",
            self.header("actual.is_empty()"),
            self.actual,
        );
        self
    }

    /// Assert that the actual map has the given length.
    ///
    /// ```
    /// # use assert4rs::Assert;
    /// # use std::collections::HashMap;
    /// Assert::that(HashMap::from([("a", 1)])).has_length(1);
    /// ```
    ///
    /// ```should_panic
    /// # use assert4rs::Assert;
    /// # use std::collections::HashMap;
    /// Assert::that(HashMap::from([("a", 1)])).has_length(2);
    /// ```
    #[track_caller]
    pub fn has_length(self, expected: usize) -> Self {
        assert!(
            self.actual.len() == expected,
            "{}\n  Actual:   `{}`\n  Expected: `{}`",
            self.header("actual.len() == expected"),
            self.actual.len(),
            expected,
        );
        self
    }
}

/// Structural-diff equality check. Requires `K: Ord` and `V: PartialEq`
/// (beyond what `contains_key`/`get`/`is_empty`/`has_length` need) so
/// the diff's Missing/Extra/Changed lists can be sorted for
/// deterministic output and values can be compared for the `Changed`
/// category.
impl<K, V> Assert<HashMap<K, V>>
where
    K: Eq + Hash + Debug + Ord,
    V: PartialEq + Debug,
{
    /// Assert that `self` equals `expected`, reporting a structural
    /// (entry-aware) diff on failure instead of a text diff.
    ///
    /// ```
    /// # use assert4rs::Assert;
    /// # use std::collections::HashMap;
    /// Assert::that(HashMap::from([("a", 1)])).is_eq_to(HashMap::from([("a", 1)]));
    /// ```
    ///
    /// ```should_panic
    /// # use assert4rs::Assert;
    /// # use std::collections::HashMap;
    /// Assert::that(HashMap::from([("a", 1)])).is_eq_to(HashMap::from([("a", 2)]));
    /// ```
    #[track_caller]
    pub fn is_eq_to(self, expected: HashMap<K, V>) -> Self {
        if self.actual == expected {
            return self;
        }
        let (missing_keys, extra_keys, changed) =
            crate::structural_diff::map_diff(&self.actual, &expected);
        let mut message = format!(
            "{}\n  Actual:   `{:?}`\n  Expected: `{:?}`",
            self.header("actual.is_eq_to(expected)"),
            self.actual,
            expected,
        );
        if !missing_keys.is_empty() {
            message.push_str(&format!("\n  Missing keys: `{missing_keys:?}`"));
        }
        if !extra_keys.is_empty() {
            message.push_str(&format!("\n  Extra keys: `{extra_keys:?}`"));
        }
        if !changed.is_empty() {
            let rendered: Vec<String> = changed
                .iter()
                .map(|(k, av, ev)| format!("{k:?}: {av:?} != {ev:?}"))
                .collect();
            let changed_repr = format!("{{{}}}", rendered.join(", "));
            message.push_str(&format!("\n  Changed: `{changed_repr}`"));
        }
        panic!("{message}");
    }
}

#[cfg(test)]
mod tests {
    use crate::Assert;
    use std::collections::HashMap;

    #[test]
    #[should_panic(
        expected = "Assertion failed: `(actual.contains_key(key))`\n  Actual:   `{\"a\": 1}`\n  Expected to contain key: `\"z\"`\n  Missing key: `\"z\"`"
    )]
    fn contains_key_reports_full_message() {
        Assert::that(HashMap::from([("a", 1)])).contains_key(&"z");
    }

    #[test]
    #[should_panic(expected = "Assertion failed for `x`: `(actual.contains_key(key))`")]
    fn contains_key_reports_label_when_named() {
        Assert::that(HashMap::from([("a", 1)]))
            .named("x")
            .contains_key(&"z");
    }

    #[test]
    #[should_panic(expected = "Assertion failed: `(actual.is_empty())`\n  Actual: `{\"a\": 1}`")]
    fn is_empty_reports_full_message() {
        Assert::that(HashMap::from([("a", 1)])).is_empty();
    }

    #[test]
    #[should_panic(expected = "Assertion failed for `x`: `(actual.is_empty())`")]
    fn is_empty_reports_label_when_named() {
        Assert::that(HashMap::from([("a", 1)]))
            .named("x")
            .is_empty();
    }

    #[test]
    #[should_panic(
        expected = "Assertion failed: `(actual.len() == expected)`\n  Actual:   `1`\n  Expected: `5`"
    )]
    fn has_length_reports_full_message() {
        Assert::that(HashMap::from([("a", 1)])).has_length(5);
    }

    #[test]
    #[should_panic(expected = "Assertion failed for `x`: `(actual.len() == expected)`")]
    fn has_length_reports_label_when_named() {
        Assert::that(HashMap::from([("a", 1)]))
            .named("x")
            .has_length(5);
    }

    #[test]
    #[should_panic(expected = "Assertion failed: `(actual.is_eq_to(expected))`")]
    fn is_eq_to_reports_generic_header_without_label() {
        Assert::that(HashMap::from([("a", 1)])).is_eq_to(HashMap::from([("a", 2)]));
    }

    #[test]
    #[should_panic(expected = "Assertion failed for `x`: `(actual.is_eq_to(expected))`")]
    fn is_eq_to_reports_label_when_named() {
        Assert::that(HashMap::from([("a", 1)]))
            .named("x")
            .is_eq_to(HashMap::from([("a", 2)]));
    }

    #[test]
    fn is_eq_to_reports_missing_extra_and_changed_together() {
        let actual = HashMap::from([("a", 1), ("b", 2), ("d", 4)]);
        let expected = HashMap::from([("a", 1), ("b", 99), ("c", 3)]);
        let result = std::panic::catch_unwind(|| {
            Assert::that(actual).is_eq_to(expected);
        });
        let message = result.unwrap_err();
        let message = message.downcast_ref::<String>().unwrap();
        assert!(
            message.contains("Missing keys: `[\"c\"]`"),
            "message: {message}"
        );
        assert!(
            message.contains("Extra keys: `[\"d\"]`"),
            "message: {message}"
        );
        assert!(
            message.contains("Changed: `{\"b\": 2 != 99}`"),
            "message: {message}"
        );
    }
}
