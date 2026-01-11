#[doc(hidden)]
pub mod drop_guard_empty;
#[doc(hidden)]
pub mod drop_guard_msg;
#[doc(hidden)]
pub mod drop_guard_no_op;

mod guard_state;
pub use guard_state::GuardState;

pub use drop_guard_empty::DropGuardEmpty;
pub use drop_guard_msg::{DropGuardMsg, DropGuardMsg as DropGuard};
pub type DropGuardNoOpEmpty = drop_guard_no_op::DropGuardNoOp<'static, crate::markers::NoMsg>;
pub type DropGuardNoOpMsg<'msg> = drop_guard_no_op::DropGuardNoOp<'msg, crate::markers::Msg>;

/// Error type returned when attempting to turn a Drop Guard into a No Drop when it is not armed.
#[derive(Debug, thiserror::Error)]
#[error("guard is not armed")]
pub struct GuardNotArmed;
