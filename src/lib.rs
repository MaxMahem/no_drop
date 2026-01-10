#![doc = include_str!("../README.md")]

#[warn(clippy::pedantic)]
#[warn(clippy::nursery)]
#[warn(clippy::cargo)]
#[warn(missing_docs)]
#[allow(clippy::match_bool)]
mod guards;
mod into;
mod markers;

#[doc(hidden)]
pub mod wrap;

/// Module containing [`NoDrop`](wrap::NoDropEmpty) and [`NoDropMsg`](wrap::NoDropMsg)
/// with debug-only panic behavior.
pub mod dbg {
    pub use crate::guards::GuardNotArmed;
    pub use crate::wrap::DEFAULT_DROP_PANIC_MSG;

    #[cfg(debug_assertions)]
    pub use crate::wrap::NoDrop;

    #[cfg(not(debug_assertions))]
    pub use crate::wrap::NoDropPassEmpty as NoDropEmpty;

    #[cfg(debug_assertions)]
    pub use crate::into::IntoNoDropRls as IntoNoDrop;

    #[cfg(not(debug_assertions))]
    pub use crate::into::IntoNoDropDbg as IntoNoDrop;

    #[cfg(debug_assertions)]
    pub use crate::wrap::NoDropMsg;

    #[cfg(not(debug_assertions))]
    pub use crate::wrap::NoDropPassMsg as NoDropMsg;

    #[cfg(debug_assertions)]
    pub use crate::guards::DropGuardEmpty;

    #[cfg(not(debug_assertions))]
    pub use crate::guards::DropGuardPassthroughEmpty as DropGuardEmpty;

    #[cfg(debug_assertions)]
    pub use crate::guards::DropGuard;

    #[cfg(not(debug_assertions))]
    pub use crate::guards::DropGuardPassthroughMsg as DropGuard;
}

/// Module containing [`NoDrop`](no_drop::NoDropEmpty) and [`NoDropMsg`](no_drop::NoDropMsg) with always-[`panic!`]ing behavior.
pub mod rls {
    pub use crate::guards::GuardNotArmed;
    pub use crate::wrap::DEFAULT_DROP_PANIC_MSG;

    pub use crate::wrap::NoDrop;

    pub use crate::into::IntoNoDropRls as IntoNoDrop;

    pub use crate::wrap::NoDropMsg;

    pub use crate::guards::DropGuardEmpty;

    pub use crate::guards::DropGuard;
}
