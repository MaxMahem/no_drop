#![no_std]
#![doc = include_str!("../README.md")]
#![warn(clippy::pedantic, clippy::cargo, clippy::nursery)]
#![warn(missing_docs, missing_debug_implementations)]
#![allow(clippy::match_bool, clippy::single_match_else)]

#[cfg(feature = "alloc")]
extern crate alloc;

/// Internal default panic message used by Empty variant types.
pub(crate) const DEFAULT_DROP_PANIC_MSG: &str = "Value was dropped without being unwrapped";

#[doc(hidden)]
pub mod guards;

#[doc(hidden)]
pub mod markers;

#[doc(hidden)]
pub mod wrap;

/// Module containing [`NoDrop`](wrap::NoDropEmpty) and [`NoDropMsg`](wrap::NoDropMsg)
/// with debug-only panic behavior.
pub mod dbg {
    pub use crate::guards::GuardNotArmed;

    #[cfg(debug_assertions)]
    pub use crate::wrap::NoDrop;

    #[cfg(not(debug_assertions))]
    pub use crate::wrap::NoDropNoOpEmpty as NoDropEmpty;

    #[cfg(feature = "alloc")]
    #[cfg(debug_assertions)]
    pub use crate::wrap::NoDropMsg;

    #[cfg(feature = "alloc")]
    #[cfg(not(debug_assertions))]
    pub use crate::wrap::NoDropNoOpMsg as NoDropMsg;

    #[cfg(debug_assertions)]
    pub use crate::guards::DropGuardEmpty;

    #[cfg(not(debug_assertions))]
    pub use crate::guards::DropGuardNoOpEmpty as DropGuardEmpty;

    #[cfg(feature = "alloc")]
    #[cfg(debug_assertions)]
    pub use crate::guards::DropGuard;

    #[cfg(feature = "alloc")]
    #[cfg(not(debug_assertions))]
    pub use crate::guards::DropGuardNoOpMsg as DropGuard;
}

/// Module containing [`NoDrop`](wrap::NoDropEmpty) and [`NoDropMsg`](wrap::NoDropMsg) with always-[`panic!`]ing behavior.
pub mod rls {
    pub use crate::guards::GuardNotArmed;

    pub use crate::wrap::NoDrop;

    #[cfg(feature = "alloc")]
    pub use crate::wrap::NoDropMsg;

    pub use crate::guards::DropGuardEmpty;

    #[cfg(feature = "alloc")]
    pub use crate::guards::DropGuard;
}
