use std::mem::ManuallyDrop;
use std::ptr;

/// A wrapper around a `T` value that always panics if dropped without being
/// [`consume`](Self::consume)d.
///
/// This type uses `unsafe` code to ensure the inner value is only extracted via
/// [`consume`](Self::consume). If dropped normally, it will [`panic!`].
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
pub struct NoDrop<T>(T);

impl<T> NoDrop<T> {
    /// Creates a new wrapper around `value`.
    pub fn wrap(value: T) -> Self {
        Self(value)
    }

    /// Consumes the wrapper and returns the inner value.
    ///
    /// This is the only safe way to extract the value from the wrapper.
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
        unsafe { ptr::read(&raw const this.0) }
    }

    /// Forgets the value, allowing it to be dropped.
    #[inline]
    pub fn forget(self) {
        let _ = ManuallyDrop::new(self);
    }
}

impl NoDrop<()> {
    /// Creates a new empty `NoDrop` value.
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
pub struct NoDropPassthrough<T>(T);

#[allow(dead_code)]
impl<T> NoDropPassthrough<T> {
    /// Creates a new wrapper around `value`.
    pub fn wrap(value: T) -> Self {
        Self(value)
    }

    /// Consumes the wrapper and returns the inner value.
    #[inline]
    #[must_use]
    pub fn consume(self) -> T {
        self.0
    }

    /// Forgets the value, allowing it to be dropped.
    pub fn forget(self) {
        drop(self);
    }
}

#[allow(dead_code)]
impl NoDropPassthrough<()> {
    /// Creates a new empty `NoDropPassthrough` value.
    pub const fn new() -> Self {
        Self(())
    }
}

impl Default for NoDropPassthrough<()> {
    fn default() -> Self {
        Self(())
    }
}

/// Extension trait for wrapping values in a [`NoDropPassthrough`].
///
/// This is the "dbg" version that returns a zero-cost passthrough wrapper.
#[allow(dead_code)]
pub trait IntoNoDropDbg: Sized {
    /// Wraps this value in a [`NoDropPassthrough`].
    fn no_drop(self) -> NoDropPassthrough<Self>;
}

impl<T> IntoNoDropDbg for T {
    fn no_drop(self) -> NoDropPassthrough<Self> {
        NoDropPassthrough::wrap(self)
    }
}

/// Extension trait for wrapping values in a [`NoDrop`].
///
/// This is the "rls" version that always returns a panicking wrapper.
#[allow(dead_code)]
pub trait IntoNoDropRls: Sized {
    /// Wraps this value in a [`NoDrop`].
    fn no_drop(self) -> NoDrop<Self>;
}

impl<T> IntoNoDropRls for T {
    fn no_drop(self) -> NoDrop<Self> {
        NoDrop::wrap(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_ctor {
        ($test_name:ident, $ctor:expr, ($($params:tt)*), $expected:expr) => {
            #[test]
            fn $test_name() {
                let wrapper = $ctor($($params)*);
                assert_eq!(wrapper.consume(), $expected);
            }
        };
    }

    macro_rules! test_forget {
        ($test_name:ident, $type:ty) => {
            #[test]
            fn $test_name() {
                let wrapper = <$type>::wrap(42);
                wrapper.forget();
            }
        };
    }

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

    test_forget!(no_drop_forget, NoDrop<i32>);
    test_forget!(no_drop_passthrough_forget, NoDropPassthrough<i32>);
}
