/// Assert that a `Result` is an `Err` whose `Debug` representation contains `substring`.
///
/// Useful for checking that contract invocations panic with the expected error.
pub fn assert_err_contains<T: std::fmt::Debug, E: std::fmt::Debug>(
    result: Result<T, E>,
    substring: &str,
) {
    match result {
        Err(e) => {
            let msg = format!("{e:?}");
            assert!(
                msg.contains(substring),
                "Expected error containing {:?} but got: {:?}",
                substring,
                msg
            );
        }
        Ok(v) => {
            panic!(
                "Expected an error containing {:?} but the call succeeded with: {:?}",
                substring, v
            );
        }
    }
}

/// Assert that two values are equal, printing both on mismatch.
#[macro_export]
macro_rules! assert_eq_val {
    ($left:expr, $right:expr) => {
        assert_eq!(
            $left, $right,
            "assertion failed: left = {:?}, right = {:?}",
            $left, $right
        );
    };
}
