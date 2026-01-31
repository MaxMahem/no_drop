use super::NoDropNoOp;
use crate::DEFAULT_DROP_PANIC_MSG;
use crate::markers::NoMsg;

// Implementation for NoDropNoOp<T, NoMsg> (no message variant)
impl<T> NoDropNoOp<'static, T, NoMsg> {
    /// Default panic message that would be used if this type panicked (it doesn't).
    pub const PANIC_MSG: &'static str = DEFAULT_DROP_PANIC_MSG;

    /// Creates a new wrapper around `value`.
    pub const fn wrap(value: T) -> Self {
        Self::new(value)
    }
}

// Implementation for NoDropNoOp<(), NoMsg> (empty no message variant)
impl NoDropNoOp<'static, (), NoMsg> {
    /// Creates a new empty guard.
    pub const fn guard() -> Self {
        Self::EMPTY
    }
}
