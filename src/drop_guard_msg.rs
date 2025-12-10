use std::borrow::Cow;

use crate::dbg::NoDropMsg;

/// A mutable drop guard with custom panic message.
///
/// This guard can be toggled between [`Self::armed`] and [`Self::disarmed`] states via
/// [`Self::arm`] and [`Self::disarm`], respectively. While [`Self::armed`] it will [`panic!`]
/// with the custom message if dropped, when [`Self::disarmed`] it will not.
///
/// The panic message is retained even when disarmed, allowing the guard to be rearmed with
/// the same message.
///
/// This can be used to guard a critical state or another type, ensuring it is not dropped while in
/// that state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DropGuardMsg<'msg>(DropGuardMsgState<'msg>);

#[derive(Debug, Clone, PartialEq, Eq, derivative::Derivative)]
#[derivative(Default)]
enum DropGuardMsgState<'msg> {
    Armed(NoDropMsg<'msg>),
    #[derivative(Default)]
    Disarmed(Cow<'msg, str>),
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
}

/// A zero-cost wrapper with no drop checking.
///
/// This is a transparent no-op wrapper. It does not [`panic!`] when dropped.
/// Intended to be transparently substituted for [`DropGuardMsg`] in release builds.
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc(hidden)]
#[must_use]
pub struct DropGuardMsgPassthrough<'msg> {
    armed: bool,
    _lifetime: std::marker::PhantomData<&'msg ()>,
}

#[allow(dead_code)]
impl<'msg> DropGuardMsgPassthrough<'msg> {
    /// Creates a new armed guard with a custom panic message.
    ///
    /// The message is immediately dropped and ignored, since this type never [`panic!`]s.
    pub fn new_armed<M: Into<Cow<'msg, str>>>(_msg: M) -> Self {
        Self { armed: true, _lifetime: std::marker::PhantomData }
    }

    /// Creates a new disarmed guard with a custom panic message.
    ///
    /// The message is immediately dropped and ignored, since this type never [`panic!`]s.
    pub fn new_disarmed<M: Into<Cow<'msg, str>>>(_msg: M) -> Self {
        Self { armed: false, _lifetime: std::marker::PhantomData }
    }

    /// Returns whether the guard is armed.
    pub fn armed(&self) -> bool {
        self.armed
    }

    /// Returns whether the guard is disarmed.
    pub fn disarmed(&self) -> bool {
        !self.armed
    }

    /// Arms the guard.
    ///
    /// Returns `true` if the guard was armed, or `false` if it was already armed.
    pub fn arm(&mut self) -> bool {
        !std::mem::replace(&mut self.armed, true)
    }

    /// Disarms the guard.
    ///
    /// Returns `true` if the guard was disarmed or `false` if it was already disarmed.
    pub fn disarm(&mut self) -> bool {
        std::mem::replace(&mut self.armed, false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_armed() {
        let mut guard = DropGuardMsg::new_armed("custom panic message");
        assert!(guard.armed());
        assert!(!guard.disarmed());
        guard.disarm(); // Prevent panic on drop
    }

    #[test]
    fn new_disarmed() {
        let guard = DropGuardMsg::new_disarmed("custom message");
        assert!(guard.disarmed());
        assert!(!guard.armed());
    }

    #[test]
    fn arm_when_disarmed() {
        let mut guard = DropGuardMsg::new_disarmed("test message");
        let changed = guard.arm();
        assert!(changed); // State changed: disarmed -> armed
        assert!(guard.armed());
        guard.disarm(); // Prevent panic on drop
    }

    #[test]
    fn arm_when_armed() {
        let mut guard = DropGuardMsg::new_armed("test message");
        let changed = guard.arm();
        assert!(!changed); // No state change: armed -> armed
        assert!(guard.armed());
        guard.disarm(); // Prevent panic on drop
    }

    #[test]
    fn disarm_when_armed() {
        let mut guard = DropGuardMsg::new_armed("test message");
        let changed = guard.disarm();
        assert!(changed); // State changed: armed -> disarmed
        assert!(guard.disarmed());
    }

    #[test]
    fn disarm_when_disarmed() {
        let guard = DropGuardMsg::new_disarmed("test message");
        let mut guard_copy = guard.clone();
        let changed = guard_copy.disarm();
        assert!(!changed); // No state change: disarmed -> disarmed
        assert!(guard_copy.disarmed());
    }

    #[test]
    fn drop_disarmed_no_panic() {
        let guard = DropGuardMsg::new_disarmed("should not panic");
        drop(guard);
    }

    #[test]
    #[should_panic(expected = "custom panic on drop")]
    fn drop_armed_panics() {
        let guard = DropGuardMsg::new_armed("custom panic on drop");
        drop(guard);
    }

    // Passthrough tests
    #[test]
    fn passthrough_new_armed() {
        let mut guard = DropGuardMsgPassthrough::new_armed("message");
        assert!(guard.armed());
        assert!(!guard.disarmed());
        guard.disarm();
    }

    #[test]
    fn passthrough_new_disarmed() {
        let guard = DropGuardMsgPassthrough::new_disarmed("message");
        assert!(guard.disarmed());
        assert!(!guard.armed());
    }

    #[test]
    fn passthrough_arm_disarm() {
        let mut guard = DropGuardMsgPassthrough::new_disarmed("message");
        assert!(guard.arm());
        assert!(guard.armed());
        assert!(guard.disarm());
        assert!(guard.disarmed());
    }

    #[test]
    fn passthrough_drop_no_panic() {
        let guard = DropGuardMsgPassthrough::new_armed("should not panic");
        drop(guard); // No panic even when armed
    }
}
