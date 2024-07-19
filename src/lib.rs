#![doc = include_str!("../README.md")]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
// the docs illustrate the usage in test functions
#![allow(clippy::test_attr_in_doctest)]

/// Error, but one which immediately panics with a stacktrace.
///
/// Usually used via [`TestResult`].
///
/// Any other type of error can be converted to this one but the
/// conversion will always panic.
///
/// This type is useful only in unit tests.
/// It cannot be instantiated: no values of this type can ever exist.
#[derive(Debug)]
pub enum TestError {}

impl<T: std::fmt::Display> From<T> for TestError {
    #[track_caller] // Will show the location of the caller in test failure messages
    fn from(error: T) -> Self {
        // Use alternate format for rich error message for anyhow
        // See: https://docs.rs/anyhow/latest/anyhow/struct.Error.html#display-representations
        panic!("error: {} - {:#}", std::any::type_name::<T>(), error);
    }
}

/// Unit test result - always panics when an error occurs.
///
/// This type allows panicking when encountering any type of
/// failure. Thus it allows using the `?` operator in unit tests but still
/// getting the complete stacktrace and the exact place of a failure during
/// tests.
///
/// # Examples
///
/// Using [`TestResult`] as a result of a test function:
///
/// ```
/// use testresult::TestResult;
///
/// #[test]
/// fn it_works() -> TestResult {
///     // ...
///     std::fs::File::open("this-file-does-not-exist")?;
///     // ...
///     Ok(())
/// }
/// ```
///
/// As [`TestResult`] is generic one can use it in test helper functions to return
/// objects to test functions.
///
/// For example [`TestResult`] used in `rstest` fixture returns a [`std::fs::File`] object that
/// can be used by the test:
///
/// ```
/// use std::fs::File;
///
/// use rstest::{fixture, rstest};
/// use testresult::TestResult;
///
/// #[fixture]
/// fn a_file() -> TestResult<File> {
///     let file = File::open("this-file-does-not-exist")?;
///     // ...
///     Ok(file)
/// }
///
/// #[rstest]
/// fn it_works(file: File) -> TestResult {
///     // ...
///     Ok(())
/// }
/// ```
pub type TestResult<T = ()> = std::result::Result<T, TestError>;

#[cfg(test)]
mod tests {
    use anyhow::Context as _;

    use super::*;

    #[test]
    #[ignore] // ignored test must still compile
              // this checks whether conversion from all errors is accomplished
    fn compilation_works() -> TestResult {
        std::fs::File::open("this-file-does-not-exist")?;
        Ok(())
    }

    // helper function which always fails
    fn test_fn() -> TestResult<String> {
        let string = String::from_utf8(vec![0, 159, 146, 150])?;
        Ok(string)
    }

    #[test]
    fn check_if_panics() -> TestResult {
        let result = std::panic::catch_unwind(|| {
            let _ = test_fn();
        });
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert_eq!(
            Some(
                &"error: alloc::string::FromUtf8Error - invalid utf-8 sequence of 1 bytes from index 1"
                    .to_string()
            ),
            err.downcast_ref::<String>()
        );
        Ok(())
    }

    fn anyhow_a() -> anyhow::Result<String> {
        let string = String::from_utf8(vec![0, 159, 146, 150])?;
        Ok(string)
    }

    fn anyhow_b() -> anyhow::Result<String> {
        let file = anyhow_a().context("Parsing a string")?;
        Ok(file)
    }

    fn anyhow_c() -> TestResult<String> {
        let file = anyhow_b()?;
        Ok(file)
    }

    #[test]
    fn check_if_anyhow_panics() -> TestResult {
        let result = std::panic::catch_unwind(|| {
            let _ = anyhow_c();
        });
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert_eq!(
            Some(
                &"error: anyhow::Error - Parsing a string: invalid utf-8 sequence of 1 bytes from index 1"
                    .to_string()
            ),
            err.downcast_ref::<String>()
        );
        Ok(())
    }
}
