use crate::{Assert, AssertEquals};

use std::fmt::Debug;

/// DSL for [Option] assertions.
pub trait AssertOption<T> {
    /// Assert that `actual` is equal to [Some] `expected' value.
    fn is_some(self, expected: T) -> Self;
    /// Assert that `actual` is equal to [None].
    fn is_none(self) -> Self;
    /// Unwrap the [Option] value, panic for [None].
    fn unwrap(self) -> Assert<T>;
}

impl<T> AssertOption<T> for Assert<Option<T>>
where
    T: PartialEq + Debug,
{
    fn is_some(self, expected: T) -> Self {
        self.is(Some(expected))
    }

    fn is_none(self) -> Self {
        self.is(None)
    }

    fn unwrap(self) -> Assert<T> {
        match self.actual {
            Some(value) => Assert::that(value),
            None => panic!(
                "Assertion failed: `(actual.is_some())`
  Actual:   `None`"
            ),
        }
    }
}
