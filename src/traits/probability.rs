use core::fmt::Debug;
use num_traits::{One, Zero, Float};

/// Probability marker.
pub trait Probability: Zero + One + PartialOrd + Debug + Float{}

impl<T> Probability for T where T: Zero + One + PartialOrd + Debug + Float {}


// bounds to be fractional ??