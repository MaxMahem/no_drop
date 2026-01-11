use std::borrow::Cow;

use crate::guards::{GuardNotArmed, GuardState};
use crate::wrap::NoDropMsg;

/// A mutable drop guard with custom panic message.
///
/// This guard can be toggled between [`Self::armed`] and [`Self::disarmed`] states via
/// [`Self::arm`] and [`Self::disarm`], respectively. While [`Self::armed`] it will [`panic!`]
/// with the custom message if dropped, when [`Self::disarmed`] it will not.
///
/// This can be used to guard a critical state or another type, ensuring it is not dropped while in
/// that state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DropGuardMsg<'msg>(DropGuardMsgState<'msg>);

#[derive(Debug, Clone, PartialEq, Eq)]
enum DropGuardMsgState<'msg> {
    Armed(NoDropMsg<'msg>),
    Disarmed(Cow<'msg, str>),
}

impl DropGuardMsgState<'_> {
    const EMPTY: Self = Self::Disarmed(Cow::Borrowed(""));

    /// Arms the state, consuming it and returning the armed state.
    ///
    /// Returns the state unchanged if already armed.
    fn arm(self) -> (Self, bool) {
        match self {
            Self::Armed(_) => (self, false),
            Self::Disarmed(msg) => (Self::Armed(NoDropMsg::guard(msg)), true),
        }
    }

    /// Disarms the state, consuming it and returning the disarmed state.
    ///
    /// Returns the state unchanged if already disarmed.
    fn disarm(self) -> (Self, bool) {
        match self {
            Self::Disarmed(_) => (self, false),
            Self::Armed(guard) => (Self::Disarmed(guard.unwrap_msg()), true),
        }
    }

    /// Toggles the state between armed and disarmed.
    fn toggle(self) -> (Self, GuardState) {
        match self {
            Self::Armed(guard) => (Self::Disarmed(guard.unwrap_msg()), GuardState::Disarmed),
            Self::Disarmed(msg) => (Self::Armed(NoDropMsg::guard(msg)), GuardState::Armed),
        }
    }
}

impl<'msg> DropGuardMsg<'msg> {
    /// Creates a new armed guard with a custom panic message.
    #[must_use]
    pub fn new_armed<M: Into<Cow<'msg, str>>>(msg: M) -> Self {
        Self(DropGuardMsgState::Armed(NoDropMsg::guard(msg)))
    }

    /// Creates a new disarmed guard with a custom panic message.
    ///
    /// The message is retained and will be used if the guard is later armed.
    #[must_use]
    pub fn new_disarmed<M: Into<Cow<'msg, str>>>(msg: M) -> Self {
        Self(DropGuardMsgState::Disarmed(msg.into()))
    }

    /// Returns whether the guard is armed.
    #[must_use]
    pub const fn armed(&self) -> bool {
        matches!(self.0, DropGuardMsgState::Armed(_))
    }

    /// Returns whether the guard is disarmed.
    #[must_use]
    pub const fn disarmed(&self) -> bool {
        matches!(self.0, DropGuardMsgState::Disarmed(_))
    }

    /// Arms the guard.
    ///
    /// Returns `true` if the guard was armed, or `false` if it was already armed.
    pub fn arm(&mut self) -> bool {
        self.apply_state_change(DropGuardMsgState::arm)
    }

    /// Disarms the guard.
    ///
    /// Returns `true` if the guard was disarmed or `false` if it was already disarmed.
    pub fn disarm(&mut self) -> bool {
        self.apply_state_change(DropGuardMsgState::disarm)
    }

    /// Helper method to apply a state transformation function.
    fn apply_state_change(
        &mut self,
        f: impl FnOnce(DropGuardMsgState<'msg>) -> (DropGuardMsgState<'msg>, bool),
    ) -> bool {
        let (new_state, changed) = f(std::mem::replace(&mut self.0, DropGuardMsgState::EMPTY));
        self.0 = new_state;
        changed
    }

    /// Toggles the guard between armed and disarmed states.
    ///
    /// Returns the new state of the guard.
    pub fn toggle(&mut self) -> GuardState {
        let (new_state, state) = std::mem::replace(&mut self.0, DropGuardMsgState::EMPTY).toggle();
        self.0 = new_state;
        state
    }

    /// Sets a new panic message, preserving the armed/disarmed state.
    ///
    /// Note: This changes the lifetime of the guard to match the new message's lifetime.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use no_drop::rls::DropGuard;
    ///
    /// let mut guard = DropGuard::new_disarmed("original message");
    /// let mut guard = guard.set_msg("new message");
    /// ```
    pub fn set_msg<'new, M: Into<Cow<'new, str>>>(self, msg: M) -> DropGuardMsg<'new> {
        let msg = msg.into();
        match self.0 {
            DropGuardMsgState::Armed(guard) => DropGuardMsg(DropGuardMsgState::Armed(guard.set_msg(msg))),
            DropGuardMsgState::Disarmed(_) => DropGuardMsg(DropGuardMsgState::Disarmed(msg)),
        }
    }

    /// Consumes the guard, returning the inner [`NoDropMsg`] if armed, or [`None`] if disarmed.
    #[must_use]
    pub fn into_guard(self) -> Option<NoDropMsg<'msg>> {
        match self.0 {
            DropGuardMsgState::Armed(guard) => Some(guard),
            DropGuardMsgState::Disarmed(_) => None,
        }
    }
}

impl<'msg> From<NoDropMsg<'msg>> for DropGuardMsg<'msg> {
    fn from(no_drop: NoDropMsg<'msg>) -> Self {
        Self(DropGuardMsgState::Armed(no_drop))
    }
}

impl<'msg> TryFrom<DropGuardMsg<'msg>> for NoDropMsg<'msg> {
    type Error = GuardNotArmed;

    fn try_from(value: DropGuardMsg<'msg>) -> Result<Self, Self::Error> {
        value.into_guard().ok_or(GuardNotArmed)
    }
}
