#[doc(hidden)]
pub mod no_drop_empty;
#[doc(hidden)]
pub mod no_drop_msg;
#[doc(hidden)]
pub mod no_drop_no_op;

pub use no_drop_empty::NoDropEmpty as NoDrop;
pub use no_drop_empty::NoDropEmpty;
pub use no_drop_msg::NoDropMsg;

use crate::markers::{Msg, NoMsg};

// Type aliases for no_op variants
pub type NoDropNoOpEmpty<T = ()> = no_drop_no_op::NoDropNoOp<'static, T, NoMsg>;
pub type NoDropNoOpMsg<'msg, T = ()> = no_drop_no_op::NoDropNoOp<'msg, T, Msg>;
