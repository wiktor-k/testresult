#![doc = include_str!("../README.md")]
#![deny(missing_debug_implementations)]
//#![deny(missing_docs)]

/// Error with stacktrace
///
/// Any other type of error can be converted to this one but the
/// conversion will always panic.
///
/// This type is useful only in unit tests.
#[derive(Debug)]
#[doc(hidden)]
pub struct ErrorWithStacktrace;

impl<T: std::error::Error> From<T> for ErrorWithStacktrace {
    fn from(error: T) -> Self {
        panic!("Error: {}", error);
    }
}

impl std::fmt::Display for ErrorWithStacktrace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "This is an error with stacktrace")
    }
}

/// Unit test result
///
/// This type allows panicking when encountering any type of
/// failure. Thus it allows using `?` operator in unit tests but still
/// get the complete stacktrace and exact place of failure during
/// tests.
///
/// # Examples
///
/// ```
/// use testresult::TestResult;
///
/// #[test]
/// fn it_works() -> TestResult {
///   // ...
///    std::fs::File::open("this-file-does-not-exist")?;
///    // ...
///    Ok(())
/// }
/// ```

pub type TestResult = std::result::Result<(), ErrorWithStacktrace>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_wo2rks() -> TestResult {
        let result = 2 + 2;
        assert_eq!(result, 4);
        Ok(())
    }

    #[test]
    #[ignore] // ignored test must still compile
              // this checks whether conversion from all errors is accomplished
    fn it_works_too() -> TestResult {
        std::fs::File::open("this-file-does-not-exist")?;
        Ok(())
    }
}
