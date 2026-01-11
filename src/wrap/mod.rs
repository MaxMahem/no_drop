#[doc(hidden)]
pub mod no_drop_empty;
#[doc(hidden)]
pub mod no_drop_msg;
#[doc(hidden)]
pub mod no_drop_no_op_empty;
#[doc(hidden)]
pub mod no_drop_no_op_msg;

pub use no_drop_empty::NoDropEmpty as NoDrop;
pub use no_drop_empty::NoDropEmpty;
pub use no_drop_msg::NoDropMsg;

// Type aliases for no_op variants
pub type NoDropNoOpEmpty<T = ()> = no_drop_no_op_empty::NoDropNoOp<T>;
pub type NoDropNoOpMsg<'msg, T = ()> = no_drop_no_op_msg::NoDropNoOp<'msg, T>;
