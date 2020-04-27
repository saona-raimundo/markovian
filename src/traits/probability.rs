use num_traits::{One, Zero};

/// Probability marker.
pub trait Probability: Zero + One + PartialOrd {}

impl<T> Probability for T where T: Zero + One + PartialOrd {}


bounds to be fractional ??