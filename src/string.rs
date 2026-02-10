use crate::Assert;

/// DSL for [String].
impl Assert<String> {
    /// Assert that the actual string starts with the given prefix.
    ///
    /// ```
    /// # use assert4rs::Assert;
    /// Assert::that(String::from("hello world")).starts_with("hello");
    /// ```
    ///
    /// ```should_panic
    /// # use assert4rs::Assert;
    /// Assert::that(String::from("hello world")).starts_with("world");
    /// ```
    pub fn starts_with(self, prefix: &str) -> Self {
        assert!(
            self.actual.starts_with(prefix),
            "Assertion failed: `(actual.starts_with(prefix))`
  Actual: `{:?}`
  Prefix: `{:?}`",
            self.actual,
            prefix,
        );
        self
    }

    /// Assert that the actual string ends with the given suffix.
    ///
    /// ```
    /// # use assert4rs::Assert;
    /// Assert::that(String::from("hello world")).ends_with("world");
    /// ```
    ///
    /// ```should_panic
    /// # use assert4rs::Assert;
    /// Assert::that(String::from("hello world")).ends_with("hello");
    /// ```
    pub fn ends_with(self, suffix: &str) -> Self {
        assert!(
            self.actual.ends_with(suffix),
            "Assertion failed: `(actual.ends_with(suffix))`
  Actual: `{:?}`
  Suffix: `{:?}`",
            self.actual,
            suffix,
        );
        self
    }

    /// Assert that the actual string contains the given pattern.
    ///
    /// ```
    /// # use assert4rs::Assert;
    /// Assert::that(String::from("hello world")).contains("lo wo");
    /// ```
    ///
    /// ```should_panic
    /// # use assert4rs::Assert;
    /// Assert::that(String::from("hello world")).contains("xyz");
    /// ```
    pub fn contains(self, pattern: &str) -> Self {
        assert!(
            self.actual.contains(pattern),
            "Assertion failed: `(actual.contains(pattern))`
  Actual:  `{:?}`
  Pattern: `{:?}`",
            self.actual,
            pattern,
        );
        self
    }
}
