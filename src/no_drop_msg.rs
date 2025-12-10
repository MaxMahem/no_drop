use std::borrow::Cow;
use std::mem::ManuallyDrop;
use std::ptr;

/// A wrapper around a `T` `value` with a custom panic `msg` and will [`panic!`]s if dropped without being
/// [`Self::consume`]d or [`Self::forget`].
///
/// The lifetime parameter `'msg` allows borrowing the message, and most commonly will be `'static`.
#[derive(Debug, derive_more::Deref, derive_more::DerefMut)]
#[must_use]
pub struct NoDropMsg<'msg, T = ()> {
    #[deref]
    #[deref_mut]
    value: T,
    msg: Cow<'msg, str>,
}

impl<'msg, T> NoDropMsg<'msg, T> {
    /// Creates a new wrapper around `value` with a custom panic `msg`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use no_drop::rls::NoDropMsg;
    ///
    /// let wrapper = NoDropMsg::wrap_msg(42, "forgot to process this value");
    /// assert_eq!(wrapper.consume(), 42);
    /// ```
    pub fn wrap_msg<M: Into<Cow<'msg, str>>>(value: T, msg: M) -> Self {
        Self { value, msg: msg.into() }
    }

    /// Consumes the wrapper and returns `value`.
    #[inline]
    #[must_use]
    pub fn consume(self) -> T {
        let this = ManuallyDrop::new(self);
        unsafe { ptr::read(&raw const this.value) }
    }

    /// Forgets the value, allowing it to be dropped.
    #[inline]
    pub fn forget(self) {
        let _ = ManuallyDrop::new(self);
    }
}

impl<'msg, T> Drop for NoDropMsg<'msg, T> {
    /// [`panic!`]s with `msg`.
    #[track_caller]
    fn drop(&mut self) {
        panic!("{}", self.msg);
    }
}

/// A zero-cost wrapper with no drop checking.
///
/// This is a transparent wrapper around the `T` value. It does not panic when dropped.
/// Transparently substituted for [`NoDropMsg`] in release builds.
#[derive(Debug, derive_more::Deref, derive_more::DerefMut)]
#[doc(hidden)]
#[must_use]
pub struct NoDropMsgPassthrough<'msg, T = ()> {
    #[deref]
    #[deref_mut]
    value: T,
    _lifetime: std::marker::PhantomData<&'msg ()>,
}

#[allow(dead_code)]
impl<'msg, T> NoDropMsgPassthrough<'msg, T> {
    /// Creates a new wrapper around `value` with a message. The message is ignored.
    pub fn wrap_msg<M: Into<Cow<'msg, str>>>(value: T, _msg: M) -> Self {
        Self { value, _lifetime: std::marker::PhantomData }
    }

    /// Consumes the wrapper and returns the inner value.
    #[inline]
    #[must_use]
    pub fn consume(self) -> T {
        self.value
    }

    /// Forgets the value, allowing it to be dropped.
    pub fn forget(self) {
        drop(self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::into::{IntoNoDropDbg, IntoNoDropRls};
    use crate::{test_ctor, test_forget};

    #[test]
    #[should_panic(expected = "custom panic message")]
    fn no_drop_msg_panics() {
        let wrapper = NoDropMsg::wrap_msg(42, "custom panic message");
        drop(wrapper);
    }

    test_ctor!(no_drop_msg_static_str, NoDropMsg::wrap_msg, (42, "custom message"), 42);
    test_ctor!(no_drop_msg_string, NoDropMsg::wrap_msg, (42, String::from("owned message")), 42);
    test_ctor!(no_drop_msg_passthrough, NoDropMsgPassthrough::wrap_msg, (42, "message"), 42);

    test_ctor!(into_no_drop_msg_dbg_trait, IntoNoDropDbg::no_drop_msg, (42, "msg"), 42);
    test_ctor!(into_no_drop_msg_rls_trait, IntoNoDropRls::no_drop_msg, (42, "msg"), 42);

    test_forget!(no_drop_msg_forget, NoDropMsg::wrap_msg, (42, "custom message"));
    test_forget!(no_drop_msg_passthrough_forget, NoDropMsgPassthrough::wrap_msg, (42, "custom message"));

    #[test]
    fn no_drop_msg_borrowed() {
        let msg = String::from("borrowed message");
        let wrapper = NoDropMsg::wrap_msg(42, msg.as_str());
        assert_eq!(wrapper.consume(), 42);
    }
}
