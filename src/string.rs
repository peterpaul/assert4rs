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
    #[track_caller]
    pub fn starts_with(self, prefix: &str) -> Self {
        assert!(
            self.actual.starts_with(prefix),
            "{}\n  Actual: `{:?}`\n  Prefix: `{:?}`",
            self.header("actual.starts_with(prefix)"),
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
    #[track_caller]
    pub fn ends_with(self, suffix: &str) -> Self {
        assert!(
            self.actual.ends_with(suffix),
            "{}\n  Actual: `{:?}`\n  Suffix: `{:?}`",
            self.header("actual.ends_with(suffix)"),
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
    #[track_caller]
    pub fn contains(self, pattern: &str) -> Self {
        assert!(
            self.actual.contains(pattern),
            "{}\n  Actual:  `{:?}`\n  Pattern: `{:?}`",
            self.header("actual.contains(pattern)"),
            self.actual,
            pattern,
        );
        self
    }
}

/// DSL for `&String`, mirroring [`Assert<String>`] for read-only
/// assertions that don't need ownership of the string — avoids a
/// `.clone()` at the call site when the string is used again afterward.
impl Assert<&String> {
    /// Assert that the actual string starts with the given prefix.
    ///
    /// ```
    /// # use assert4rs::Assert;
    /// let s = String::from("hello world");
    /// Assert::that(&s).starts_with("hello");
    /// ```
    ///
    /// ```should_panic
    /// # use assert4rs::Assert;
    /// let s = String::from("hello world");
    /// Assert::that(&s).starts_with("world");
    /// ```
    #[track_caller]
    pub fn starts_with(self, prefix: &str) -> Self {
        assert!(
            self.actual.starts_with(prefix),
            "{}\n  Actual: `{:?}`\n  Prefix: `{:?}`",
            self.header("actual.starts_with(prefix)"),
            self.actual,
            prefix,
        );
        self
    }

    /// Assert that the actual string ends with the given suffix.
    ///
    /// ```
    /// # use assert4rs::Assert;
    /// let s = String::from("hello world");
    /// Assert::that(&s).ends_with("world");
    /// ```
    ///
    /// ```should_panic
    /// # use assert4rs::Assert;
    /// let s = String::from("hello world");
    /// Assert::that(&s).ends_with("hello");
    /// ```
    #[track_caller]
    pub fn ends_with(self, suffix: &str) -> Self {
        assert!(
            self.actual.ends_with(suffix),
            "{}\n  Actual: `{:?}`\n  Suffix: `{:?}`",
            self.header("actual.ends_with(suffix)"),
            self.actual,
            suffix,
        );
        self
    }

    /// Assert that the actual string contains the given pattern.
    ///
    /// ```
    /// # use assert4rs::Assert;
    /// let s = String::from("hello world");
    /// Assert::that(&s).contains("lo wo");
    /// ```
    ///
    /// ```should_panic
    /// # use assert4rs::Assert;
    /// let s = String::from("hello world");
    /// Assert::that(&s).contains("xyz");
    /// ```
    #[track_caller]
    pub fn contains(self, pattern: &str) -> Self {
        assert!(
            self.actual.contains(pattern),
            "{}\n  Actual:  `{:?}`\n  Pattern: `{:?}`",
            self.header("actual.contains(pattern)"),
            self.actual,
            pattern,
        );
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::Assert;

    #[test]
    #[should_panic(expected = "Assertion failed for `x`: `(actual.contains(pattern))`")]
    fn contains_reports_label_when_named() {
        Assert::that(String::from("hello"))
            .named("x")
            .contains("xyz");
    }

    #[test]
    fn ref_starts_with_leaves_string_usable_afterward() {
        let s = String::from("hello world");
        Assert::that(&s).starts_with("hello");
        assert_eq!(s, "hello world");
    }

    #[test]
    #[should_panic(expected = "Assertion failed for `x`: `(actual.contains(pattern))`")]
    fn ref_contains_reports_label_when_named() {
        let s = String::from("hello");
        Assert::that(&s).named("x").contains("xyz");
    }

    #[test]
    #[should_panic(
        expected = "Assertion failed for `x`: `(actual.ends_with(suffix))`\n  Actual: `\"hello world\"`\n  Suffix: `\"hello\"`"
    )]
    fn ref_ends_with_reports_full_message() {
        let s = String::from("hello world");
        Assert::that(&s).named("x").ends_with("hello");
    }

    #[test]
    #[should_panic(
        expected = "Assertion failed for `x`: `(actual.starts_with(prefix))`\n  Actual: `\"hello world\"`\n  Prefix: `\"world\"`"
    )]
    fn starts_with_reports_full_message() {
        Assert::that(String::from("hello world"))
            .named("x")
            .starts_with("world");
    }

    #[test]
    #[should_panic(
        expected = "Assertion failed for `x`: `(actual.ends_with(suffix))`\n  Actual: `\"hello world\"`\n  Suffix: `\"hello\"`"
    )]
    fn ends_with_reports_full_message() {
        Assert::that(String::from("hello world"))
            .named("x")
            .ends_with("hello");
    }
}
