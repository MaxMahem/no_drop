use crate::markers::{MsgMarker, NoMsg};
use core::marker::PhantomData;

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

// Shared implementation for all variants
impl<T, M: MsgMarker> NoDropNoOp<'_, T, M> {
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
    pub fn forget(self) {}
}

impl<M: MsgMarker> NoDropNoOp<'_, (), M> {
    pub(crate) const EMPTY: Self = Self::new(());
}
