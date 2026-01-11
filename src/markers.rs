use sealed::sealed;

/// Sealed trait for passthrough marker types.
///
/// This trait is sealed and cannot be implemented outside this crate.
#[sealed]
pub trait MsgMarker {}

/// Marker type for passthrough types without custom message.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct NoMsg;

#[sealed]
impl MsgMarker for NoMsg {}

/// Marker type for passthrough types with custom message.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Msg;

#[sealed]
impl MsgMarker for Msg {}
