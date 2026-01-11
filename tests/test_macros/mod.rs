/// Test macro for guard state verification.
/// Matches on `armed` or `disarmed` keywords to handle cleanup automatically.
#[allow(unused_macros)]
macro_rules! guard_ctor {
    // Constructed armed state - but does not panic
    ($test_name:ident, $ctor_expr:expr, armed_no_panic) => {
        #[test]
        fn $test_name() {
            let guard = $ctor_expr;
            assert!(guard.armed());
            assert!(!guard.disarmed());
        }
    };

    // Constructed disarmed state
    ($test_name:ident, $ctor_expr:expr, disarmed) => {
        #[test]
        fn $test_name() {
            let guard = $ctor_expr;
            assert!(guard.disarmed());
            assert!(!guard.armed());
        }
    };

    // Constructed armed - panics
    ($test_name:ident, $ctor_expr:expr, armed, $panic_msg:literal) => {
        #[test]
        #[should_panic(expected = $panic_msg)]
        fn $test_name() {
            let guard = $ctor_expr;
            drop(guard); // Will panic with expected message
        }
    };

    ($test_name:ident, $ctor_expr:expr, armed) => {
        #[test]
        #[should_panic(expected = "Value was dropped without being unwrapped")]
        fn $test_name() {
            let guard = $ctor_expr;
            drop(guard); // Will panic with expected message
        }
    };
}

/// Test macro for state transitions (arm/disarm/toggle).
/// Matches on final `armed` or `disarmed` state to handle cleanup.
#[allow(unused_macros)]
macro_rules! transition {
    ($test_name:ident, $ctor_expr:expr, $method:ident, $expected:expr, armed_no_panic) => {
        #[test]
        fn $test_name() {
            let mut guard = $ctor_expr;
            let changed = guard.$method();
            assert_eq!(changed, $expected);
            assert!(guard.armed());
            assert!(!guard.disarmed());
        }
    };
    // Transition ending in armed state - auto-disarms
    ($test_name:ident, $ctor_expr:expr, $method:ident, $expected:expr, armed, $panic_msg:literal) => {
        #[test]
        #[should_panic(expected = $panic_msg)]
        fn $test_name() {
            let mut guard = $ctor_expr;
            let changed = guard.$method();
            assert_eq!(changed, $expected);
            assert!(guard.armed());
            assert!(!guard.disarmed());
        }
    };

    ($test_name:ident, $ctor_expr:expr, $method:ident, $expected:expr, armed) => {
        #[test]
        #[should_panic(expected = "Value was dropped without being unwrapped")]
        fn $test_name() {
            let mut guard = $ctor_expr;
            let changed = guard.$method();
            assert_eq!(changed, $expected);
            assert!(guard.armed());
            assert!(!guard.disarmed());
        }
    };

    // Transition ending in disarmed state - safe to drop
    ($test_name:ident, $ctor_expr:expr, $method:ident, $expected:expr, disarmed) => {
        #[test]
        fn $test_name() {
            let mut guard = $ctor_expr;
            let changed = guard.$method();
            assert_eq!(changed, $expected);
            assert!(guard.disarmed());
            assert!(!guard.armed());
        }
    };
}

/// Test macro for `TryFrom` implementations.
/// Tests both success and error branches.
#[allow(unused_macros)]
macro_rules! try_from {
    // Test successful conversion from armed guard
    ($test_name_ok:ident, $ctor_expr:expr, $target_type:ty, armed) => {
        #[test]
        fn $test_name_ok() {
            let guard = $ctor_expr;
            let result: Result<$target_type, _> = guard.try_into();
            assert!(result.is_ok());
            // Forget the converted value to prevent panic on drop
            result.unwrap().forget();
        }
    };

    // Test failed conversion from disarmed guard
    ($test_name_err:ident, $ctor_expr:expr, $target_type:ty, disarmed) => {
        #[test]
        fn $test_name_err() {
            let guard = $ctor_expr;
            let result: Result<$target_type, _> = guard.try_into();
            assert!(result.is_err());
        }
    };
}

/// Test macro for `set_msg` method.
/// Tests that setting a new message works correctly.
#[allow(unused_macros)]
macro_rules! set_msg {
    // For armed guards that panic on drop - verify the new message is used
    ($test_name:ident, $ctor_expr:expr, $new_msg:literal) => {
        #[test]
        #[should_panic(expected = $new_msg)]
        fn $test_name() {
            let guard = $ctor_expr;
            let guard = guard.set_msg($new_msg);
            drop(guard); // Will panic with new message
        }
    };

    // For no-op guards - just verify it can be called without error
    ($test_name:ident, $ctor_expr:expr, $new_msg:literal, no_panic) => {
        #[test]
        fn $test_name() {
            let guard = $ctor_expr;
            let _guard = guard.set_msg($new_msg);
            // Just calling set_msg and dropping without panic
        }
    };
}

/// Test macro for constructors that take a single value parameter.
/// Pattern: test_ctor!(test_name, constructor, (params), expected);
#[allow(unused_macros)]
macro_rules! test_ctor {
    ($test_name:ident, $ctor:expr, ($($params:tt)*), $expected:expr) => {
        #[test]
        fn $test_name() {
            let wrapper = $ctor($($params)*);
            assert_eq!(wrapper.unwrap(), $expected);
        }
    };
}

/// Test macro for the forget method.
/// Pattern: test_forget!(test_name, constructor, (params));
#[allow(unused_macros)]
macro_rules! test_forget {
    ($test_name:ident, $ctor:expr, ($($params:tt)*)) => {
        #[test]
        fn $test_name() {
            let wrapper = $ctor($($params)*);
            wrapper.forget();
        }
    };
}

/// Test macro for the clone method.
/// Pattern: test_clone!(test_name, type);
#[allow(unused_macros)]
macro_rules! test_clone {
    ($test_name:ident, $type:ty, $ctor:expr, ($($params:tt)*)) => {
        #[test]
        fn $test_name() {
            let wrapper = $ctor($($params)*);
            let clone = <$type>::clone(&wrapper);
            wrapper.forget();
            clone.forget();
        }
    };
}

#[allow(unused_imports)]
pub(crate) use guard_ctor;
#[allow(unused_imports)]
pub(crate) use set_msg;
#[allow(unused_imports)]
pub(crate) use test_clone;
#[allow(unused_imports)]
pub(crate) use test_ctor;
#[allow(unused_imports)]
pub(crate) use test_forget;
#[allow(unused_imports)]
pub(crate) use transition;
#[allow(unused_imports)]
pub(crate) use try_from;
