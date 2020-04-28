use rand::Rng;

pub trait ExponentialClock<T> {
	/// Should use #[inline].
    fn sample_period<R>(&self, rng: &mut R) -> T
    where
        R: Rng + ?Sized;
}
