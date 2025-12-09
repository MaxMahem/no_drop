use std::mem::ManuallyDrop;
use std::ptr;

/// Trait for types that require explicit consumption before dropping.
///
/// This trait provides a safe interface for working with values that must be
/// explicitly consumed via [`Consume::consume()`] rather than dropped implicitly.
///
/// This trait is automatically in scope when you import a `NoDrop` type from
/// either the `dbg` or `release` modules.
pub trait Consume: Sized {
    /// The type of the inner value being wrapped.
    type Inner;

    /// Creates a new wrapper around `value`.
    fn new(value: Self::Inner) -> Self;

    /// Consumes the wrapper and returns the `Inner` value.
    ///
    /// This is the only safe way to extract the value from the wrapper.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use no_drop::dbg::{Consume, NoDrop};
    ///
    /// let wrapper = NoDrop::new(42);
    /// assert_eq!(wrapper.consume(), 42);
    /// ```
    fn consume(self) -> Self::Inner;
}

/// A wrapper around a `T` value that always panics if dropped without being
/// [`Consume::consume`]d.
///
/// This type uses `unsafe` code to ensure the inner value is only extracted via
/// [`Consume::consume`]. If dropped normally, it will [`panic!`].
#[derive(Debug)]
#[repr(transparent)]
pub struct NoDrop<T>(T);

impl<T> Drop for NoDrop<T> {
    /// [`panic!`]s.
    #[track_caller]
    fn drop(&mut self) {
        panic!("Value was dropped without being consumed");
    }
}

impl<T> Consume for NoDrop<T> {
    type Inner = T;

    fn new(value: T) -> Self {
        Self(value)
    }

    #[inline]
    fn consume(self) -> T {
        let this = ManuallyDrop::new(self);
        unsafe { ptr::read(&raw const this.0) }
    }
}

/// A zero-cost wrapper with no drop checking.
///
/// This is a transparent no-op wrapper around the `T` value. It does not panic when
/// dropped. Intended to be transparently substituted for [`NoDrop`] in release builds.
#[derive(Debug)]
#[doc(hidden)]
#[allow(dead_code)]
#[repr(transparent)]
pub struct NoDropPassthrough<T>(T);

impl<T> Consume for NoDropPassthrough<T> {
    type Inner = T;

    fn new(value: T) -> Self {
        Self(value)
    }

    fn consume(self) -> T {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_drop() {
        let wrapper = NoDrop::new(42);
        assert_eq!(wrapper.consume(), 42);
    }

    #[test]
    fn no_drop_passthrough() {
        let wrapper = NoDropPassthrough::new(42);
        assert_eq!(wrapper.consume(), 42);
    }
}
