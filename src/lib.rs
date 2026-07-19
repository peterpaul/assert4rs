//! Fluent assertions for Rust.
//!
//! This crate provides a fluent API for writing assertions in tests.
//! All assertions start with [`Assert::that`] and can be chained:
//!
//! ```
//! use assert4rs::Assert;
//!
//! Assert::that("foo")
//!     .is("foo")
//!     .is_not("bar");
//! ```
//!
//! Type-specific assertions are available for [`Option`], [`Result`],
//! and [`Vec`]:
//!
//! ```
//! use assert4rs::Assert;
//!
//! Assert::that(Some(42)).unwrap().is_gt(0);
//! Assert::that(vec![1, 2, 3]).contains(&2);
//! ```
//!
//! Values can be transformed with [`Assert::map`] to apply further
//! assertions:
//!
//! ```
//! use assert4rs::Assert;
//!
//! Assert::that("3")
//!     .map(|v| v.parse::<i32>().unwrap())
//!     .is(3);
//! ```

mod diff;
pub mod equals;
pub mod option;
pub mod result;
pub mod string;
pub mod vec;

/// Entry point for the [Assert] DSL.
///
/// [Assert] provides a fluent API for assertions. An [Assert] holds
/// the `actual` value that can be used in assertion statements.
///
/// ```
/// # use assert4rs::Assert;
/// Assert::that(3).is(3);
/// ```
///
/// It takes ownership of self and returns ownership back so that
/// assertions can be chained in fluent manner.
///
/// Like in the following example:
///
/// ```
/// # use assert4rs::Assert;
/// Assert::that("foo")
///     .is("foo")
///     .is_not("bar");
/// ```
pub struct Assert<T> {
    actual: T,
    label: Option<String>,
}

impl<T> Assert<T> {
    /// Create an [Assert] instance for the `actual` value.
    ///
    /// All assertions start with `Assert::that(actual)`.
    pub fn that(actual: T) -> Self {
        Assert {
            actual,
            label: None,
        }
    }

    /// Maps the `actual` value using lambda `f`.
    ///
    /// This method is intended to map the actual value in order to
    /// apply further assertions.
    ///
    /// ```
    /// # use assert4rs::Assert;
    /// Assert::that("3")
    ///     .map(|v| v.parse::<i32>().unwrap())
    ///     .is(3);
    /// ```
    ///
    /// ```should_panic
    /// # use assert4rs::Assert;
    /// Assert::that(2).map(|v| v + 2).is(3);
    /// ```
    pub fn map<R>(self, f: impl FnOnce(T) -> R) -> Assert<R> {
        Assert::that(f(self.actual))
    }

    /// Attach a label to this assertion, used in the panic message if
    /// the assertion fails. Useful to identify which value failed when
    /// asserting on several unrelated values in the same test.
    ///
    /// ```
    /// # use assert4rs::Assert;
    /// Assert::that(30).named("user.age").is(30);
    /// ```
    ///
    /// ```should_panic
    /// # use assert4rs::Assert;
    /// Assert::that(25).named("user.age").is(30);
    /// ```
    pub fn named(mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self
    }

    pub(crate) fn header(&self, assertion: &str) -> String {
        match &self.label {
            Some(label) => format!("Assertion failed for `{label}`: `({assertion})`"),
            None => format!("Assertion failed: `({assertion})`"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_without_label() {
        let a = Assert::that(1);
        assert_eq!(a.header("a == b"), "Assertion failed: `(a == b)`");
    }

    #[test]
    fn header_with_label() {
        let a = Assert::that(1).named("x");
        assert_eq!(a.header("a == b"), "Assertion failed for `x`: `(a == b)`");
    }

    #[test]
    fn map_resets_label() {
        let a = Assert::that(1).named("x").map(|v| v + 1);
        assert_eq!(a.header("a == b"), "Assertion failed: `(a == b)`");
    }
}
