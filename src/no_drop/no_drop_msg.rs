use std::borrow::Cow;
use std::mem::ManuallyDrop;

/// A wrapper around a `T` `value` with a custom panic `msg` and will [`panic!`]s if dropped without being
/// [`Self::unwrap`]ped or [`Self::forget`]ten.
///
/// The lifetime parameter `'msg` allows borrowing the message, and most commonly will be `'static`.
#[derive(
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Debug,
    derive_more::Deref,
    derive_more::DerefMut,
    derive_more::AsMut,
    derive_more::AsRef,
)]
#[must_use]
pub struct NoDropMsg<'msg, T = ()> {
    #[deref]
    #[deref_mut]
    #[as_mut]
    #[as_ref]
    value: T,
    msg: Cow<'msg, str>,
}

impl<'msg, T> NoDropMsg<'msg, T> {
    /// Creates a new wrapper around `value` with a custom [`panic!`] `msg`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use no_drop::rls::NoDropMsg;
    ///
    /// let wrapper = NoDropMsg::wrap(42, "forgot to process this value");
    /// assert_eq!(wrapper.unwrap(), 42);
    /// ```
    #[inline]
    pub fn wrap<M: Into<Cow<'msg, str>>>(value: T, msg: M) -> Self {
        Self { value, msg: msg.into() }
    }

    /// Consumes the wrapper and returns the inner `T`.
    #[inline]
    #[must_use]
    pub fn unwrap(self) -> T {
        let this = ManuallyDrop::new(self);
        // SAFETY: `T` is moved out of the wrapper exactly once, then this is dropped.
        // No uninitialized access can occur.
        unsafe { std::ptr::read(&raw const this.value) }
    }

    /// Forgets this guard, safely dropping it.
    #[inline]
    pub fn forget(self) {
        let _ = ManuallyDrop::new(self);
    }
}

impl<'msg> NoDropMsg<'msg, ()> {
    /// Creates a new empty [`NoDropMsg`] guard with a custom panic `msg`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use no_drop::rls::NoDropMsg;
    ///
    /// let wrapper = NoDropMsg::guard("this should be consumed");
    /// wrapper.forget();
    /// ```
    pub fn guard<M: Into<Cow<'msg, str>>>(msg: M) -> Self {
        Self { value: (), msg: msg.into() }
    }

    /// Consumes the guard and returns the inner panic message.
    pub(crate) fn unwrap_msg(self) -> Cow<'msg, str> {
        let this = ManuallyDrop::new(self);
        // SAFETY: `msg` is moved out of the wrapper exactly once, then this is dropped.
        // No uninitialized access can occur.
        unsafe { std::ptr::read(&raw const this.msg) }
    }
}

impl<'msg> Clone for NoDropMsg<'msg, ()> {
    fn clone(&self) -> Self {
        Self { value: (), msg: self.msg.clone() }
    }
}

impl<'msg, T> Drop for NoDropMsg<'msg, T> {
    /// [`panic!`]s with `msg`.
    #[track_caller]
    fn drop(&mut self) {
        panic!("{}", self.msg);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::into::{IntoNoDropDbg, IntoNoDropRls};
    use crate::no_drop::test_macros::{test_clone, test_ctor, test_forget};

    #[test]
    #[should_panic(expected = "custom panic message")]
    fn no_drop_msg_panics() {
        let wrapper = NoDropMsg::wrap(42, "custom panic message");
        drop(wrapper);
    }

    test_ctor!(no_drop_msg_static_str, NoDropMsg::wrap, (42, "custom message"), 42);
    test_ctor!(no_drop_msg_string, NoDropMsg::wrap, (42, String::from("owned message")), 42);

    test_ctor!(into_no_drop_msg_dbg_trait, IntoNoDropDbg::expect_no_drop, (42, "msg"), 42);
    test_ctor!(into_no_drop_msg_rls_trait, IntoNoDropRls::expect_no_drop, (42, "msg"), 42);

    test_ctor!(no_drop_msg_expect_static_str, NoDropMsg::guard, ("expected message"), ());
    test_ctor!(no_drop_msg_expect_string, NoDropMsg::guard, (String::from("owned expected message")), ());

    test_clone!(no_drop_clone, NoDropMsg, NoDropMsg::guard, ("custom message"));

    test_forget!(no_drop_msg_forget, NoDropMsg::wrap, (42, "custom message"));

    #[test]
    fn no_drop_msg_borrowed() {
        let msg = String::from("borrowed message");
        let wrapper = NoDropMsg::wrap(42, msg.as_str());
        assert_eq!(wrapper.unwrap(), 42);
    }

    #[test]
    #[should_panic(expected = "unit value must be consumed")]
    fn no_drop_msg_expect_panics() {
        let wrapper = NoDropMsg::guard("unit value must be consumed");
        drop(wrapper);
    }
}
