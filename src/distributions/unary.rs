
use core::marker::PhantomData;
use rand::Rng;
use rand_distr::Distribution;

/// Concrete struct for the function of a `Distribution. 
/// 
/// # Examples
/// 
/// The squared of a exponential.
/// ```
/// # use rand_distr::Exp1;
/// # use markovian::distributions::Unary;
/// let exp_squared = Unary::new(|x: f64| x.powi(2_i32),  Exp1);
/// ```
#[derive(Debug, Copy, Clone)]
pub struct Unary<S, T, F, D> 
where
    F: Fn(S) -> T,
    D: Distribution<S>,
{
    func: F,
    distr: D,
    phantom: PhantomData<(S, T)>
}

impl<S, T, F, D> Unary<S, T, F, D>
where
    F: Fn(S) -> T,
    D: Distribution<S>,
{
    #[inline]
    pub fn new(func: F, distr: D) -> Self {
        Unary { func, distr, phantom: PhantomData }
    }
}

impl<S, T, F, D> Distribution<T> for Unary<S, T, F, D>
where
    F: Fn(S) -> T,
    D: Distribution<S>,
{
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> T {
        (self.func)(self.distr.sample(rng))
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	use rand_distr::Exp1;

    #[test]
    fn use_cases() {
        let mut rng = crate::tests::rng(1);
        let expected = 0.5851203106605716 - 1.;
        let sample: f64 = Unary::new(|x: f64| x - 1.,  Exp1).sample(&mut rng);

        assert_eq!(sample, expected);

        let expected = 0.15721404552459717;
        let sample: f64 = Unary::new(|x:f64| x + 1., Unary::new(|x: f32| (x - 1.) as f64,  Exp1)).sample(&mut rng);
        assert_eq!(sample, expected);
    }
}
