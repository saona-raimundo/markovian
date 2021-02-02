// Traits
use crate::{State, StateIterator};
use core::fmt::Debug;
use num_traits::{sign::Unsigned, One, Zero};
use rand::Rng;
use rand_distr::Distribution;

// Structs
use crate::errors::InvalidState;

// Functions
use core::mem;

/// Branching process in the natural numbers NN = {0, 1, 2, ...}.
/// 
/// A Branching process is characterized by a density p over NN. It can be 
/// thought of the size of a population. 
/// In this population, each individual is identical to the rest and they are 
/// independent of each other. Moreover, at each time step, 
/// individuals have descendents and die. Their descendants 
/// constitutes the second generation and the process repeats. 
/// The overall process is therefore characterized by the number of 
/// offsprings an individual has. 
/// The resulting process is a Markov Chain in NN.
#[derive(Debug, Clone)]
pub struct Branching<T, D, R> 
where
    T: Debug + PartialEq + Clone + One + Zero + PartialOrd + Unsigned,
    D: Distribution<T>,
    R: Rng,
{
    state: T,
    base_distribution: D,
    rng: R,
}

impl<T, D, R> Branching<T, D, R>
where
    T: Debug + PartialEq + Clone + One + Zero + PartialOrd + Unsigned,
    D: Distribution<T>,
    R: Rng,
{
    /// Creates a new Branching process. 
    /// 
    /// # Examples
    /// 
    /// Construction using density p(0) = 0.3, p(1) = 0.4, p(2) = 0.3. 
    /// ```
    /// # #![allow(unused_mut)]
    /// # use markovian::prelude::*;
    /// # use rand::prelude::*;
    /// let init_state: u32 = 1;
    /// let density = raw_dist![(0.3, 0), (0.4, 1), (0.3, 2)];
    /// let rng = thread_rng();
    /// let mut branching_process = markovian::Branching::new(init_state, density, rng);
    /// ``` 
    ///
    #[inline]
    pub fn new(state: T, base_distribution: D, rng: R) -> Self {
        Branching {
            state,
            base_distribution,
            rng,
        }
    }
}

impl<T, D, R> State for Branching<T, D, R>
where
    T: Debug + PartialEq + Clone + One + Zero + PartialOrd + Unsigned,
    D: Distribution<T>,
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

impl<T, D, R> Iterator for Branching<T, D, R>
where
    T: Debug + PartialEq + Clone + One + Zero + PartialOrd + Unsigned,
    D: Distribution<T>,
    R: Rng,
{
    type Item = T;

    /// Changes the state of the Branching to a new state, chosen 
    /// according to the distribution for offsprings, and returns the new state. 
    /// 
    /// # Examples
    /// 
    ///  ```
    /// # use rand::prelude::*;
    /// # use markovian::prelude::*;
    /// let init_state: u32 = 1;
    /// let density = raw_dist![(0.3, 0), (0.4, 1), (0.3, 2)];
    /// let rng = thread_rng();
    /// let mut branching_process = markovian::Branching::new(init_state, density, rng);
    ///
    /// // The next state is 0, 1 or 2. 
    /// let new_state = branching_process.next();
    /// assert!( (new_state == Some(0)) || (new_state == Some(1)) || (new_state == Some(2)) );
    /// ```
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let mut count = T::one();
        let mut acc = T::zero();
        while count <= self.state {
            acc = acc + self.base_distribution.sample(&mut self.rng);
            count = count + T::one();
        }
        self.state = acc.clone();
        Some(acc)
    }
}

impl<T, D, R> StateIterator for Branching<T, D, R>
where
    T: Debug + PartialEq + Clone + One + Zero + PartialOrd + Unsigned,
    D: Distribution<T>,
    R: Rng,
{
    #[inline]
    fn state_as_item(&self) -> Option<<Self as std::iter::Iterator>::Item> {
        self.state().cloned()
    }
}

impl<T, D, R> Distribution<T> for Branching<T, D, R>
where
    T: Debug + PartialEq + Clone + One + Zero + PartialOrd + Unsigned,
    D: Distribution<T>,
    R: Rng,
{
    /// Sample a possible next state. 
    #[inline]
    fn sample<R2>(&self, rng: &mut R2) -> T 
    where
        R2: Rng + ?Sized,
    { 
        let mut count = T::one();
        let mut acc = T::zero();
        while count < self.state {
            acc = acc + self.base_distribution.sample(rng);
            count = count + T::one();
        }
        acc
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn value_stability() {
        let expected = vec![2, 1, 2, 1, 1, 2, 4, 3, 2, 1, 1, 0];
        let init_state: u32 = 1;
        let density = raw_dist![(0.3, 0), (0.4, 1), (0.3, 2)];
        let rng = crate::tests::rng(1);
        let branching_process = Branching::new(init_state, density, rng);
        let sample: Vec<u32> = branching_process.take(12).collect();
        assert_eq!(sample, expected);
    }
}