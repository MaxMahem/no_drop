use no_drop::dbg::*;

#[test]
fn dbg_consume_returns_value() {
    let value = NoDrop::wrap(42);
    assert_eq!(value.consume(), 42);
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = "Value was dropped without being consumed")]
fn dbg_panics_on_drop_in_debug() {
    let _value = NoDrop::wrap(42);
    // Should panic in debug mode
}

#[test]
#[cfg(not(debug_assertions))]
fn dbg_does_not_panic_in_release() {
    let _value = NoDrop::new(42);
    // Should not panic in release mode
}
