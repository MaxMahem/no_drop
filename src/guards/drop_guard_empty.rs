use crate::no_drop::NoDropEmpty;

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
}

impl From<NoDropEmpty> for DropGuardEmpty {
    fn from(no_drop: NoDropEmpty) -> Self {
        Self(Some(no_drop))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_armed() {
        let mut guard = DropGuardEmpty::new_armed();
        assert!(guard.armed());
        assert!(!guard.disarmed());
        guard.disarm(); // Prevent panic on drop
    }

    #[test]
    fn new_disarmed() {
        let guard = DropGuardEmpty::new_disarmed();
        assert!(guard.disarmed());
        assert!(!guard.armed());
    }

    #[test]
    fn from_no_drop() {
        let no_drop = NoDropEmpty::new();
        let mut guard = DropGuardEmpty::from(no_drop);
        guard.disarm();
    }

    #[test]
    fn arm_when_disarmed() {
        let mut guard = DropGuardEmpty::new_disarmed();
        let changed = guard.arm();
        assert!(changed); // State changed: disarmed -> armed
        assert!(guard.armed());
        guard.disarm(); // Prevent panic on drop
    }

    #[test]
    fn arm_when_armed() {
        let mut guard = DropGuardEmpty::new_armed();
        let changed = guard.arm();
        assert!(!changed); // No state change: armed -> armed
        assert!(guard.armed());
        guard.disarm(); // Prevent panic on drop
    }

    #[test]
    fn disarm_when_armed() {
        let mut guard = DropGuardEmpty::new_armed();
        let changed = guard.disarm();
        assert!(changed); // State changed: armed -> disarmed
        assert!(guard.disarmed());
    }

    #[test]
    fn disarm_when_disarmed() {
        let mut guard = DropGuardEmpty::new_disarmed();
        let changed = guard.disarm();
        assert!(!changed); // No state change: disarmed -> disarmed
        assert!(guard.disarmed());
    }

    // Test drop behavior
    #[test]
    fn drop_disarmed_no_panic() {
        let guard = DropGuardEmpty::new_disarmed();
        drop(guard);
    }

    #[test]
    #[should_panic(expected = "Value was dropped without being consumed")]
    fn drop_armed_panics() {
        let guard = DropGuardEmpty::new_armed();
        drop(guard);
    }
}
