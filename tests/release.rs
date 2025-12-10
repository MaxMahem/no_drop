use no_drop::rls::*;

#[test]
fn release_consume_returns_value() {
    let value = NoDrop::wrap(42);
    assert_eq!(value.consume(), 42);
}

#[test]
#[should_panic(expected = "Value was dropped without being consumed")]
fn release_panics_on_drop() {
    let _value = NoDrop::wrap(42);
    // Should always panic in all build modes
}
