#![doc = include_str!("../README.md")]

#[warn(clippy::pedantic)]
#[warn(clippy::cargo)]
mod guards;
mod into;
mod markers;
mod no_drop;

#[cfg(test)]
mod test_macros;

/// Module containing [`NoDrop`](no_drops::NoDropEmpty) and [`NoDropMsg`](no_drops::NoDropMsg)
/// with debug-only panic behavior.
///
/// In debug builds, dropping without calling [`consume`](NoDropMsg::consume)
/// on the value will [`panic!`]. In release builds, this is a zero-cost wrapper with no checks.
pub mod dbg {
    #[cfg(debug_assertions)]
    pub use crate::no_drop::NoDropEmpty;

    #[cfg(not(debug_assertions))]
    pub use crate::no_drop::NoDropPassEmpty as NoDropEmpty;

    #[cfg(debug_assertions)]
    pub use crate::into::IntoNoDropRls as IntoNoDrop;

    #[cfg(not(debug_assertions))]
    pub use crate::into::IntoNoDropDbg as IntoNoDrop;

    #[cfg(debug_assertions)]
    pub use crate::no_drop::NoDropMsg;

    #[cfg(not(debug_assertions))]
    pub use crate::no_drop::NoDropPassMsg as NoDropMsg;

    pub use NoDropEmpty as NoDrop;

    #[cfg(debug_assertions)]
    pub use crate::guards::DropGuardEmpty;

    #[cfg(not(debug_assertions))]
    pub use crate::guards::DropGuardPassthroughEmpty as DropGuardEmpty;

    #[cfg(debug_assertions)]
    pub use crate::guards::DropGuardMsg as DropGuard;

    #[cfg(not(debug_assertions))]
    pub use crate::guards::DropGuardPassthroughMsg as DropGuard;
}

/// Module containing [`NoDrop`](no_drops::NoDropEmpty) and [`NoDropMsg`](no_drops::NoDropMsg) with always-[`panic!`]ing behavior.
///
/// In all builds (debug and release), dropping without calling [`consume`](NoDropMsg::consume) on
/// the value will [`panic!`].
pub mod rls {
    pub use crate::no_drop::NoDropEmpty;

    pub use crate::into::IntoNoDropRls as IntoNoDrop;

    pub use crate::no_drop::NoDropMsg;

    pub use NoDropEmpty as NoDrop;

    pub use crate::guards::DropGuardEmpty;

    pub use crate::guards::DropGuardMsg as DropGuard;
}
