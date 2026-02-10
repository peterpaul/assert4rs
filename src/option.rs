use crate::Assert;

use std::fmt::Debug;

impl<T> Assert<Option<T>>
where
    T: PartialEq + Debug,
{
    /// Assert that `actual` is equal to [Some] `expected' value.
    pub fn is_some(self, expected: T) -> Self {
        self.is(Some(expected))
    }

    /// Assert that `actual` is equal to [None].
    pub fn is_none(self) -> Self {
        self.is(None)
    }

    /// Unwrap the [Option] value, panic for [None].
    pub fn unwrap(self) -> Assert<T> {
        match self.actual {
            Some(value) => Assert::that(value),
            None => panic!(
                "Assertion failed: `(actual.is_some())`
  Actual:   `None`"
            ),
        }
    }
}
