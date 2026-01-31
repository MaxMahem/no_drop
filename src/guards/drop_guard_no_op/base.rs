use core::marker::PhantomData;

use crate::guards::GuardState;
use crate::markers::{MsgMarker, NoMsg};

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
    pub(crate) state: GuardState,
    _lifetime: PhantomData<&'msg ()>,
    _marker: PhantomData<M>,
}

// Shared implementation for both variants
impl<M: MsgMarker> DropGuardNoOp<'_, M> {
    pub(crate) const ARMED: Self = Self { state: GuardState::Armed, _lifetime: PhantomData, _marker: PhantomData };
    pub(crate) const DISARMED: Self =
        Self { state: GuardState::Disarmed, _lifetime: PhantomData, _marker: PhantomData };

    /// Returns the current state of the guard.
    #[must_use]
    pub const fn state(&self) -> GuardState {
        self.state
    }

    /// Returns whether the guard is armed.
    #[must_use]
    pub const fn is_armed(&self) -> bool {
        matches!(self.state, GuardState::Armed)
    }

    /// Returns whether the guard is disarmed.
    #[must_use]
    pub const fn is_disarmed(&self) -> bool {
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
