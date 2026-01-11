use crate::DEFAULT_DROP_PANIC_MSG;
use std::mem::ManuallyDrop;

/// A wrapper around a `T` value that always [`panic!`]s if dropped without being
/// [`Self::unwrap`]ed or [`Self::forget`]ten.
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
pub struct NoDropEmpty<T = ()>(T);

impl<T> NoDropEmpty<T> {
    /// Default panic message used when the value is dropped without being unwrapped.
    pub const PANIC_MSG: &'static str = DEFAULT_DROP_PANIC_MSG;

    /// Creates a new wrapper around `value`.
    pub fn wrap(value: T) -> Self {
        Self(value)
    }

    /// Consumes the wrapper and returns the inner `T`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use no_drop::rls::NoDrop;
    ///
    /// let wrapper = NoDrop::wrap(42);
    /// assert_eq!(wrapper.unwrap(), 42);
    /// ```
    #[inline]
    #[must_use]
    pub fn unwrap(self) -> T {
        let this = ManuallyDrop::new(self);
        // SAFETY: `T` is moved out of the wrapper exactly once, then this type is dropped.
        // No uninitialized access can occur.
        unsafe { std::ptr::read(&raw const this.0) }
    }

    /// Forgets this guard, safely dropping it.
    #[inline]
    pub fn forget(self) {
        let _ = ManuallyDrop::new(self);
    }
}

impl NoDropEmpty<()> {
    /// Creates a new empty [`NoDropEmpty`] guard.
    pub const fn new() -> Self {
        Self(())
    }
}

impl Default for NoDropEmpty<()> {
    fn default() -> Self {
        Self(())
    }
}

impl Clone for NoDropEmpty<()> {
    fn clone(&self) -> Self {
        Self::new()
    }
}

impl<T> Drop for NoDropEmpty<T> {
    /// [`panic!`]s.
    #[track_caller]
    fn drop(&mut self) {
        panic!("{}", DEFAULT_DROP_PANIC_MSG);
    }
}
