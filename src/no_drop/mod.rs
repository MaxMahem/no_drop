#[cfg(test)]
mod test_macros;

mod no_drop_empty;
mod no_drop_msg;
mod no_drop_pass;

pub use no_drop_empty::NoDropEmpty;
pub use no_drop_msg::NoDropMsg;

// Type aliases for passthrough variants
pub(crate) type NoDropPassEmpty<T = ()> = no_drop_pass::NoDropPass<'static, crate::markers::Empty, T>;
pub(crate) type NoDropPassMsg<'msg, T = ()> = no_drop_pass::NoDropPass<'msg, crate::markers::Msg, T>;
