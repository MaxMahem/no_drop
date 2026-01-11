use std::borrow::Cow;

use crate::wrap::{NoDropEmpty, NoDropMsg, NoDropNoOpEmpty, NoDropNoOpMsg};

/// Extension trait for wrapping values in [`NoDropNoOpEmpty`] or [`NoDropNoOpMsg`].
///
/// This is the "dbg" version that returns zero-cost no_op wrappers.
pub trait IntoNoDropDbg: Sized {
    /// Wraps this value in a [`NoDropNoOpEmpty`].
    fn no_drop(self) -> NoDropNoOpEmpty<Self>;

    /// Wraps this value in a [`NoDropNoOpMsg`] with a custom message.
    fn expect_no_drop<'msg, M: Into<Cow<'msg, str>>>(self, msg: M) -> NoDropNoOpMsg<'msg, Self>;
}

impl<T> IntoNoDropDbg for T {
    fn no_drop(self) -> NoDropNoOpEmpty<Self> {
        NoDropNoOpEmpty::wrap(self)
    }

    fn expect_no_drop<'msg, M: Into<Cow<'msg, str>>>(self, msg: M) -> NoDropNoOpMsg<'msg, Self> {
        NoDropNoOpMsg::wrap(self, msg)
    }
}

/// Extension trait for wrapping values in [`NoDropEmpty`] or [`NoDropMsg`].
///
/// This is the "rls" version that always returns panicking wrappers.
pub trait IntoNoDropRls: Sized {
    /// Wraps this value in a [`NoDropEmpty`].
    fn no_drop(self) -> NoDropEmpty<Self>;

    /// Wraps this value in a [`NoDropMsg`] with a custom `msg`.
    fn expect_no_drop<'msg, M: Into<Cow<'msg, str>>>(self, msg: M) -> NoDropMsg<'msg, Self>;
}

impl<T> IntoNoDropRls for T {
    fn no_drop(self) -> NoDropEmpty<Self> {
        NoDropEmpty::wrap(self)
    }

    fn expect_no_drop<'msg, M: Into<Cow<'msg, str>>>(self, msg: M) -> NoDropMsg<'msg, Self> {
        NoDropMsg::wrap(self, msg)
    }
}
