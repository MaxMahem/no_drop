use super::DropGuardNoOp;
use crate::guards::{GuardNotArmed, GuardState};
use crate::markers::Msg;
use crate::wrap::NoDropNoOpMsg;
use alloc::borrow::Cow;
use core::convert::TryFrom;

// Implementation for DropGuardNoOp<Msg> (message variant)
impl<'msg> DropGuardNoOp<'msg, Msg> {
    /// Creates a new armed guard with a custom panic message.
    ///
    /// The message is immediately dropped and ignored, since this type never [`panic!`]s.
    pub fn new_armed<M: Into<Cow<'msg, str>>>(_msg: M) -> Self {
        Self::ARMED
    }

    /// Creates a new disarmed guard with a custom panic message.
    ///
    /// The message is immediately dropped and ignored, since this type never [`panic!`]s.
    pub fn new_disarmed<M: Into<Cow<'msg, str>>>(_msg: M) -> Self {
        Self::DISARMED
    }

    /// Consumes the guard, returning the inner [`NoDropNoOpMsg`] if armed, or [`None`] if disarmed.
    #[must_use]
    #[allow(clippy::missing_const_for_fn, reason = "API consistency with other guard types")]
    pub fn into_guard(self) -> Option<NoDropNoOpMsg<'msg>> {
        match self.state {
            GuardState::Armed => Some(NoDropNoOpMsg::new(())),
            GuardState::Disarmed => None,
        }
    }

    /// Sets a new panic message, preserving the armed/disarmed state.
    ///
    /// This method is a no-op for this type, since it never panics. The message is immediately
    /// dropped and ignored.
    pub fn set_msg<M: Into<Cow<'msg, str>>>(self, _msg: M) -> Self {
        self
    }
}

impl<'msg> From<NoDropNoOpMsg<'msg>> for DropGuardNoOp<'msg, Msg> {
    fn from(_: NoDropNoOpMsg<'msg>) -> Self {
        Self::ARMED
    }
}

impl<'msg> TryFrom<DropGuardNoOp<'msg, Msg>> for NoDropNoOpMsg<'msg> {
    type Error = GuardNotArmed;

    fn try_from(value: DropGuardNoOp<'msg, Msg>) -> Result<Self, Self::Error> {
        value.into_guard().ok_or(GuardNotArmed)
    }
}
