use crate::DEFAULT_DROP_PANIC_MSG;
use crate::markers::NoMsg;

/// A zero-cost wrapper with no drop checking.
///
/// This is a transparent no-op wrapper. It does not [`panic!`] when dropped.
/// Intended to be transparently substituted for [`NoDropEmpty`](super::NoDropEmpty)
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
pub struct NoDropNoOp<T = ()> {
    #[deref]
    #[deref_mut]
    #[as_mut]
    #[as_ref]
    value: T,
    _marker: std::marker::PhantomData<NoMsg>,
}

#[allow(dead_code)]
impl<T> NoDropNoOp<T> {
    /// Default panic message that would be used if this type panicked (it doesn't).
    pub const PANIC_MSG: &'static str = DEFAULT_DROP_PANIC_MSG;

    /// Creates a new wrapper around `value`.
    pub fn wrap(value: T) -> Self {
        Self { value, _marker: std::marker::PhantomData }
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
impl NoDropNoOp<()> {
    /// Creates a new empty guard.
    pub const fn new() -> Self {
        Self { value: (), _marker: std::marker::PhantomData }
    }
}

impl Default for NoDropNoOp<()> {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for NoDropNoOp<()> {
    fn clone(&self) -> Self {
        Self::new()
    }
}
