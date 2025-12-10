/// Test macro for constructors that take a single value parameter.
/// Pattern: test_ctor!(test_name, constructor, (params), expected);
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

pub(crate) use test_clone;
pub(crate) use test_ctor;
pub(crate) use test_forget;
