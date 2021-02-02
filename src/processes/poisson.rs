// Traits
use num_traits::Float;
use rand_distr::{Exp1, Exp};
use crate::{State, StateIterator};
use core::fmt::Debug;
use num_traits::{sign::Unsigned, One, Zero};
use rand::Rng;
use rand_distr::Distribution;

// Structs
use crate::errors::InvalidState;

// Functions
use core::mem;

/// Homogeneous [poisson process] in the natural numbers NN = {0, 1, 2, ...}.
/// 
/// [poisson process]: https://en.wikipedia.org/wiki/Poisson_point_process#Homogeneous_Poisson_point_process
#[derive(Debug, Clone)]
pub struct Poisson<N, T, R>
where
    N: Float,
    Exp1: Distribution<N>, 
    T: Debug + PartialEq + Clone + One + Zero + PartialOrd + Unsigned,
    R: Rng,
{
    state: T,
    exp: Exp<N>,
    rng: R,
}

impl<N, T, R> Poisson<N, T, R>
where
    N: Float,
    Exp1: Distribution<N>, 
    T: Debug + PartialEq + Clone + One + Zero + PartialOrd + Unsigned,
    R: Rng,
{
    /// Construct a new `Poisson` process with the given shape parameter
    /// `lambda`.
    /// 
    /// # Remarks
    /// 
    /// For custom types `N` implementing the [`Float`] trait,
    /// the case `lambda = 0` is handled as follows: each sample corresponds
    /// to a sample from an `Exp1` multiplied by `1 / 0`. Primitive types 
    /// yield infinity, since `1 / 0 = infinity`.
    /// 
    /// # Examples
    /// 
    /// Construction using `lambda` one.
    /// ```
    /// # #![allow(unused_mut)]
    /// # use markovian::prelude::*;
    /// # use rand::prelude::*;
    /// let lambda = 1.;
    /// let rng = thread_rng();
    /// let mut poisson_process = markovian::processes::Poisson::<f64, usize, _>::new(lambda, rng);
    /// ```
    #[inline]
    pub fn new(lambda: N, rng: R) -> Result<Self, rand_distr::ExpError> {
        Ok(Poisson {
            state: T::zero(),
            exp: Exp::new(lambda)?,
            rng,
        })
    }
}

impl<N, T, R> State for Poisson<N, T, R>
where
    N: Float,
    Exp1: Distribution<N>, 
    T: Debug + PartialEq + Clone + One + Zero + PartialOrd + Unsigned,
    R: Rng,
{
    type Item = T;

    #[inline]
    fn state(&self) -> Option<&Self::Item> {
        Some(&self.state)
    }

    #[inline]
    fn state_mut(&mut self) -> Option<&mut Self::Item> {
        Some(&mut self.state)
    }

    #[inline]
    fn set_state(
        &mut self,
        mut new_state: Self::Item,
    ) -> Result<Option<Self::Item>, InvalidState<Self::Item>> {
        mem::swap(&mut self.state, &mut new_state);
        Ok(Some(new_state))
    }
}

impl<N, T, R> Iterator for Poisson<N, T, R>
where
    N: Float,
    Exp1: Distribution<N>, 
    T: Debug + PartialEq + Clone + One + Zero + PartialOrd + Unsigned,
    R: Rng,
{
    type Item = (N, T);

    /// Changes the state of the Branching to a new state, chosen 
    /// according to the distribution for offsprings, and returns the new state. 
    ///
    /// # Panics
    ///
    /// Panics if the new state exceeds maximum of `T`.
    /// 
    /// # Examples
    /// 
    ///  ```
    /// # use rand::prelude::*;
    /// # use markovian::prelude::*;
    /// let lambda = 1.0;
    /// let rng = thread_rng();
    /// let mut poisson_process = Poisson::new(lambda, rng).unwrap();
    ///
    /// // The next state is 1, after some time. 
    /// let (period, new_state): (f64, usize) = poisson_process.next().unwrap();
    /// assert!(period > 0.);
    /// assert_eq!(new_state, 1);
    /// ```
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let period = self.exp.sample(&mut self.rng);
        self.set_state(self.state.clone() + T::one()).unwrap();
        self.state().cloned().map(|state| (period, state))
    }
}

impl<N, T, R> StateIterator for Poisson<N, T, R>
where
    N: Float,
    Exp1: Distribution<N>, 
    T: Debug + PartialEq + Clone + One + Zero + PartialOrd + Unsigned,
    R: Rng,
{
    #[inline]
    fn state_as_item(&self) -> Option<<Self as std::iter::Iterator>::Item> {
        self.state().cloned().map(|state| (N::zero(), state))
    }
}

impl<N, T, R> Distribution<(N, T)> for Poisson<N, T, R>
where
    N: Float,
    Exp1: Distribution<N>, 
    T: Debug + PartialEq + Clone + One + Zero + PartialOrd + Unsigned,
    R: Rng,
{
    /// Sample a possible next state. 
    #[inline]
    fn sample<R2>(&self, rng: &mut R2) -> (N, T)
    where
        R2: Rng + ?Sized,
    { 
        (self.exp.sample(rng), self.state.clone() + T::one())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn value_stability() {
        let rng = crate::tests::rng(3);
        let lambda = 1.;
        let expected = vec![(0.529274135874436, 1), (0.5369108748992898, 2), (0.3618522192460201, 3), (0.5717432176122981, 4)];
        let mc = Poisson::new(lambda, rng).unwrap();
        let sample: Vec<(f64, u64)> = mc.take(4).collect();

        assert_eq!(sample, expected);
    }
}