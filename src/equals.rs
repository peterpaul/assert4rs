use crate::Assert;
use std::fmt::Debug;

impl<T> Assert<T>
where
    T: Debug,
{
    /// Assert that `self` is equal to the `expected` value.
    ///
    /// ```
    /// # use assert4rs::Assert;
    /// Assert::that(2).is(2);
    /// Assert::that(String::from("2")).is(String::from("2"));
    /// Assert::that(String::from("2")).is("2");
    /// ```
    ///
    /// ```should_panic
    /// # use assert4rs::Assert;
    /// Assert::that(2).is(3);
    /// ```
    pub fn is<R>(self, expected: R) -> Self
    where
        T: PartialEq<R>,
        R: Debug,
    {
        assert!(
            self.actual == expected,
            "Assertion failed: `(actual == expected)`
  Actual:   `{:?}`
  Expected: `{:?}`",
            self.actual,
            expected,
        );
        self
    }

    /// Assert that `self` is not equal to the `other` value.
    ///
    /// ```
    /// # use assert4rs::Assert;
    /// Assert::that(2).is_not(3);
    /// ```
    ///
    /// ```should_panic
    /// # use assert4rs::Assert;
    /// Assert::that(2).is_not(2);
    /// ```
    pub fn is_not<R>(self, other: R) -> Self
    where
        T: PartialEq<R>,
        R: Debug,
    {
        assert!(
            self.actual != other,
            "Assertion failed: `(actual != other)`
  Actual:   `{:?}`
  Other:    `{:?}`",
            self.actual,
            other,
        );
        self
    }

    /// Assert that `self` is greater than the `other` value.
    ///
    /// ```
    /// # use assert4rs::Assert;
    /// Assert::that(3).is_gt(2);
    /// ```
    ///
    /// ```should_panic
    /// # use assert4rs::Assert;
    /// Assert::that(3).is_gt(3);
    /// ```
    ///
    /// ```should_panic
    /// # use assert4rs::Assert;
    /// Assert::that(3).is_gt(4);
    /// ```
    pub fn is_gt<R>(self, other: R) -> Self
    where
        T: PartialOrd<R>,
        R: Debug,
    {
        assert!(
            self.actual > other,
            "Assertion failed: `(actual > other)`
  Actual:   `{:?}`
  Other:    `{:?}`",
            self.actual,
            other,
        );
        self
    }

    /// Assert that `self` is greater than or equal to the `other` value.
    ///
    /// ```
    /// # use assert4rs::Assert;
    /// Assert::that(3).is_ge(3);
    /// Assert::that(3).is_ge(2);
    /// ```
    ///
    /// ```should_panic
    /// # use assert4rs::Assert;
    /// Assert::that(3).is_ge(4);
    /// ```
    pub fn is_ge<R>(self, other: R) -> Self
    where
        T: PartialOrd<R>,
        R: Debug,
    {
        assert!(
            self.actual >= other,
            "Assertion failed: `(actual >= other)`
  Actual:   `{:?}`
  Other:    `{:?}`",
            self.actual,
            other,
        );
        self
    }

    /// Assert that `self` is less than the `other` value.
    ///
    /// ```
    /// # use assert4rs::Assert;
    /// Assert::that(3).is_lt(4);
    /// ```
    ///
    /// ```should_panic
    /// # use assert4rs::Assert;
    /// Assert::that(3).is_lt(2);
    /// ```
    ///
    /// ```should_panic
    /// # use assert4rs::Assert;
    /// Assert::that(3).is_lt(3);
    /// ```
    pub fn is_lt<R>(self, other: R) -> Self
    where
        T: PartialOrd<R>,
        R: Debug,
    {
        assert!(
            self.actual < other,
            "Assertion failed: `(actual < other)`
  Actual:   `{:?}`
  Other:    `{:?}`",
            self.actual,
            other,
        );
        self
    }

    /// Assert that `self` is less than or equal to the `other` value.
    ///
    /// ```
    /// # use assert4rs::Assert;
    /// Assert::that(3).is_le(3);
    /// Assert::that(3).is_le(4);
    /// ```
    ///
    /// ```should_panic
    /// # use assert4rs::Assert;
    /// Assert::that(3).is_le(2);
    /// ```
    pub fn is_le<R>(self, other: R) -> Self
    where
        T: PartialOrd<R>,
        R: Debug,
    {
        assert!(
            self.actual <= other,
            "Assertion failed: `(actual <= other)`
  Actual:   `{:?}`
  Other:    `{:?}`",
            self.actual,
            other,
        );
        self
    }

    /// Assert that the actual value satisfies the given predicate.
    ///
    /// ```
    /// # use assert4rs::Assert;
    /// Assert::that(4).satisfies(|v| v % 2 == 0);
    /// ```
    ///
    /// ```should_panic
    /// # use assert4rs::Assert;
    /// Assert::that(3).satisfies(|v| v % 2 == 0);
    /// ```
    pub fn satisfies(self, predicate: impl FnOnce(&T) -> bool) -> Self {
        assert!(
            predicate(&self.actual),
            "Assertion failed: `(satisfies predicate)`
  Actual: `{:?}`",
            self.actual,
        );
        self
    }
}
