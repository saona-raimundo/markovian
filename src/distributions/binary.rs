
use core::marker::PhantomData;
use rand::Rng;
use rand_distr::Distribution;

/// Concrete struct for the function of two Distributions`. 
/// 
/// # Examples
/// 
/// A Beta(2, 1) as the sum of two exponentials(1).
/// ```
/// # use rand_distr::Exp1;
/// # use markovian::distributions::Binary;
/// let beta = Binary::new(|x: f64, y: f64| x + y,  Exp1, Exp1);
/// ```
#[derive(Debug, Copy, Clone)]
pub struct Binary<S1, S2, T, F, D1, D2> 
where
    F: Fn(S1, S2) -> T,
    D1: Distribution<S1>,
    D2: Distribution<S2>,
{
    func: F,
    distr_1: D1,
    distr_2: D2,
    phantom: PhantomData<(S1, S2, T)>
}

impl<S1, S2, T, F, D1, D2> Binary<S1, S2, T, F, D1, D2>
where
    F: Fn(S1, S2) -> T,
    D1: Distribution<S1>,
    D2: Distribution<S2>,
{
    #[inline]
    pub fn new(func: F, distr_1: D1, distr_2: D2) -> Self {
        Binary { func, distr_1, distr_2, phantom: PhantomData }
    }
}

impl<S1, S2, T, F, D1, D2> Distribution<T> for Binary<S1, S2, T, F, D1, D2>
where
    F: Fn(S1, S2) -> T,
    D1: Distribution<S1>,
    D2: Distribution<S2>,
{
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> T {
        (self.func)(self.distr_1.sample(rng), self.distr_2.sample(rng))
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	use rand_distr::Exp1;

    #[test]
    fn use_cases() {
        let mut rng = crate::tests::rng(1);
        let expected = -0.2576656082993758;
        let sample: f64 = Binary::new(|x: f64, y: f64| x + y - 1.,  Exp1, Exp1).sample(&mut rng);

        assert_eq!(sample, expected);

        let expected = 1.5435699675490837;
        let sample: f64 = Binary::new(
            |x: f64, y: f64| x + y + 1., 
            crate::distributions::Unary::new(|x: f32| (x - 1.) as f64,  Exp1), 
            Exp1
            )
            .sample(&mut rng);
        assert_eq!(sample, expected);
    }
}
