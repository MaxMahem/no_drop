#![doc = include_str!("../README.md")]

#[warn(clippy::pedantic)]
#[warn(clippy::cargo)]
mod no_drop;

/// Module containing [`NoDrop`](no_drop::NoDrop) with debug-only panic behavior.
///
/// In debug builds, dropping without [`Consume::consume`](crate::no_drop::Consume)ing
/// the value will [`panic!`]. In release builds, this is a zero-cost wrapper with no checks.
pub mod dbg {
    pub use crate::no_drop::Consume;

    #[cfg(debug_assertions)]
    pub use crate::no_drop::NoDrop;

    #[cfg(not(debug_assertions))]
    pub use crate::no_drop::NoDropPassthrough as NoDrop;

    #[cfg(debug_assertions)]
    pub use crate::no_drop::IntoNoDropRls as IntoNoDrop;

    #[cfg(not(debug_assertions))]
    pub use crate::no_drop::IntoNoDropDbg as IntoNoDrop;
}

/// Module containing [`NoDrop`](no_drop::NoDrop) with always-[`panic!`]ing behavior.
///
/// In all builds (debug and release), dropping without
/// [`Consume::consume`](crate::no_drop::Consume)ing the value will [`panic!`].
pub mod rls {
    pub use crate::no_drop::Consume;
    pub use crate::no_drop::NoDrop;

    pub use crate::no_drop::IntoNoDropRls as IntoNoDrop;
}
