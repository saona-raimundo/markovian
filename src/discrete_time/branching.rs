//! Branching process in the natural numbers NN = {0, 1, 2, ...}.
//! It is characterized by a density p over NN.
//! 
//! The stochastic process can be thought of the size of a population. 
//! In this population, each individual is identical to the rest and they are 
//! independent of each other. Moreover, at each time step, 
//! individuals have descendents and die. Their descendants 
//! constitutes the second generation and the process repeats. 
//! The overall process is therefore characterized by the number of 
//! offsprings an individual has. 
//! The resulting process is a Markov Chain in NN.

// Traits

use crate::traits::{BranchingTrait, MarkovChainTrait};
use num_traits::identities::{One, Zero};
use num_traits::sign::Unsigned;
use std::cmp::PartialOrd;
use std::ops::AddAssign;

/// Branching process. A stochastic Markov chain in {0, 1, ...}. 
/// 
/// By stochastic we mean that the process always leaves in {0, 1, ...}.  
/// 
/// It has two associated types for ease of use. 
///  - T, type of the values the process take. 
///  - I, something that can be converted into iterator that represents 
/// a distribution over the number of offsrpings. Its elements are
/// pairs of (quantity, probability). 
/// 
/// A Branching can be seen as: 
/// - State-iterator: an iterator with a current state, changing randomly to another 
/// state when ``next`` method is called. See 
/// [Iterator](https://doc.rust-lang.org/nightly/core/iter/trait.Iterator.html)
/// and [MarkovChainTrait](file:///C:/Users/rasau/projects/markovian/target/doc/markovian/discrete_time/struct.MarkovChain.html) 
/// implementation. 
/// 
#[derive(Debug)]
pub struct Branching<T, I>
where
    T: Clone + Unsigned + Zero + One + PartialOrd + AddAssign,
    I: IntoIterator<Item = (T, f64)> + Clone,
{
    state: T,
    density: I,
}

impl<T, I> Branching<T, I>
where
    T: Clone + Unsigned + Zero + One + PartialOrd + AddAssign,
    I: IntoIterator<Item = (T, f64)> + Clone,
{
    /// Creates a new Branching. 
    /// 
    /// # Examples
    /// 
    /// Construction using density p(0) = 0.3, p(1) = 0.4, p(2) = 0.3. 
    /// ```
    /// # #![allow(unused_mut)]
    /// let init_state: u32 = 1;
    /// let density = vec![(0, 0.3), (1, 0.4), (2, 0.3)];
    /// let mut branching_process = markovian::Branching::new(init_state, density);
    /// ``` 
    ///
    pub fn new(state: T, density: I) -> Self {
        Branching { state, density }
    }
}

impl<T, I> BranchingTrait<T, I> for Branching<T, I>
where
    T: Clone + Unsigned + Zero + One + PartialOrd + AddAssign,
    f64: From<T>,
    I: IntoIterator<Item = (T, f64)> + Clone,
{
    /// Returns the density used for the process. 
    fn density(&self) -> I {
        self.density.clone()
    }
}

impl<T, I> MarkovChainTrait<T> for Branching<T, I>
where
    T: Clone + Unsigned + Zero + One + PartialOrd + AddAssign,
    I: IntoIterator<Item = (T, f64)> + Clone,
{
    /// Current state of the process. 
    fn state(&self) -> &T {
        &self.state
    }

    /// Change the current state of the process. 
    fn set_state(&mut self, state: T) -> &mut Self {
        self.state = state;
        self
    }
}

impl<T, I> Iterator for Branching<T, I>
where
    T: Clone + Unsigned + Zero + One + PartialOrd + AddAssign,
    I: IntoIterator<Item = (T, f64)> + Clone,
{
    type Item = T;

    /// Changes the state of the Branching to a new state, chosen 
    /// according to the distribution for offsprings, and returns the new state. 
    /// 
    /// # Examples
    /// 
    ///  ```
    /// let init_state: u32 = 1;
    /// let density = vec![(0, 0.3), (1, 0.4), (2, 0.3)];
    /// let mut branching_process = markovian::Branching::new(init_state, density);
    ///
    /// // The next state is 0, 1 or 2. 
    /// let new_state = branching_process.next();
    /// assert!( (new_state == Some(0)) || (new_state == Some(1)) || (new_state == Some(2)) );
    /// 
    /// use markovian::traits::MarkovChainTrait;
    /// assert_eq!(new_state, Some(branching_process.state()).copied()) ;
    /// ```
    fn next(&mut self) -> Option<Self::Item> {
        let mut next = T::zero();

        // Simulate each individual in the population.
        let mut counter = T::zero();
        let limit = self.state().to_owned();
        while counter < limit {
            let uniform: f64 = rand::random();
            let mut cummulative = 0.;

            let neigbours = self.density.clone();
            for (state, prob) in neigbours {
                cummulative += prob;
                if uniform <= cummulative {
                    next += state;
                    break;
                }
            }
            counter += T::one();
        }

        // Update
        self.set_state(next);

        // Return
        Some(self.state().clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checking_construction() {
        // density as a vector

        let state: u32 = 1;
        let density = vec![(0, 0.5), (2, 0.5)];

        let branching_process = Branching::new(state, density.clone());

        assert_eq!(&state, branching_process.state());
        assert_eq!(density, Branching::density(&branching_process));

        // density as a closure

        let state: usize = 1;
        let density = vec![0, 2].into_iter().map(|i| (i, 0.5));

        let branching_process = Branching::new(state, density);

        assert_eq!(&state, branching_process.state());

        // Infinite support

        let state: usize = 1;
        let density = (0..).map(|i| (i, 0.5));

        let branching_process = Branching::new(state, density);

        assert_eq!(&state, branching_process.state());
    }

    #[test]
    fn generating_fun_evaluations() {
        // density as a vector

        let state: u32 = 1;
        let density = vec![(0, 0.5), (2, 0.5)];

        let branching_process = Branching::new(state, density);

        assert_eq!(1.0, branching_process.approx_generating_fun(1.0, 2));
        assert_eq!(0.5, branching_process.approx_generating_fun(0.0, 2));

        // density as a closure

        let state: u32 = 1;
        let density = vec![0, 2].into_iter().map(|i| (i, 0.5));

        let branching_process = Branching::new(state, density);

        assert_eq!(&state, branching_process.state());

        // Infinite support

        let state: u32 = 1;
        let density = (0..).map(|i| (i, 0.5));

        let branching_process = Branching::new(state, density);

        assert_eq!(1.5, branching_process.approx_generating_fun(1.0, 3));
        assert_eq!(
            0.5 + 2.0 * 0.5 + 4.0 * 0.5,
            branching_process.approx_generating_fun(2.0, 3)
        );
    }
}
