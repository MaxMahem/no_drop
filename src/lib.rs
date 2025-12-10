#![doc = include_str!("../README.md")]

#[warn(clippy::pedantic)]
#[warn(clippy::cargo)]
mod guards;
mod into;
mod no_drop;
mod no_drop_msg;

#[cfg(test)]
mod test_macros;

/// Module containing [`NoDrop`](no_drop::NoDrop) and [`NoDropMsg`](no_drop_msg::NoDropMsg) with debug-only panic behavior.
///
/// In debug builds, dropping without calling [`consume`](NoDrop::consume)
/// on the value will [`panic!`]. In release builds, this is a zero-cost wrapper with no checks.
pub mod dbg {
    #[cfg(debug_assertions)]
    pub use crate::no_drop::NoDrop;

    #[cfg(not(debug_assertions))]
    pub use crate::no_drop::NoDropPassthrough as NoDrop;

    #[cfg(debug_assertions)]
    pub use crate::into::IntoNoDropRls as IntoNoDrop;

    #[cfg(not(debug_assertions))]
    pub use crate::into::IntoNoDropDbg as IntoNoDrop;

    #[cfg(debug_assertions)]
    pub use crate::no_drop_msg::NoDropMsg;

    #[cfg(not(debug_assertions))]
    pub use crate::no_drop_msg::NoDropMsgPassthrough as NoDropMsg;

    #[cfg(debug_assertions)]
    pub use crate::guards::DropGuardEmpty;

    #[cfg(not(debug_assertions))]
    pub use crate::guards::DropGuardPassthroughEmpty as DropGuardEmpty;

    #[cfg(debug_assertions)]
    pub use crate::guards::DropGuardMsg as DropGuard;

    #[cfg(not(debug_assertions))]
    pub use crate::guards::DropGuardPassthroughMsg as DropGuard;
}

/// Module containing [`NoDrop`](no_drop::NoDrop) and [`NoDropMsg`](no_drop_msg::NoDropMsg) with always-[`panic!`]ing behavior.
///
/// In all builds (debug and release), dropping without calling [`consume`](NoDrop::consume) on
/// the value will [`panic!`].
pub mod rls {
    pub use crate::no_drop::NoDrop;

    pub use crate::into::IntoNoDropRls as IntoNoDrop;

    pub use crate::no_drop_msg::NoDropMsg;

    pub use crate::guards::DropGuardEmpty;

    pub use crate::guards::DropGuardMsg as DropGuard;
}
