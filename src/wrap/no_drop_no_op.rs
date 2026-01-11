use std::borrow::Cow;
use std::marker::PhantomData;

use crate::DEFAULT_DROP_PANIC_MSG;
use crate::markers::{Msg, MsgMarker, NoMsg};

/// A zero-cost wrapper with no drop checking.
///
/// This is a transparent no-op wrapper. It does not [`panic!`] when dropped.
/// Intended to be transparently substituted for [`NoDrop`](super::NoDrop) or
/// [`NoDropMsg`](super::NoDropMsg) in release builds.
///
/// The type parameter `M` is a zero-sized marker that distinguishes between the plain variant
/// ([`NoMsg`]) and the message variant ([`Msg`]).
#[derive(
    Debug,
    Default,
    Clone,
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
pub struct NoDropNoOp<'msg, T = (), M: MsgMarker = NoMsg> {
    #[deref]
    #[deref_mut]
    #[as_mut]
    #[as_ref]
    value: T,
    _lifetime: PhantomData<&'msg ()>,
    _marker: PhantomData<M>,
}

// Implementation for NoDropNoOp<T, NoMsg> (no message variant)
impl<T> NoDropNoOp<'static, T, NoMsg> {
    /// Default panic message that would be used if this type panicked (it doesn't).
    pub const PANIC_MSG: &'static str = DEFAULT_DROP_PANIC_MSG;

    /// Creates a new wrapper around `value`.
    pub fn wrap(value: T) -> Self {
        Self { value, _lifetime: PhantomData, _marker: PhantomData }
    }
}

// Implementation for NoDropNoOp<(), NoMsg> (empty no message variant)
impl NoDropNoOp<'static, (), NoMsg> {
    /// Creates a new empty guard.
    pub const fn guard() -> Self {
        Self::EMPTY
    }
}

// Implementation for NoDropNoOp<T, Msg> (message variant)
impl<'msg, T> NoDropNoOp<'msg, T, Msg> {
    /// Panic message if type is default constructed
    pub const DEFAULT_PANIC_MSG: &'static str = DEFAULT_DROP_PANIC_MSG;

    /// Creates a new wrapper around `value` with a custom panic message.
    ///
    /// The message is immediately dropped and ignored, since this type never [`panic!`]s.
    pub fn wrap<M: Into<Cow<'msg, str>>>(value: T, _msg: M) -> Self {
        Self::new(value)
    }
}

// Implementation for NoDropNoOp<(), Msg> (empty message variant)
impl<'msg> NoDropNoOp<'msg, (), Msg> {
    /// Creates a new empty no drop guard, with a custom panic message.
    ///
    /// The message is immediately dropped and ignored, since this type never [`panic!`]s.
    pub fn guard<M: Into<Cow<'msg, str>>>(_msg: M) -> Self {
        Self::EMPTY
    }
}

// Shared implementation for all variants
impl<'msg, T, M: MsgMarker> NoDropNoOp<'msg, T, M> {
    /// Internal constructor for guards.
    pub(crate) const fn new(value: T) -> Self {
        Self { value, _lifetime: PhantomData, _marker: PhantomData }
    }

    /// Consumes the wrapper and returns the inner `T`.
    #[must_use]
    pub fn unwrap(self) -> T {
        self.value
    }

    /// Forgets this guard, safely dropping it.
    pub fn forget(self) {
        drop(self);
    }
}

impl<'msg, M: MsgMarker> NoDropNoOp<'msg, (), M> {
    const EMPTY: Self = Self::new(());
}
