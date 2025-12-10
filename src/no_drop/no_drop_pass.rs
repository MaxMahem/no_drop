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
    pub fn consume(self) -> T {
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
    pub fn consume(self) -> T {
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

    // Tests for Empty variant
    #[test]
    fn passthrough_empty_wrap_consume() {
        let wrapper = NoDropPass::<Empty, _>::wrap(42);
        assert_eq!(wrapper.consume(), 42);
    }

    #[test]
    fn passthrough_empty_new() {
        let wrapper = NoDropPass::<Empty, ()>::new();
        assert_eq!(*wrapper, ());
    }

    #[test]
    fn passthrough_empty_default() {
        let wrapper = NoDropPass::<Empty, ()>::default();
        assert_eq!(*wrapper, ());
    }

    #[test]
    fn passthrough_empty_forget() {
        let wrapper = NoDropPass::<Empty, _>::wrap(42);
        wrapper.forget();
    }

    #[test]
    fn passthrough_empty_clone() {
        let wrapper = NoDropPass::<Empty, ()>::new();
        let cloned = wrapper.clone();
        assert_eq!(wrapper, cloned);
    }

    #[test]
    fn passthrough_empty_drop_no_panic() {
        let wrapper = NoDropPass::<Empty, _>::wrap(42);
        drop(wrapper); // No panic
    }

    // Tests for Msg variant
    #[test]
    fn passthrough_msg_wrap_consume() {
        let wrapper = NoDropPass::<Msg, _>::wrap(42, "message");
        assert_eq!(wrapper.consume(), 42);
    }

    #[test]
    fn passthrough_msg_guard() {
        let wrapper = NoDropPass::<Msg, ()>::guard("expected message");
        assert_eq!(*wrapper, ());
    }

    #[test]
    fn passthrough_msg_forget() {
        let wrapper = NoDropPass::<Msg, _>::wrap(42, "message");
        wrapper.forget();
    }

    #[test]
    fn passthrough_msg_clone() {
        let wrapper = NoDropPass::<Msg, ()>::guard("message");
        let cloned = wrapper.clone();
        assert_eq!(wrapper, cloned);
    }

    #[test]
    fn passthrough_msg_drop_no_panic() {
        let wrapper = NoDropPass::<Msg, _>::wrap(42, "should not panic");
        drop(wrapper); // No panic
    }

    #[test]
    fn passthrough_msg_borrowed() {
        let msg = String::from("borrowed message");
        let wrapper = NoDropPass::<Msg, _>::wrap(42, msg.as_str());
        assert_eq!(wrapper.consume(), 42);
    }
}
