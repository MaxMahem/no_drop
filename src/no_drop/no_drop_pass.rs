use std::borrow::Cow;

use crate::markers::{Empty, Msg, PassMarker};

/// A zero-cost wrapper with no drop checking.
///
/// This is a transparent no-op wrapper. It does not [`panic!`] when dropped.
/// Intended to be transparently substituted for [`NoDropEmpty`](super::NoDropEmpty)
/// or [`NoDropMsg`](super::NoDropMsg) in release builds.
///
/// The type parameter `M` is a zero-sized marker that distinguishes between
/// the plain variant ([`Empty`]) and the message variant ([`Msg`]).
#[derive(
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    derive_more::Deref,
    derive_more::DerefMut,
    derive_more::AsMut,
    derive_more::AsRef,
)]
#[doc(hidden)]
#[must_use]
pub struct NoDropPass<'msg, M: PassMarker = Empty, T = ()> {
    #[deref]
    #[deref_mut]
    #[as_mut]
    #[as_ref]
    value: T,
    _lifetime: std::marker::PhantomData<&'msg ()>,
    _marker: std::marker::PhantomData<M>,
}

// Implementation for NoDropPass<Empty, T> (no message variant)
#[allow(dead_code)]
impl<T> NoDropPass<'static, Empty, T> {
    /// Creates a new wrapper around `value`.
    pub fn wrap(value: T) -> Self {
        Self { value, _lifetime: std::marker::PhantomData, _marker: std::marker::PhantomData }
    }

    /// Consumes the wrapper and returns the inner `T`.
    #[inline]
    #[must_use]
    pub fn unwrap(self) -> T {
        self.value
    }

    /// Forgets this guard, safely dropping it.
    pub fn forget(self) {
        drop(self);
    }
}

#[allow(dead_code)]
impl NoDropPass<'static, Empty, ()> {
    /// Creates a new empty guard.
    pub const fn new() -> Self {
        Self { value: (), _lifetime: std::marker::PhantomData, _marker: std::marker::PhantomData }
    }
}

impl Default for NoDropPass<'static, Empty, ()> {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for NoDropPass<'static, Empty, ()> {
    fn clone(&self) -> Self {
        Self::new()
    }
}

// Implementation for NoDropPass<Msg, T> (message variant)
#[allow(dead_code)]
impl<'msg, T> NoDropPass<'msg, Msg, T> {
    /// Creates a new wrapper around `value` with a custom panic message.
    ///
    /// The message is immediately dropped and ignored, since this type never [`panic!`]s.
    pub fn wrap<M: Into<Cow<'msg, str>>>(value: T, _msg: M) -> Self {
        Self { value, _lifetime: std::marker::PhantomData, _marker: std::marker::PhantomData }
    }

    /// Consumes the wrapper and returns the inner `T`.
    #[inline]
    #[must_use]
    pub fn unwrap(self) -> T {
        self.value
    }

    /// Forgets this guard, safely dropping it.
    pub fn forget(self) {
        drop(self);
    }
}

#[allow(dead_code)]
impl<'msg> NoDropPass<'msg, Msg, ()> {
    /// Creates a new empty no drop guard, with a custom panic message.
    ///
    /// The message is immediately dropped and ignored, since this type never [`panic!`]s.
    pub fn guard<M: Into<Cow<'msg, str>>>(_msg: M) -> Self {
        Self { value: (), _lifetime: std::marker::PhantomData, _marker: std::marker::PhantomData }
    }
}

impl<'msg> Clone for NoDropPass<'msg, Msg, ()> {
    fn clone(&self) -> Self {
        Self { value: (), _lifetime: std::marker::PhantomData, _marker: std::marker::PhantomData }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::no_drop::test_macros::{test_clone, test_ctor, test_forget};

    // Tests for Empty variant
    test_ctor!(passthrough_empty_wrap_consume, NoDropPass::<Empty, _>::wrap, (42), 42);
    test_ctor!(passthrough_empty_new, NoDropPass::<Empty, ()>::new, (), ());
    test_ctor!(passthrough_empty_default, NoDropPass::<Empty, ()>::default, (), ());

    test_forget!(passthrough_empty_forget, NoDropPass::<Empty, _>::wrap, (42));

    test_clone!(passthrough_empty_clone, NoDropPass<'static, Empty, ()>, NoDropPass::<Empty, ()>::new, ());

    #[test]
    fn passthrough_empty_drop_no_panic() {
        let wrapper = NoDropPass::<Empty, _>::wrap(42);
        drop(wrapper); // No panic
    }

    // Tests for Msg variant
    test_ctor!(passthrough_msg_wrap_consume, NoDropPass::<Msg, _>::wrap, (42, "message"), 42);
    test_ctor!(passthrough_msg_guard, NoDropPass::<Msg, ()>::guard, ("expected message"), ());

    test_forget!(passthrough_msg_forget, NoDropPass::<Msg, _>::wrap, (42, "message"));

    test_clone!(passthrough_msg_clone, NoDropPass<'static, Msg, ()>, NoDropPass::<Msg, ()>::guard, ("message"));

    #[test]
    fn passthrough_msg_drop_no_panic() {
        let wrapper = NoDropPass::<Msg, _>::wrap(42, "should not panic");
        drop(wrapper); // No panic
    }

    #[test]
    fn passthrough_msg_borrowed() {
        let msg = String::from("borrowed message");
        let wrapper = NoDropPass::<Msg, _>::wrap(42, msg.as_str());
        assert_eq!(wrapper.unwrap(), 42);
    }
}
