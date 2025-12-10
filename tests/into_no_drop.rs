mod dbg {
    use no_drop::dbg::IntoNoDrop;

    #[test]
    fn dbg_into_no_drop() {
        let wrapper = 42.no_drop();
        assert_eq!(wrapper.consume(), 42);
    }
}

mod rls_tests {
    use no_drop::rls::IntoNoDrop;

    #[test]
    fn rls_into_no_drop() {
        let wrapper = 42.no_drop();
        assert_eq!(wrapper.consume(), 42);
    }
}
