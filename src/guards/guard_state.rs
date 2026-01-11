/// Represents the state of a drop guard after an operation.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum GuardState {
    /// The guard is armed and will panic if dropped.
    Armed,
    /// The guard is disarmed and safe to drop.
    #[default]
    Disarmed,
}
