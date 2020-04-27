// Traits
use crate::traits::DistributionOnce;
use crate::traits::Probability;
use rand::Rng;
use rand_distr::Distribution;
use rand_distr::Standard;
// use num_traits::Zero;

/// A quick and higly error prone struct for quickly define a struct that
/// implements `Distribution<T>`.
/// `Clone` is needed, since `Distribution` is a trait for state-less struts.
#[derive(Debug)]
pub struct Raw<I> {
    iter: I,
}

impl<I> Raw<I> {
    #[inline]
    pub fn new(iter: I) -> Self {
        Raw { iter }
    }
}

impl<P, T, I> DistributionOnce<T> for Raw<I>
where
    P: Probability,
    Standard: Distribution<P>,
    I: IntoIterator<Item = (P, T)> + Clone,
{
    #[inline]
    fn sample_once<R>(&self, rng: &mut R) -> T
    where
        R: Rng + ?Sized,
    {
        let cum_goal: P = rng.gen();

        This is not correct. 

        let mut acc = P::zero();

        for (prob, state) in self.iter.clone() {
            acc = acc + prob;
            if acc >= cum_goal {
                return state;
            }
        }
        panic!("Sampling was not possible: probabilities did not cover all posiibilities. Check the type of your probabilities and all possibilities by rng.gen() there.")
    }
}

// impl<P, T, I> Distribution<T> for Raw<I>
// where
//     P: Probability,
//     Standard: Distribution<P>,
//     I: IntoIterator<Item = (P, T)> + Clone,
// {
//     #[inline]
//     fn sample<R>(&self, rng: &mut R) -> T
//     where
//         R: Rng + ?Sized,
//     {
//         let cum_goal: P = rng.gen();
//         let mut acc = P::zero();

//         for (prob, state) in self.iter.clone() {
//             acc = acc + prob;
//             if acc >= cum_goal {
//                 return state;
//             }
//         }
//         panic!("Sampling was not possible: probabilities did not cover all posiibilities. Check the type of your probabilities and all possibilities by rng.gen() there.")
//     }
// }

#[cfg(test)]
mod tests {
    use super::Raw;
    use pretty_assertions::assert_eq;
    use rand_distr::Distribution;
    // use approx::abs_diff_eq;

    #[test]
    fn sampling_stability() {
        let mut rng = crate::test::rng(1);
        let expected = 1;
        let dis = Raw::new(vec![(1.0, expected)]);
        let sample = (0..100).map(|_| dis.sample(&mut rng));
        for x in sample {
            assert_eq!(x, expected);
        }

        let expected = 1;
        let dis = Raw::new(vec![(std::u32::MAX, expected)]);
        let sample = (0..100).map(|_| dis.sample(&mut rng));
        for x in sample {
            assert_eq!(x, expected);
        }

        let dis = Raw::new(vec![(0.5, 1), (0.5, 2)]);
        let sample = (0..100).map(|_| dis.sample(&mut rng));
        for x in sample {
            assert!(x == 1 || x == 2);
        }
    }

    #[test]
    fn value_stability() {
        let mut rng = crate::test::rng(2);
        let expected = [2, 1, 1, 2];
        let dis = Raw::new(vec![(0.5, 1), (0.5, 2)]);
        let sample = [
            dis.sample(&mut rng),
            dis.sample(&mut rng),
            dis.sample(&mut rng),
            dis.sample(&mut rng),
        ];

        assert_eq!(sample, expected);
    }
}
