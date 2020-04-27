use rand::Rng;

pub trait ExponentialClock<T> {
    fn sample_period<R>(&self, rng: &mut R) -> T
    where
        R: Rng + ?Sized;
}
