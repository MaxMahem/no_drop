use super::DropGuardNoOp;
use crate::DEFAULT_DROP_PANIC_MSG;
use crate::guards::{GuardNotArmed, GuardState};
use crate::markers::NoMsg;
use crate::wrap::NoDropNoOpEmpty;
use core::convert::TryFrom;

// Implementation for DropGuardNoOp<NoMsg> (no message variant)
impl DropGuardNoOp<'static, NoMsg> {
    /// Default panic message that would be used if this type panicked (it doesn't).
    pub const PANIC_MSG: &'static str = DEFAULT_DROP_PANIC_MSG;

    /// Creates a new armed guard.
    pub const fn new_armed() -> Self {
        Self::ARMED
    }

    /// Creates a new disarmed guard.
    pub const fn new_disarmed() -> Self {
        Self::DISARMED
    }

    /// Consumes the guard, returning the inner [`NoDropNoOpEmpty`] if armed, or [`None`] if disarmed.
    #[must_use]
    pub const fn into_guard(self) -> Option<NoDropNoOpEmpty> {
        match self.state {
            GuardState::Armed => Some(NoDropNoOpEmpty::new(())),
            GuardState::Disarmed => None,
        }
    }
}

impl From<NoDropNoOpEmpty> for DropGuardNoOp<'_, NoMsg> {
    fn from(_: NoDropNoOpEmpty) -> Self {
        Self::ARMED
    }
}

impl TryFrom<DropGuardNoOp<'static, NoMsg>> for NoDropNoOpEmpty {
    type Error = GuardNotArmed;

    fn try_from(value: DropGuardNoOp<'static, NoMsg>) -> Result<Self, Self::Error> {
        value.into_guard().ok_or(GuardNotArmed)
    }
}
