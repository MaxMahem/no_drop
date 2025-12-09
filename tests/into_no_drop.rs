mod dbg {
    use no_drop::dbg::{Consume, IntoNoDrop};

    #[test]
    fn dbg_into_no_drop() {
        let wrapper = 42.no_drop();
        assert_eq!(wrapper.consume(), 42);
    }
}

mod rls_tests {
    use no_drop::rls::{Consume, IntoNoDrop};

    #[test]
    fn rls_into_no_drop() {
        let wrapper = 42.no_drop();
        assert_eq!(wrapper.consume(), 42);
    }
}
