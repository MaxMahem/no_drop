use std::borrow::Cow;

use crate::markers::Msg;

/// A zero-cost wrapper with no drop checking.
///
/// This is a transparent no-op wrapper. It does not [`panic!`] when dropped.
/// Intended to be transparently substituted for [`NoDropMsg`](super::NoDropMsg)
/// in release builds.
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
pub struct NoDropNoOp<'msg, T = ()> {
    #[deref]
    #[deref_mut]
    #[as_mut]
    #[as_ref]
    value: T,
    _lifetime: std::marker::PhantomData<&'msg ()>,
    _marker: std::marker::PhantomData<Msg>,
}

#[allow(dead_code)]
impl<'msg, T> NoDropNoOp<'msg, T> {
    /// Internal constructor for guards.
    pub(crate) const fn new(value: T) -> Self {
        Self { value, _lifetime: std::marker::PhantomData, _marker: std::marker::PhantomData }
    }

    /// Creates a new wrapper around `value` with a custom panic message.
    ///
    /// The message is immediately dropped and ignored, since this type never [`panic!`]s.
    pub fn wrap<M: Into<Cow<'msg, str>>>(value: T, _msg: M) -> Self {
        Self::new(value)
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
impl<'msg> NoDropNoOp<'msg, ()> {
    /// Creates a new empty no drop guard, with a custom panic message.
    ///
    /// The message is immediately dropped and ignored, since this type never [`panic!`]s.
    pub fn guard<M: Into<Cow<'msg, str>>>(_msg: M) -> Self {
        Self::new(())
    }
}

impl<'msg> Clone for NoDropNoOp<'msg, ()> {
    fn clone(&self) -> Self {
        Self::new(())
    }
}
