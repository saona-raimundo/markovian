// Traits
use core::fmt::Debug;
use num_traits::{One, Zero};
use rand::Rng;
use rand_distr::Distribution;

// use num_traits::Zero;

/// Distribution over possibly infinte iterators. 
/// 
/// A random variable is represented by an iterator that yields values ``(P, T)``,
/// where ``P`` represents the probability of the realization ``T``.
/// See [Distribution implementation] for the trait constrains over ``P``.
/// 
/// # Examples
/// 
/// With help of the `raw_dist` macro, we construct a random variable that samples always a fixed value.
/// ```
/// # use markovian::prelude::*;
/// # use rand::prelude::*;
/// let value = 0;
/// let dis = raw_dist![(1, value)];
///
/// assert_eq!(value, dis.sample(&mut thread_rng()));
/// ```
/// 
/// # Correctedness
/// 
/// Bounds on probabilities are checked only in debug mode using `debug_assert`.
/// This way, there are guarantees when developing code that probabilities
/// have valid values, but during a release run there is no overhead!
/// 
/// # Costs
/// 
/// Sample cost: O(iterator length).
/// Construction cost: O(1).
/// 
/// # Remarks
/// 
/// This struct is meant to be used when one needs to sample once from an infinte iterator.
///
/// [Distribution implementation]: struct.Raw.html#impl-Distribution<T>
#[derive(Debug, Clone, PartialEq)]
pub struct Raw<I> {
    iter: I,
}

impl<I> Raw<I> {
    #[inline]
    pub fn new(iter: I) -> Self {
        Raw { iter }
    }
}

impl<P, T, I> Distribution<T> for Raw<I>
where
    P: Zero + One + PartialOrd + Debug + Copy, 
    f64: From<P>,
    I: IntoIterator<Item = (P, T)> + Clone,
{
    #[inline]
    fn sample<R>(&self, rng: &mut R) -> T
    where
        R: Rng + ?Sized,
    {
        let cum_goal: f64 = rng.gen(); // NOT CORRECT

        let mut acc: f64 = 0.0;

        for (prob, state) in self.iter.clone() {
            debug_assert!(P::zero() <= prob, "Probabilities can not be negative. Tried to use {:?}", prob);
            debug_assert!(f64::from(P::one()) >= acc, "Probabilities can not be more than one. Tried to use {:?}", acc);
        	acc += f64::from(prob);
            if acc >= cum_goal {
                return state;
            }
        }
        panic!("Sampling was not possible: probabilities did not cover all posiibilities. Check the type of your probabilities and all possibilities by rng.gen() there.")
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use rand_distr::Distribution;
    use crate::raw_dist;

    #[test]
    fn constants() {
        let mut rng = crate::tests::rng(1);
        let expected = 1;
        let dis = raw_dist![(1, expected)];
        let sample = (0..100).map(|_| dis.sample(&mut rng));
        for x in sample {
            assert_eq!(x, expected);
        }
    }

    #[test]
    fn sampling_stability() {
        let mut rng = crate::tests::rng(1);
        let expected = 1;
        let dis = raw_dist![(1.0, expected)];
        let sample = (0..100).map(|_| dis.clone().sample(&mut rng));
        for x in sample {
            assert_eq!(x, expected);
        }

        let expected = 1;
        let dis = raw_dist![(1., expected)];
        let sample = (0..100).map(|_| dis.clone().sample(&mut rng));
        for x in sample {
            assert_eq!(x, expected);
        }

        let dis = raw_dist![(0.5, 1), (0.5, 2)];
        let sample = (0..100).map(|_| dis.clone().sample(&mut rng));
        for x in sample {
            assert!(x == 1 || x == 2);
        }
    }

    #[test]
    fn value_stability() {
        let mut rng = crate::tests::rng(2);
        let expected = [2, 1, 1, 2];
        let dis = raw_dist![(0.5, 1), (0.5, 2)];
        let sample = [
            dis.clone().sample(&mut rng),
            dis.clone().sample(&mut rng),
            dis.clone().sample(&mut rng),
            dis.clone().sample(&mut rng),
        ];

        assert_eq!(sample, expected);
    }
}
