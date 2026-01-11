use std::borrow::Cow;
use std::convert::TryFrom;

use crate::DEFAULT_DROP_PANIC_MSG;
use crate::guards::{GuardNotArmed, GuardState};
use crate::markers::{Msg, MsgMarker, NoMsg};
use crate::wrap::{NoDropNoOpEmpty, NoDropNoOpMsg};

/// A zero-cost mutable drop guard that never panics.
///
/// This guard can be toggled between armed and disarmed states but will never panic on drop,
/// making it a zero-cost abstraction suitable for release builds.
///
/// The type parameter `M` is a zero-sized marker that distinguishes between the plain variant
/// ([`NoMsg`]) and the message variant ([`Msg`]).
#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[must_use]
pub struct DropGuardNoOp<'msg, M: MsgMarker = NoMsg> {
    state: GuardState,
    _lifetime: std::marker::PhantomData<&'msg ()>,
    _marker: std::marker::PhantomData<M>,
}

// Implementation for DropGuardNoOp<Empty> (no message variant)
#[allow(dead_code)]
impl DropGuardNoOp<'static, NoMsg> {
    /// Default panic message that would be used if this type panicked (it doesn't).
    pub const PANIC_MSG: &'static str = DEFAULT_DROP_PANIC_MSG;

    /// Creates a new armed guard.
    pub const fn new_armed() -> Self {
        Self::new(GuardState::Armed)
    }

    /// Creates a new disarmed guard.
    pub const fn new_disarmed() -> Self {
        Self::new(GuardState::Disarmed)
    }

    /// Consumes the guard, returning the inner [`NoDropNoOpEmpty`] if armed, or [`None`] if disarmed.
    #[must_use]
    pub const fn into_guard(self) -> Option<NoDropNoOpEmpty> {
        match self.state {
            GuardState::Armed => Some(NoDropNoOpEmpty::new()),
            GuardState::Disarmed => None,
        }
    }
}

// Implementation for DropGuardNoOp<Msg> (message variant)
#[allow(dead_code)]
impl<'msg> DropGuardNoOp<'msg, Msg> {
    /// Creates a new armed guard with a custom panic message.
    ///
    /// The message is immediately dropped and ignored, since this type never [`panic!`]s.
    pub fn new_armed<M: Into<Cow<'msg, str>>>(_msg: M) -> Self {
        Self::new(GuardState::Armed)
    }

    /// Creates a new disarmed guard with a custom panic message.
    ///
    /// The message is immediately dropped and ignored, since this type never [`panic!`]s.
    pub fn new_disarmed<M: Into<Cow<'msg, str>>>(_msg: M) -> Self {
        Self::new(GuardState::Disarmed)
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

// Shared implementation for both variants
#[allow(dead_code)]
impl<M: MsgMarker> DropGuardNoOp<'_, M> {
    /// Internal constructor for guards.
    pub(crate) const fn new(state: GuardState) -> Self {
        Self { state, _lifetime: std::marker::PhantomData, _marker: std::marker::PhantomData }
    }

    /// Returns whether the guard is armed.
    #[must_use]
    pub const fn armed(&self) -> bool {
        matches!(self.state, GuardState::Armed)
    }

    /// Returns whether the guard is disarmed.
    #[must_use]
    pub const fn disarmed(&self) -> bool {
        matches!(self.state, GuardState::Disarmed)
    }

    /// Arms the guard.
    ///
    /// Returns `true` if the guard was armed, or `false` if it was already armed.
    #[allow(clippy::missing_const_for_fn, reason = "API consistency with other guard types")]
    pub fn arm(&mut self) -> bool {
        match self.state {
            GuardState::Armed => false,
            GuardState::Disarmed => {
                self.state = GuardState::Armed;
                true
            }
        }
    }

    /// Disarms the guard.
    ///
    /// Returns `true` if the guard was disarmed or `false` if it was already disarmed.
    #[allow(clippy::missing_const_for_fn, reason = "API consistency with other guard types")]
    pub fn disarm(&mut self) -> bool {
        match self.state {
            GuardState::Disarmed => false,
            GuardState::Armed => {
                self.state = GuardState::Disarmed;
                true
            }
        }
    }

    /// Toggles the guard between armed and disarmed states.
    ///
    /// Returns the new state of the guard.
    #[allow(clippy::missing_const_for_fn, reason = "API consistency with other guard types")]
    pub fn toggle(&mut self) -> GuardState {
        self.state = match self.state {
            GuardState::Armed => GuardState::Disarmed,
            GuardState::Disarmed => GuardState::Armed,
        };
        self.state
    }
}

impl From<NoDropNoOpEmpty> for DropGuardNoOp<'_, NoMsg> {
    fn from(_: NoDropNoOpEmpty) -> Self {
        Self::new(GuardState::Armed)
    }
}

impl From<NoDropNoOpMsg<'_>> for DropGuardNoOp<'_, Msg> {
    fn from(_: NoDropNoOpMsg<'_>) -> Self {
        Self::new(GuardState::Armed)
    }
}

impl TryFrom<DropGuardNoOp<'static, NoMsg>> for NoDropNoOpEmpty {
    type Error = GuardNotArmed;

    fn try_from(value: DropGuardNoOp<'static, NoMsg>) -> Result<Self, Self::Error> {
        value.into_guard().ok_or(GuardNotArmed)
    }
}

impl<'msg> TryFrom<DropGuardNoOp<'msg, Msg>> for NoDropNoOpMsg<'msg> {
    type Error = GuardNotArmed;

    fn try_from(value: DropGuardNoOp<'msg, Msg>) -> Result<Self, Self::Error> {
        value.into_guard().ok_or(GuardNotArmed)
    }
}
