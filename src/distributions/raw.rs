// Traits
use crate::traits::Probability;
use rand::Rng;
use rand_distr::Distribution;
use rand_distr::Standard;
// use num_traits::Zero;

/// Distribution over possibly infinte iterators. 
/// Sample cost: O(iterator length).
/// Construction cost: O(1).
/// 
/// # Remarks
/// 
/// This struct is meant to be used when one needs to sample once from an infinte iterator.
#[derive(Debug, Clone)]
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
    P: Probability,
    Standard: Distribution<P>,
    I: IntoIterator<Item = (P, T)> + Clone,
{
    #[inline]
    fn sample<R>(&self, rng: &mut R) -> T
    where
        R: Rng + ?Sized,
    {
        let cum_goal: P = rng.gen(); // NOT CORRECT

        let zero = P::zero();
        let one = P::one();
        let mut acc = P::zero();

        for (prob, state) in self.iter.clone() {
            if zero > prob { panic!("Probabilities can not be negative. Tried to use {:?}", prob); }
            acc = acc + prob;
            if acc > one { panic!("Probabilities can not be more than one. Tried to use {:?}", acc); }
            if acc >= cum_goal {
                return state;
            }
        }
        panic!("Sampling was not possible: probabilities did not cover all posiibilities. Check the type of your probabilities and all possibilities by rng.gen() there.")
    }
}

#[cfg(test)]
mod tests {
    use super::Raw;
    use pretty_assertions::assert_eq;
    use rand_distr::Distribution;
    // use approx::abs_diff_eq;

    #[test]
    fn sampling_stability() {
        let mut rng = crate::tests::rng(1);
        let expected = 1;
        let dis = Raw::new(vec![(1.0, expected)]);
        let sample = (0..100).map(|_| dis.clone().sample(&mut rng));
        for x in sample {
            assert_eq!(x, expected);
        }

        let expected = 1;
        let dis = Raw::new(vec![(1., expected)]);
        let sample = (0..100).map(|_| dis.clone().sample(&mut rng));
        for x in sample {
            assert_eq!(x, expected);
        }

        let dis = Raw::new(vec![(0.5, 1), (0.5, 2)]);
        let sample = (0..100).map(|_| dis.clone().sample(&mut rng));
        for x in sample {
            assert!(x == 1 || x == 2);
        }
    }

    #[test]
    fn value_stability() {
        let mut rng = crate::tests::rng(2);
        let expected = [2, 1, 1, 2];
        let dis = Raw::new(vec![(0.5, 1), (0.5, 2)]);
        let sample = [
            dis.clone().sample(&mut rng),
            dis.clone().sample(&mut rng),
            dis.clone().sample(&mut rng),
            dis.clone().sample(&mut rng),
        ];

        assert_eq!(sample, expected);
    }
}
