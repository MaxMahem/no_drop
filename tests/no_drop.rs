mod dbg {

    use no_drop::dbg::*;

    #[test]
    fn consume_returns_value() {
        let value = NoDrop::wrap(42);
        assert_eq!(value.unwrap(), 42);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic(expected = "Value was dropped without being unwrapped")]
    fn panics_on_drop_in_debug() {
        let _value = NoDrop::wrap(42);
        // Should panic in debug mode
    }

    #[test]
    #[cfg(not(debug_assertions))]
    fn does_not_panic_in_release() {
        let _value = NoDrop::new(42);
        // Should not panic in release mode
    }

    #[test]
    fn no_drop_unit() {
        let value = NoDrop::new();
        value.forget();
    }
}

mod rls {
    use no_drop::rls::*;

    #[test]
    fn consume_returns_value() {
        let value = NoDrop::wrap(42);
        assert_eq!(value.unwrap(), 42);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic(expected = "Value was dropped without being unwrapped")]
    fn panics_on_drop_in_debug() {
        let _value = NoDrop::wrap(42);
        // Should panic in debug mode
    }

    #[test]
    #[cfg(not(debug_assertions))]
    #[should_panic(expected = "Value was dropped without being unwrapped")]
    fn panics_on_drop_in_release() {
        let _value = NoDrop::new(42);
        // Should panic in release mode
    }

    #[test]
    fn no_drop_unit() {
        let value = NoDrop::new();
        value.forget();
    }
}
