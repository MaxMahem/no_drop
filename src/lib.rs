#![doc = include_str!("../README.md")]

/// Internal default panic message used by Empty variant types.
pub(crate) const DEFAULT_DROP_PANIC_MSG: &str = "Value was dropped without being unwrapped";

#[warn(clippy::pedantic)]
#[warn(clippy::nursery)]
#[warn(clippy::cargo)]
#[warn(missing_docs)]
#[warn(missing_debug_implementations)]
#[allow(clippy::match_bool)]
#[allow(clippy::single_match_else)]
#[doc(hidden)]
pub mod guards;
#[doc(hidden)]
pub mod into;
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

    #[cfg(debug_assertions)]
    pub use crate::into::IntoNoDropRls as IntoNoDrop;

    #[cfg(not(debug_assertions))]
    pub use crate::into::IntoNoDropDbg as IntoNoDrop;

    #[cfg(debug_assertions)]
    pub use crate::wrap::NoDropMsg;

    #[cfg(not(debug_assertions))]
    pub use crate::wrap::NoDropNoOpMsg as NoDropMsg;

    #[cfg(debug_assertions)]
    pub use crate::guards::DropGuardEmpty;

    #[cfg(not(debug_assertions))]
    pub use crate::guards::DropGuardNoOpEmpty as DropGuardEmpty;

    #[cfg(debug_assertions)]
    pub use crate::guards::DropGuard;

    #[cfg(not(debug_assertions))]
    pub use crate::guards::DropGuardNoOpMsg as DropGuard;
}

/// Module containing [`NoDrop`](wrap::NoDropEmpty) and [`NoDropMsg`](wrap::NoDropMsg) with always-[`panic!`]ing behavior.
pub mod rls {
    pub use crate::guards::GuardNotArmed;

    pub use crate::wrap::NoDrop;

    pub use crate::into::IntoNoDropRls as IntoNoDrop;

    pub use crate::wrap::NoDropMsg;

    pub use crate::guards::DropGuardEmpty;

    pub use crate::guards::DropGuard;
}
