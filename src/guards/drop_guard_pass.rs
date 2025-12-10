use std::borrow::Cow;

use crate::markers::{Empty, Msg, PassMarker};

/// A zero-cost wrapper with no drop checking.
///
/// This is a transparent no-op wrapper. It does not [`panic!`] when dropped.
/// Intended to be transparently substituted for [`DropGuard`](super::DropGuard)
/// or [`DropGuardMsg`](super::DropGuardMsg) in release builds.
///
/// The type parameter `M` is a zero-sized marker that distinguishes between
/// the plain variant ([`Empty`]) and the message variant ([`Msg`]).
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc(hidden)]
#[must_use]
pub struct DropGuardPass<'msg, M: PassMarker = Empty> {
    armed: bool,
    _lifetime: std::marker::PhantomData<&'msg ()>,
    _marker: std::marker::PhantomData<M>,
}

// Implementation for DropGuardPass<Empty> (no message variant)
#[allow(dead_code)]
impl DropGuardPass<'static, Empty> {
    /// Creates a new armed guard.
    pub fn new_armed() -> Self {
        Self { armed: true, _lifetime: std::marker::PhantomData, _marker: std::marker::PhantomData }
    }

    /// Creates a new disarmed guard.
    pub fn new_disarmed() -> Self {
        Self { armed: false, _lifetime: std::marker::PhantomData, _marker: std::marker::PhantomData }
    }
}

// Implementation for DropGuardPass<Msg> (message variant)
#[allow(dead_code)]
impl<'msg> DropGuardPass<'msg, Msg> {
    /// Creates a new armed guard with a custom panic message.
    ///
    /// The message is immediately dropped and ignored, since this type never [`panic!`]s.
    pub fn new_armed<M: Into<Cow<'msg, str>>>(_msg: M) -> Self {
        Self { armed: true, _lifetime: std::marker::PhantomData, _marker: std::marker::PhantomData }
    }

    /// Creates a new disarmed guard with a custom panic message.
    ///
    /// The message is immediately dropped and ignored, since this type never [`panic!`]s.
    pub fn new_disarmed<M: Into<Cow<'msg, str>>>(_msg: M) -> Self {
        Self { armed: false, _lifetime: std::marker::PhantomData, _marker: std::marker::PhantomData }
    }
}

// Shared implementation for both variants
#[allow(dead_code)]
impl<M: PassMarker> DropGuardPass<'_, M> {
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

    // Tests for Empty variant
    #[test]
    fn passthrough_empty_new_armed() {
        let mut guard = DropGuardPass::<Empty>::new_armed();
        assert!(guard.armed());
        assert!(!guard.disarmed());
        guard.disarm();
    }

    #[test]
    fn passthrough_empty_new_disarmed() {
        let guard = DropGuardPass::<Empty>::new_disarmed();
        assert!(guard.disarmed());
        assert!(!guard.armed());
    }

    #[test]
    fn passthrough_empty_arm_disarm() {
        let mut guard = DropGuardPass::<Empty>::new_disarmed();
        assert!(guard.arm());
        assert!(guard.armed());
        assert!(guard.disarm());
        assert!(guard.disarmed());
    }

    #[test]
    fn passthrough_empty_drop_no_panic() {
        let guard = DropGuardPass::<Empty>::new_armed();
        drop(guard); // No panic even when armed
    }

    // Tests for Msg variant
    #[test]
    fn passthrough_msg_new_armed() {
        let mut guard = DropGuardPass::<Msg>::new_armed("message");
        assert!(guard.armed());
        assert!(!guard.disarmed());
        guard.disarm();
    }

    #[test]
    fn passthrough_msg_new_disarmed() {
        let guard = DropGuardPass::<Msg>::new_disarmed("message");
        assert!(guard.disarmed());
        assert!(!guard.armed());
    }

    #[test]
    fn passthrough_msg_arm_disarm() {
        let mut guard = DropGuardPass::<Msg>::new_disarmed("message");
        assert!(guard.arm());
        assert!(guard.armed());
        assert!(guard.disarm());
        assert!(guard.disarmed());
    }

    #[test]
    fn passthrough_msg_drop_no_panic() {
        let guard = DropGuardPass::<Msg>::new_armed("should not panic");
        drop(guard); // No panic even when armed
    }
}
