#![doc = include_str!("../README.md")]

#[warn(clippy::pedantic)]
#[warn(clippy::cargo)]
mod guards;
mod into;
mod markers;
mod no_drop;

/// Module containing [`NoDrop`](no_drop::NoDropEmpty) and [`NoDropMsg`](no_drop::NoDropMsg)
/// with debug-only panic behavior.
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

/// Module containing [`NoDrop`](no_drop::NoDropEmpty) and [`NoDropMsg`](no_drop::NoDropMsg) with always-[`panic!`]ing behavior.
pub mod rls {
    pub use crate::no_drop::NoDropEmpty;

    pub use crate::into::IntoNoDropRls as IntoNoDrop;

    pub use crate::no_drop::NoDropMsg;

    pub use NoDropEmpty as NoDrop;

    pub use crate::guards::DropGuardEmpty;

    pub use crate::guards::DropGuardMsg as DropGuard;
}
