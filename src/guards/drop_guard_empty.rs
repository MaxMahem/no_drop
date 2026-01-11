use crate::DEFAULT_DROP_PANIC_MSG;
use crate::guards::{GuardNotArmed, GuardState};
use crate::wrap::NoDropEmpty;

/// A mutable drop guard with a default panic message.
///
/// This guard can be toggled between [`Self::armed`] and [`Self::disarmed`] states via
/// [`Self::arm`] and [`Self::disarm`], respectively. While [`Self::armed`] it will [`panic!`]
/// if dropped, when [`Self::disarmed`] it will not.
///
/// This can be used to guard a critical state or another type, ensuring it is not dropped while in
/// that state.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct DropGuardEmpty(Option<NoDropEmpty>);

impl DropGuardEmpty {
    /// Default panic message used when the guard is armed and dropped.
    pub const PANIC_MSG: &'static str = DEFAULT_DROP_PANIC_MSG;

    /// Creates a new armed guard.
    #[must_use]
    pub const fn new_armed() -> Self {
        Self(Some(NoDropEmpty::new()))
    }

    /// Creates a new disarmed guard.
    #[must_use]
    pub const fn new_disarmed() -> Self {
        Self(None)
    }

    /// Returns whether the guard is armed.
    #[must_use]
    pub const fn armed(&self) -> bool {
        self.0.is_some()
    }

    /// Returns whether the guard is disarmed.
    #[must_use]
    pub const fn disarmed(&self) -> bool {
        self.0.is_none()
    }

    /// Arms the guard.
    ///
    /// Returns `true` if the guard was armed, or `false` if it was already armed.
    pub fn arm(&mut self) -> bool {
        self.0.replace(NoDropEmpty::new()).map(NoDropEmpty::forget).is_none()
    }

    /// Disarms the guard.
    ///
    /// Returns `true` if the guard was disarmed or `false` if it was already disarmed.
    pub fn disarm(&mut self) -> bool {
        self.0.take().map(NoDropEmpty::forget).is_some()
    }

    /// Toggles the guard between armed and disarmed states.
    ///
    /// Returns the new state of the guard.
    pub fn toggle(&mut self) -> GuardState {
        match self.armed() {
            true => {
                self.disarm();
                GuardState::Disarmed
            }
            false => {
                self.arm();
                GuardState::Armed
            }
        }
    }

    /// Consumes the guard, returning the inner [`NoDropEmpty`] if armed, or [`None`] if disarmed.
    #[must_use]
    pub fn into_guard(self) -> Option<NoDropEmpty> {
        self.0
    }
}

impl From<NoDropEmpty> for DropGuardEmpty {
    fn from(no_drop: NoDropEmpty) -> Self {
        Self(Some(no_drop))
    }
}

impl TryFrom<DropGuardEmpty> for NoDropEmpty {
    type Error = GuardNotArmed;

    fn try_from(value: DropGuardEmpty) -> Result<Self, Self::Error> {
        value.into_guard().ok_or(GuardNotArmed)
    }
}
