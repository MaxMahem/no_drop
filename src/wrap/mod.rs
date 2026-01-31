#[doc(hidden)]
pub mod no_drop_empty;

pub use no_drop_empty::NoDropEmpty as NoDrop;
pub use no_drop_empty::NoDropEmpty;

#[doc(hidden)]
#[cfg(feature = "alloc")]
pub mod no_drop_msg;

#[cfg(feature = "alloc")]
pub use no_drop_msg::NoDropMsg;

#[doc(hidden)]
pub mod no_drop_no_op;

// Type aliases for no_op variants
pub type NoDropNoOpEmpty<T = ()> = no_drop_no_op::NoDropNoOp<'static, T, crate::markers::NoMsg>;

#[cfg(feature = "alloc")]
pub type NoDropNoOpMsg<'msg, T = ()> = no_drop_no_op::NoDropNoOp<'msg, T, crate::markers::Msg>;
