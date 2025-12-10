use std::borrow::Cow;

use crate::{dbg::NoDropMsg, guards::GuardNotArmed};

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

impl Default for DropGuardMsgState<'_> {
    fn default() -> Self {
        Self::Disarmed(Cow::Borrowed(""))
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
    pub fn armed(&self) -> bool {
        matches!(self.0, DropGuardMsgState::Armed(_))
    }

    /// Returns whether the guard is disarmed.
    #[must_use]
    pub fn disarmed(&self) -> bool {
        matches!(self.0, DropGuardMsgState::Disarmed(_))
    }

    /// Arms the guard.
    ///
    /// Returns `true` if the guard was armed, or `false` if it was already armed.
    pub fn arm(&mut self) -> bool {
        match &mut self.0 {
            DropGuardMsgState::Armed(_) => false,
            DropGuardMsgState::Disarmed(msg) => {
                let msg = std::mem::take(msg);
                self.0 = DropGuardMsgState::Armed(NoDropMsg::guard(msg));
                true
            }
        }
    }

    /// Disarms the guard.
    ///
    /// Returns `true` if the guard was disarmed or `false` if it was already disarmed.
    pub fn disarm(&mut self) -> bool {
        match std::mem::take(&mut self.0) {
            DropGuardMsgState::Disarmed(_) => false,
            DropGuardMsgState::Armed(guard) => {
                let msg = guard.unwrap_msg();
                self.0 = DropGuardMsgState::Disarmed(msg);
                true
            }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::guards::test_macros::{ctor, transition, try_from};

    ctor!(new_armed, DropGuardMsg::new_armed, ("custom panic message"), armed, "custom panic message");
    ctor!(new_disarmed, DropGuardMsg::new_disarmed, ("custom message"), disarmed);
    ctor!(from_no_drop, DropGuardMsg::from, (NoDropMsg::guard("custom")), armed, "custom");

    try_from!(try_from_armed, DropGuardMsg::new_armed, ("message"), NoDropMsg, armed);
    try_from!(try_from_disarmed, DropGuardMsg::new_disarmed, ("message"), NoDropMsg, disarmed);

    transition!(arm_when_disarmed, DropGuardMsg::new_disarmed, ("test"), arm, true, armed, "test");
    transition!(arm_when_armed, DropGuardMsg::new_armed, ("test"), arm, false, armed, "test");
    transition!(disarm_when_armed, DropGuardMsg::new_armed, ("test"), disarm, true, disarmed);
    transition!(disarm_when_disarmed, DropGuardMsg::new_disarmed, ("test"), disarm, false, disarmed);

    #[test]
    fn default_is_disarmed() {
        let state = DropGuardMsgState::default();
        assert_eq!(state, DropGuardMsgState::Disarmed(Cow::Borrowed("")));
    }
}
