use crate::Assert;

use std::fmt::Debug;

impl<T> Assert<Option<T>> {
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
    #[track_caller]
    pub fn is_some(self, expected: T) -> Self
    where
        T: PartialEq + Debug,
    {
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
    #[track_caller]
    pub fn is_none(self) -> Self
    where
        T: PartialEq + Debug,
    {
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
    #[track_caller]
    pub fn unwrap(self) -> Assert<T> {
        match self.actual {
            Some(value) => Assert::that(value),
            None => panic!("{}\n  Actual:   `None`", self.header("actual.is_some()")),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Assert;

    #[test]
    #[should_panic(expected = "Assertion failed for `x`: `(actual.is_some())`")]
    fn unwrap_reports_label_when_named() {
        Assert::that(None::<i32>).named("x").unwrap();
    }

    #[test]
    #[should_panic(
        expected = "Assertion failed for `x`: `(actual == expected)`\n  Actual:   `None`\n  Expected: `Some(2)`"
    )]
    fn is_some_reports_label_and_diff_via_delegation() {
        Assert::that(None::<i32>).named("x").is_some(2);
    }

    #[test]
    #[should_panic(
        expected = "Assertion failed for `y`: `(actual == expected)`\n  Actual:   `Some(2)`\n  Expected: `None`"
    )]
    fn is_none_reports_label_and_diff_via_delegation() {
        Assert::that(Some(2)).named("y").is_none();
    }
}
