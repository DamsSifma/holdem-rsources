/// Default tolerance for percentage calculations (1%)
pub const TOLERANCE: f64 = 1.0;

/// Default strict tolerance for percentage calculations (0.1%)
pub const STRICT_TOLERANCE: f64 = 0.1;

#[macro_export]
macro_rules! assert_within_tolerance {
    ($actual:expr, $expected:expr, $tolerance:expr) => {{
        let actual_val = $actual;
        let expected_val = $expected;
        let tolerance_val = $tolerance;
        let min = expected_val - tolerance_val;
        let max = expected_val + tolerance_val;
        assert!(
            actual_val >= min && actual_val <= max,
            "assertion failed: `{}` is within tolerance\n  actual: {:.2}%\n  expected: {:.2}% ±{:.2}%\n  range: [{:.2}%, {:.2}%]",
            stringify!($actual),
            actual_val,
            expected_val,
            tolerance_val,
            min,
            max
        );
    }};
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assert_within_tolerance_pass() {
        assert_within_tolerance!(50.0, 50.0, 1.0);
        assert_within_tolerance!(50.5, 50.0, 1.0);
        assert_within_tolerance!(49.5, 50.0, 1.0);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_assert_within_tolerance_fail() {
        assert_within_tolerance!(52.0, 50.0, 1.0);
    }
}
