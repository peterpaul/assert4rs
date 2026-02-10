use crate::Assert;
use std::fmt::Debug;

impl<T> Assert<T>
where
    T: Debug,
{
    /// Assert that `self` is equal to the `expected` value.
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
}

#[cfg(test)]
mod test {
    use crate::Assert;

    #[test]
    fn ord_ge_success() {
        Assert::that(3).is_ge(3);
        Assert::that(3).is_ge(2);
    }

    #[test]
    #[should_panic(expected = "Assertion failed: `(actual >= other)`
  Actual:   `3`
  Other:    `4`")]
    fn ord_ge_fail() {
        Assert::that(3).is_ge(4);
    }

    #[test]
    fn ord_le_success() {
        Assert::that(3).is_le(3);
        Assert::that(3).is_le(4);
    }

    #[test]
    #[should_panic(expected = "Assertion failed: `(actual <= other)`
  Actual:   `3`
  Other:    `2`")]
    fn ord_le_fail() {
        Assert::that(3).is_le(2);
    }

    #[test]
    fn ord_gt_success() {
        Assert::that(3).is_gt(2);
    }

    #[test]
    #[should_panic(expected = "Assertion failed: `(actual > other)`
  Actual:   `3`
  Other:    `3`")]
    fn ord_gt_fail_eq() {
        Assert::that(3).is_gt(3);
    }

    #[test]
    #[should_panic(expected = "Assertion failed: `(actual > other)`
  Actual:   `3`
  Other:    `4`")]
    fn ord_gt_fail_larger() {
        Assert::that(3).is_gt(4);
    }

    #[test]
    fn ord_lt_success() {
        Assert::that(3).is_lt(4);
    }

    #[test]
    #[should_panic(expected = "Assertion failed: `(actual < other)`
  Actual:   `3`
  Other:    `2`")]
    fn ord_lt_fail_smaller() {
        Assert::that(3).is_lt(2);
    }

    #[test]
    #[should_panic(expected = "Assertion failed: `(actual < other)`
  Actual:   `3`
  Other:    `3`")]
    fn ord_lt_fail_eq() {
        Assert::that(3).is_lt(3);
    }
}
