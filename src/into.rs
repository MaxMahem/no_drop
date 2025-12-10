use std::borrow::Cow;

use crate::no_drop::{NoDrop, NoDropPassthrough};
use crate::no_drop_msg::{NoDropMsg, NoDropMsgPassthrough};

/// Extension trait for wrapping values in [`NoDropPassthrough`] or [`NoDropMsgPassthrough`].
///
/// This is the "dbg" version that returns zero-cost passthrough wrappers.
#[allow(dead_code)]
pub trait IntoNoDropDbg: Sized {
    /// Wraps this value in a [`NoDropPassthrough`].
    fn no_drop(self) -> NoDropPassthrough<Self>;

    /// Wraps this value in a [`NoDropMsgPassthrough`] with a custom message.
    fn no_drop_msg<'msg, M: Into<Cow<'msg, str>>>(self, msg: M) -> NoDropMsgPassthrough<'msg, Self>;
}

impl<T> IntoNoDropDbg for T {
    fn no_drop(self) -> NoDropPassthrough<Self> {
        NoDropPassthrough::wrap(self)
    }

    fn no_drop_msg<'msg, M: Into<Cow<'msg, str>>>(self, msg: M) -> NoDropMsgPassthrough<'msg, Self> {
        NoDropMsgPassthrough::wrap(self, msg)
    }
}

/// Extension trait for wrapping values in [`NoDrop`] or [`NoDropMsg`].
///
/// This is the "rls" version that always returns panicking wrappers.
#[allow(dead_code)]
pub trait IntoNoDropRls: Sized {
    /// Wraps this value in a [`NoDrop`].
    fn no_drop(self) -> NoDrop<Self>;

    /// Wraps this value in a [`NoDropMsg`] with a custom `msg`.
    fn no_drop_msg<'msg, M: Into<Cow<'msg, str>>>(self, msg: M) -> NoDropMsg<'msg, Self>;
}

impl<T> IntoNoDropRls for T {
    fn no_drop(self) -> NoDrop<Self> {
        NoDrop::wrap(self)
    }

    fn no_drop_msg<'msg, M: Into<Cow<'msg, str>>>(self, msg: M) -> NoDropMsg<'msg, Self> {
        NoDropMsg::wrap(self, msg)
    }
}
