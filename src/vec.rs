use crate::Assert;
use std::fmt::Debug;

/// DSL for [Vec].
impl<T> Assert<Vec<T>>
where
    T: PartialEq + Debug,
{
    /// Assert that the actual vector contains a specific `expected`
    /// value.
    ///
    /// ```
    /// # use assert4rs::{Assert};
    /// Assert::that(vec![1, 2, 3]).contains(&2);
    /// ```
    pub fn contains(self, expected: &T) -> Self {
        assert!(
            self.actual.contains(expected),
            "Assertion failed: `(actual.contains(expected))`
  Actual:   `{:?}`
  Expected: `{:?}`",
            self.actual,
            expected
        );
        self
    }

    /// Returns an [Assert] for a value from the [Vec].
    ///
    /// ```
    /// # use assert4rs::{Assert};
    /// Assert::that(vec!['a', 'b', 'c']).get(1).is_some('b');
    /// Assert::that(vec!['a', 'b', 'c']).get(5).is_none();
    /// ```
    pub fn get(mut self, index: usize) -> Assert<Option<T>> {
        if index < self.actual.len() {
            Assert::that(Some(self.actual.swap_remove(index)))
        } else {
            Assert::that(None)
        }
    }
}
