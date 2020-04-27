// Traits
use rand::Rng;

/// Types (distributions) that can be used only once to create a random instance of `T`.
///
/// It is possible to sample from a distribution through both the
/// `Distribution` and `Rng` traits, via `distr.sample(&mut rng)` and
/// `rng.sample(distr)`. They do not offer the `sample_iter` method.
///
pub trait DistributionOnce<T> {
    /// Generate a random value of `T`, using `rng` as the source of randomness.
    fn sample_once<R: Rng + ?Sized>(self, rng: &mut R) -> T;
}