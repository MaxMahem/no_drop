use std::borrow::Cow;

use crate::{
    guards::GuardNotArmed,
    markers::{Empty, Msg, PassMarker},
    no_drop::{NoDropPassEmpty, NoDropPassMsg},
};

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

    /// Consumes the guard, returning the inner [`NoDropPassEmpty`] if armed, or [`None`] if disarmed.
    #[must_use]
    pub fn into_guard(self) -> Option<NoDropPassEmpty> {
        match self.armed {
            true => Some(NoDropPassEmpty::new()),
            false => None,
        }
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

    /// Consumes the guard, returning the inner [`NoDropPassMsg`] if armed, or [`None`] if disarmed.
    #[must_use]
    pub fn into_guard(self) -> Option<NoDropPassMsg<'msg>> {
        match self.armed {
            true => Some(NoDropPassMsg::guard("")),
            false => None,
        }
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

impl From<NoDropPassEmpty> for DropGuardPass<'_, Empty> {
    fn from(_: NoDropPassEmpty) -> Self {
        Self { armed: true, _lifetime: std::marker::PhantomData, _marker: std::marker::PhantomData }
    }
}

impl From<NoDropPassMsg<'_>> for DropGuardPass<'_, Msg> {
    fn from(_: NoDropPassMsg<'_>) -> Self {
        Self { armed: true, _lifetime: std::marker::PhantomData, _marker: std::marker::PhantomData }
    }
}

impl TryFrom<DropGuardPass<'static, Empty>> for NoDropPassEmpty {
    type Error = GuardNotArmed;

    fn try_from(value: DropGuardPass<'static, Empty>) -> Result<Self, Self::Error> {
        value.into_guard().ok_or(GuardNotArmed)
    }
}

impl<'msg> TryFrom<DropGuardPass<'msg, Msg>> for NoDropPassMsg<'msg> {
    type Error = GuardNotArmed;

    fn try_from(value: DropGuardPass<'msg, Msg>) -> Result<Self, Self::Error> {
        value.into_guard().ok_or(GuardNotArmed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::guards::test_macros::{ctor, transition, try_from};

    mod empty {
        use super::*;

        ctor!(new_armed, DropGuardPass::<Empty>::new_armed, (), armed_no_panic);
        ctor!(new_disarmed, DropGuardPass::<Empty>::new_disarmed, (), disarmed);
        ctor!(from, DropGuardPass::<Empty>::from, (NoDropPassEmpty::new()), armed_no_panic);

        try_from!(try_from_armed, DropGuardPass::<Empty>::new_armed, (), NoDropPassEmpty, armed);
        try_from!(try_from_disarmed, DropGuardPass::<Empty>::new_disarmed, (), NoDropPassEmpty, disarmed);

        transition!(arm_when_disarmed, DropGuardPass::<Empty>::new_disarmed, (), arm, true, armed_no_panic);
        transition!(arm_when_armed, DropGuardPass::<Empty>::new_armed, (), arm, false, armed_no_panic);
        transition!(disarm_when_armed, DropGuardPass::<Empty>::new_armed, (), disarm, true, disarmed);
        transition!(disarm_when_disarmed, DropGuardPass::<Empty>::new_disarmed, (), disarm, false, disarmed);
    }

    mod msg {
        use super::*;

        ctor!(new_armed, DropGuardPass::<Msg>::new_armed, ("message"), armed_no_panic);
        ctor!(new_disarmed, DropGuardPass::<Msg>::new_disarmed, ("message"), disarmed);
        ctor!(from, DropGuardPass::<Msg>::from, (NoDropPassMsg::guard("message")), armed_no_panic);

        try_from!(try_from_armed, DropGuardPass::<Msg>::new_armed, ("msg"), NoDropPassMsg, armed);
        try_from!(try_from_disarmed, DropGuardPass::<Msg>::new_disarmed, ("msg"), NoDropPassMsg, disarmed);

        transition!(arm_when_disarmed, DropGuardPass::<Msg>::new_disarmed, ("test"), arm, true, armed_no_panic);
        transition!(arm_when_armed, DropGuardPass::<Msg>::new_armed, ("test"), arm, false, armed_no_panic);
        transition!(disarm_when_armed, DropGuardPass::<Msg>::new_armed, ("test"), disarm, true, disarmed);
        transition!(disarm_when_disarmed, DropGuardPass::<Msg>::new_disarmed, ("test"), disarm, false, disarmed);
    }
}
