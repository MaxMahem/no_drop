mod drop_guard_empty;
mod drop_guard_msg;
mod drop_guard_pass;

pub use drop_guard_empty::DropGuardEmpty;
pub use drop_guard_msg::DropGuardMsg;

// Type aliases for passthrough variants
#[allow(dead_code)]
pub(crate) type DropGuardPassthroughEmpty = drop_guard_pass::DropGuardPass<'static, crate::markers::Empty>;
#[allow(dead_code)]
pub(crate) type DropGuardPassthroughMsg<'msg> = drop_guard_pass::DropGuardPass<'msg, crate::markers::Msg>;
