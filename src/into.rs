use std::borrow::Cow;

use crate::no_drop::{NoDropEmpty, NoDropMsg, NoDropPassEmpty, NoDropPassMsg};

/// Extension trait for wrapping values in [`NoDropPassEmpty`] or [`NoDropPassMsg`].
///
/// This is the "dbg" version that returns zero-cost passthrough wrappers.
#[allow(dead_code)]
pub trait IntoNoDropDbg: Sized {
    /// Wraps this value in a [`NoDropPassEmpty`].
    fn no_drop(self) -> NoDropPassEmpty<Self>;

    /// Wraps this value in a [`NoDropPassMsg`] with a custom message.
    fn expect_no_drop<'msg, M: Into<Cow<'msg, str>>>(self, msg: M) -> NoDropPassMsg<'msg, Self>;
}

impl<T> IntoNoDropDbg for T {
    fn no_drop(self) -> NoDropPassEmpty<Self> {
        NoDropPassEmpty::wrap(self)
    }

    fn expect_no_drop<'msg, M: Into<Cow<'msg, str>>>(self, msg: M) -> NoDropPassMsg<'msg, Self> {
        NoDropPassMsg::wrap(self, msg)
    }
}

/// Extension trait for wrapping values in [`NoDropEmpty`] or [`NoDropMsg`].
///
/// This is the "rls" version that always returns panicking wrappers.
#[allow(dead_code)]
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
