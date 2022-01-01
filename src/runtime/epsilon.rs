//! The epsilon operator trait for Tortuga.

/// A number with a tolerance (or margin of error).
pub trait Epsilon<Rhs = Self> {
    type Output;

    /// Transforms this instance into one with a numerical [`Tolerance`].
    fn epsilon(self, rhs: Rhs) -> Self::Output;
}
