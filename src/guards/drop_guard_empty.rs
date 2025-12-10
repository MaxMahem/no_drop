use crate::{guards::GuardNotArmed, no_drop::NoDropEmpty};

/// A mutable drop guard.
///
/// This guard can be toggled between [`Self::armed`] and [`Self::disarmed`] states via
/// [`Self::arm`] and [`Self::disarm`], respectively. While [`Self::armed`] it will [`panic!`]
/// if dropped, when [`Self::disarmed`] it will not.
///
/// This can be used to guard a critical state or another type, ensuring it is not dropped while in
/// that state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DropGuardEmpty(Option<NoDropEmpty>);

impl DropGuardEmpty {
    /// Creates a new armed guard.
    #[must_use]
    pub fn new_armed() -> Self {
        Self(Some(NoDropEmpty::new()))
    }

    /// Creates a new disarmed guard.
    #[must_use]
    pub fn new_disarmed() -> Self {
        Self(None)
    }

    /// Returns whether the guard is armed.
    #[must_use]
    pub fn armed(&self) -> bool {
        self.0.is_some()
    }

    /// Returns whether the guard is disarmed.
    #[must_use]
    pub fn disarmed(&self) -> bool {
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

#[rustfmt::skip]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::guards::test_macros::{ctor, try_from, transition};

    ctor!(new_armed, DropGuardEmpty::new_armed, (), armed, "Value was dropped without being unwrapped");
    ctor!(new_disarmed, DropGuardEmpty::new_disarmed, (), disarmed);
    ctor!(from_no_drop, DropGuardEmpty::from, (NoDropEmpty::new()), armed, "Value was dropped without being unwrapped");

    try_from!(try_from_armed, DropGuardEmpty::new_armed, (), NoDropEmpty, armed);
    try_from!(try_from_disarmed, DropGuardEmpty::new_disarmed, (), NoDropEmpty, disarmed);

    transition!(arm_when_disarmed, DropGuardEmpty::new_disarmed, (), arm, true, armed, "Value was dropped without being unwrapped");
    transition!(arm_when_armed, DropGuardEmpty::new_armed, (), arm, false, armed, "Value was dropped without being unwrapped");
    transition!(disarm_when_armed, DropGuardEmpty::new_armed, (), disarm, true, disarmed);
    transition!(disarm_when_disarmed, DropGuardEmpty::new_disarmed, (), disarm, false, disarmed);
}
