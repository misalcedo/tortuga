//! Representation of epsilon ranges within the Tortuga runtime.

use crate::runtime::Number;

/// A range centered around a value.
/// The start and end of the range are inclusive of the center plus and minus a value epsilon.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct EpsilonRange {
    center: Number,
    epsilon: Number,
}

impl EpsilonRange {
    /// Creates a new instance of an `EpsilonRange` around a given `center`.
    pub fn new(center: Number, epsilon: Number) -> Self {
        EpsilonRange { center, epsilon }
    }
}
