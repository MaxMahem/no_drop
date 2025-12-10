use std::mem::ManuallyDrop;
use std::ptr;

/// A wrapper around a `T` value that always [`panic!`]s if dropped without being
/// [`Self::consume`]d or [`Self::forget`].
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
pub struct NoDrop<T = ()>(T);

impl<T> NoDrop<T> {
    /// Creates a new wrapper around `value`.
    pub fn wrap(value: T) -> Self {
        Self(value)
    }

    /// Consumes the wrapper and returns the inner `T`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use no_drop::dbg::NoDrop;
    ///
    /// let wrapper = NoDrop::wrap(42);
    /// assert_eq!(wrapper.consume(), 42);
    /// ```
    #[inline]
    #[must_use]
    pub fn consume(self) -> T {
        let this = ManuallyDrop::new(self);
        // SAFETY: `T` is moved out of the wrapper exactly once, then this type is dropped.
        // No uninitialized access can occur.
        unsafe { ptr::read(&raw const this.0) }
    }

    /// Forgets this guard, safely dropping it.
    #[inline]
    pub fn forget(self) {
        let _ = ManuallyDrop::new(self);
    }
}

impl NoDrop<()> {
    /// Creates a new empty [`NoDrop`] guard.
    pub const fn new() -> Self {
        Self(())
    }
}

impl Default for NoDrop<()> {
    fn default() -> Self {
        Self(())
    }
}

impl<T> Drop for NoDrop<T> {
    /// [`panic!`]s.
    #[track_caller]
    fn drop(&mut self) {
        panic!("Value was dropped without being consumed");
    }
}

/// A zero-cost wrapper with no drop checking.
///
/// This is a transparent no-op wrapper around the `T` value. It does not panic when
/// dropped. Intended to be transparently substituted for [`NoDrop`] in release builds.
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
#[doc(hidden)]
#[must_use]
pub struct NoDropPassthrough<T = ()>(T);

#[allow(dead_code)]
impl<T> NoDropPassthrough<T> {
    /// Creates a new wrapper around `value`.
    pub fn wrap(value: T) -> Self {
        Self(value)
    }

    /// Consumes the wrapper and returns the inner `T`.
    #[inline]
    #[must_use]
    pub fn consume(self) -> T {
        self.0
    }

    /// Forgets this guard, safely dropping it.
    pub fn forget(self) {
        drop(self);
    }
}

#[allow(dead_code)]
impl NoDropPassthrough<()> {
    /// Creates a new empty guard.
    pub const fn new() -> Self {
        Self(())
    }
}

impl Default for NoDropPassthrough<()> {
    fn default() -> Self {
        Self(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::into::{IntoNoDropDbg, IntoNoDropRls};
    use crate::{test_ctor, test_forget};

    #[test]
    #[should_panic(expected = "Value was dropped without being consumed")]
    fn no_drop_panics() {
        let wrapper = NoDrop::wrap(42);
        drop(wrapper);
    }

    test_ctor!(no_drop, NoDrop::wrap, (42), 42);
    test_ctor!(no_drop_passthrough, NoDropPassthrough::wrap, (42), 42);
    test_ctor!(into_no_drop_dbg_trait, IntoNoDropDbg::no_drop, (42), 42);
    test_ctor!(into_no_drop_rls_trait, IntoNoDropRls::no_drop, (42), 42);
    test_ctor!(no_drop_new, NoDrop::new, (), ());
    test_ctor!(no_drop_default, NoDrop::default, (), ());
    test_ctor!(no_drop_passthrough_new, NoDropPassthrough::new, (), ());
    test_ctor!(no_drop_passthrough_default, NoDropPassthrough::default, (), ());

    test_forget!(no_drop_forget, NoDrop::new, ());
    test_forget!(no_drop_passthrough_forget, NoDropPassthrough::new, ());
}
