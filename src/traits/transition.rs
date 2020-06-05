// Traits
use rand_distr::Distribution;
use rand::Rng;


/// Abstraction over transition matrix.
///
/// # Remarks
///  
/// Output parameter `O` allows sampling more than only the next state,
/// for example, continuous markov chains are able to sample both a new state
/// and a time step.
pub trait Transition<T, O> {
    fn sample_from<R>(&self, state: &T, rng: &mut R) -> O
    where
        R: Rng + ?Sized;
}

impl<T, O, F, D> Transition<T, O> for F
where
    F: Fn(&T) -> D,
    D: Distribution<O>,
{
    #[inline]
    fn sample_from<R>(&self, state: &T, rng: &mut R) -> O
    where
        R: Rng + ?Sized,
    {
        self(state).sample(rng)
    }
}

#[cfg(test)]
mod tests {
    use crate::distributions::Raw;
    use crate::traits::Transition;

    #[test]
    fn use_cases() {
        let mut rng = crate::tests::rng(1);
        let expected = 1;
        fn transition(_: &u64) -> Raw<Vec<(f64, u64)>> {
            Raw::new(vec![(1.0, 1)])
        };

        assert_eq!(transition.sample_from(&0, &mut rng), expected);

        let transition = |_: &u64| Raw::new(vec![(1.0, expected)]);

        assert_eq!(transition.sample_from(&0, &mut rng), expected);
    }

    #[test]
    fn construction() {
        let mut rng = crate::tests::rng(2);
        let expected: f64 = 0.04241939711604948;

        fn transition(_: &u64) -> rand_distr::StandardNormal {
            rand_distr::StandardNormal
        };

        let sample: f64 = transition.sample_from(&0, &mut rng);
        assert_eq!(sample, expected);

    }
}