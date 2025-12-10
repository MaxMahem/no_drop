/// Test macro for guard state verification.
/// Matches on `armed` or `disarmed` keywords to handle cleanup automatically.
macro_rules! ctor {
    // Constructed armed state - but does not panic
    ($test_name:ident, $ctor:expr, ($($params:tt)*), armed_no_panic) => {
        #[test]
        fn $test_name() {
            let guard = $ctor($($params)*);
            assert!(guard.armed());
            assert!(!guard.disarmed());
        }
    };

    // Constructed disarmed state
    ($test_name:ident, $ctor:expr, ($($params:tt)*), disarmed) => {
        #[test]
        fn $test_name() {
            let guard = $ctor($($params)*);
            assert!(guard.disarmed());
            assert!(!guard.armed());
        }
    };

    // Constructed armed - panics
    ($test_name:ident, $ctor:expr, ($($params:tt)*), armed, $panic_msg:literal) => {
        #[test]
        #[should_panic(expected = $panic_msg)]
        fn $test_name() {
            let guard = $ctor($($params)*);
            drop(guard); // Will panic with expected message
        }
    };
}

/// Test macro for state transitions (arm/disarm).
/// Matches on final `armed` or `disarmed` state to handle cleanup.
macro_rules! transition {
    ($test_name:ident, $ctor:expr, ($($params:tt)*), $method:ident, $expected:expr, armed_no_panic) => {
        #[test]
        fn $test_name() {
            let mut guard = $ctor($($params)*);
            let changed = guard.$method();
            assert_eq!(changed, $expected);
            assert!(guard.armed());
        }
    };
    // Transition ending in armed state - auto-disarms
    ($test_name:ident, $ctor:expr, ($($params:tt)*), $method:ident, $expected:expr, armed, $panic_msg:literal) => {
        #[test]
        #[should_panic(expected = $panic_msg)]
        fn $test_name() {
            let mut guard = $ctor($($params)*);
            let changed = guard.$method();
            assert_eq!(changed, $expected);
            assert!(guard.armed());
        }
    };

    // Transition ending in disarmed state - safe to drop
    ($test_name:ident, $ctor:expr, ($($params:tt)*), $method:ident, $expected:expr, disarmed) => {
        #[test]
        fn $test_name() {
            let mut guard = $ctor($($params)*);
            let changed = guard.$method();
            assert_eq!(changed, $expected);
            assert!(guard.disarmed());
        }
    };
}

/// Test macro for `TryFrom` implementations.
/// Tests both success and error branches.
macro_rules! try_from {
    // Test successful conversion from armed guard
    ($test_name_ok:ident, $from_ctor:expr, ($($params:tt)*), $target_type:ty, armed) => {
        #[test]
        fn $test_name_ok() {
            let guard = $from_ctor($($params)*);
            let result: Result<$target_type, _> = guard.try_into();
            assert!(result.is_ok());
            // Forget the converted value to prevent panic on drop
            result.unwrap().forget();
        }
    };

    // Test failed conversion from disarmed guard
    ($test_name_err:ident, $from_ctor:expr, ($($params:tt)*), $target_type:ty, disarmed) => {
        #[test]
        fn $test_name_err() {
            let guard = $from_ctor($($params)*);
            let result: Result<$target_type, _> = guard.try_into();
            assert!(result.is_err());
        }
    };
}

pub(crate) use ctor;
pub(crate) use transition;
pub(crate) use try_from;
