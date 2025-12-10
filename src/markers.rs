use sealed::sealed;

/// Sealed trait for passthrough marker types.
///
/// This trait is sealed and cannot be implemented outside this crate.
#[sealed]
pub trait PassMarker {}

/// Marker type for passthrough types without custom message.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Empty;

#[sealed]
impl PassMarker for Empty {}

/// Marker type for passthrough types with custom message.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Msg;

#[sealed]
impl PassMarker for Msg {}
