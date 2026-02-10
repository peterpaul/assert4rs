use crate::Assert;

use std::fmt::Debug;

impl<T> Assert<Option<T>>
where
    T: PartialEq + Debug,
{
    /// Assert that `actual` is equal to [Some] `expected` value.
    ///
    /// ```
    /// # use assert4rs::Assert;
    /// Assert::that(Some(2)).is_some(2);
    /// ```
    ///
    /// ```should_panic
    /// # use assert4rs::Assert;
    /// Assert::that(None::<i32>).is_some(2);
    /// ```
    pub fn is_some(self, expected: T) -> Self {
        self.is(Some(expected))
    }

    /// Assert that `actual` is equal to [None].
    ///
    /// ```
    /// # use assert4rs::Assert;
    /// Assert::that(None::<i32>).is_none();
    /// ```
    ///
    /// ```should_panic
    /// # use assert4rs::Assert;
    /// Assert::that(Some(2)).is_none();
    /// ```
    pub fn is_none(self) -> Self {
        self.is(None)
    }

    /// Unwrap the [Option] value, panic for [None].
    ///
    /// ```
    /// # use assert4rs::Assert;
    /// Assert::that(Some(2)).unwrap().is(2);
    /// ```
    ///
    /// ```should_panic
    /// # use assert4rs::Assert;
    /// Assert::that(None::<i32>).unwrap();
    /// ```
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
