/// Test macro for constructors that take a single value parameter.
///
/// # Examples
///
/// ```ignore
/// test_ctor!(no_drop, NoDrop::wrap, (42), 42);
/// test_ctor!(into_no_drop_dbg_trait, IntoNoDropDbg::no_drop, (42), 42);
/// ```
#[macro_export]
macro_rules! test_ctor {
    ($test_name:ident, $ctor:expr, ($($params:tt)*), $expected:expr) => {
        #[test]
        fn $test_name() {
            let wrapper = $ctor($($params)*);
            assert_eq!(wrapper.consume(), $expected);
        }
    };
}

/// Test macro for the forget method.
#[macro_export]
macro_rules! test_forget {
    ($test_name:ident, $ctor:expr, ($($params:tt)*)) => {
        #[test]
        fn $test_name() {
            let wrapper = $ctor($($params)*);
            wrapper.forget();
        }
    };
}
