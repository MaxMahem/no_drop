use super::NoDropNoOp;
use crate::DEFAULT_DROP_PANIC_MSG;
use crate::markers::Msg;
use alloc::borrow::Cow;

// Implementation for NoDropNoOp<T, Msg> (message variant)
impl<'msg, T> NoDropNoOp<'msg, T, Msg> {
    /// Panic message if type is default constructed
    pub const DEFAULT_PANIC_MSG: &'static str = DEFAULT_DROP_PANIC_MSG;

    /// Creates a new wrapper around `value` with a custom panic message.
    ///
    /// The message is immediately dropped and ignored, since this type never [`panic!`]s.
    pub fn wrap<M: Into<Cow<'msg, str>>>(value: T, _msg: M) -> Self {
        Self::new(value)
    }
}

// Implementation for NoDropNoOp<(), Msg> (empty message variant)
impl<'msg> NoDropNoOp<'msg, (), Msg> {
    /// Creates a new empty no drop guard, with a custom panic message.
    ///
    /// The message is immediately dropped and ignored, since this type never [`panic!`]s.
    pub fn guard<M: Into<Cow<'msg, str>>>(_msg: M) -> Self {
        Self::EMPTY
    }
}
