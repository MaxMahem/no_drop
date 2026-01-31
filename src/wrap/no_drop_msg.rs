use alloc::borrow::Cow;
use core::mem::ManuallyDrop;

use crate::DEFAULT_DROP_PANIC_MSG;

/// A wrapper around a `T` `value` with a custom panic `msg` and will [`panic!`]s if dropped without being
/// [`Self::unwrap`]ped or [`Self::forget`]ten.
///
/// The lifetime parameter `'msg` allows borrowing the message, and most commonly will be `'static`.
#[derive(
    Debug,
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
#[must_use]
pub struct NoDropMsg<'msg, T = ()> {
    #[deref]
    #[deref_mut]
    #[as_mut]
    #[as_ref]
    value: T,
    msg: Cow<'msg, str>,
}

impl<'msg, T> NoDropMsg<'msg, T> {
    /// Panic message if type is default constructed
    pub const DEFAULT_PANIC_MSG: &'static str = DEFAULT_DROP_PANIC_MSG;

    /// Creates a new wrapper around `value` with a custom [`panic!`] `msg`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use no_drop::rls::NoDropMsg;
    ///
    /// let wrapper = NoDropMsg::wrap(42, "forgot to process this value");
    /// assert_eq!(wrapper.unwrap(), 42);
    /// ```
    #[inline]
    pub fn wrap<M: Into<Cow<'msg, str>>>(value: T, msg: M) -> Self {
        Self { value, msg: msg.into() }
    }

    /// Consumes the wrapper and returns the inner `T`.
    #[inline]
    #[must_use]
    pub fn unwrap(self) -> T {
        let this = ManuallyDrop::new(self);
        // SAFETY: `T` is moved out of the wrapper exactly once, then this is dropped.
        // No uninitialized access can occur.
        unsafe { core::ptr::read(&raw const this.value) }
    }

    /// Forgets this guard, safely dropping it.
    #[inline]
    pub const fn forget(self) {
        let _ = ManuallyDrop::new(self);
    }
}

impl<'msg> NoDropMsg<'msg, ()> {
    /// Creates a new empty [`NoDropMsg`] guard with a custom panic `msg`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use no_drop::rls::NoDropMsg;
    ///
    /// let wrapper = NoDropMsg::guard("this should be consumed");
    /// wrapper.forget();
    /// ```
    pub fn guard<M: Into<Cow<'msg, str>>>(msg: M) -> Self {
        Self { value: (), msg: msg.into() }
    }

    /// Consumes the guard and returns the inner panic message.
    pub(crate) fn unwrap_msg(self) -> Cow<'msg, str> {
        let this = ManuallyDrop::new(self);
        // SAFETY: `msg` is moved out of the wrapper exactly once, then this is dropped.
        // No uninitialized access can occur.
        unsafe { core::ptr::read(&raw const this.msg) }
    }

    /// Sets a new panic message, consuming the old guard.
    pub(crate) fn set_msg<'new, M: Into<Cow<'new, str>>>(self, msg: M) -> NoDropMsg<'new, ()> {
        self.forget();
        NoDropMsg::guard(msg)
    }
}

impl<T: Default> Default for NoDropMsg<'_, T> {
    fn default() -> Self {
        Self { value: T::default(), msg: Cow::from(Self::DEFAULT_PANIC_MSG) }
    }
}

impl<T> Drop for NoDropMsg<'_, T> {
    /// [`panic!`]s with `msg`.
    #[track_caller]
    fn drop(&mut self) {
        panic!("{}", self.msg);
    }
}
