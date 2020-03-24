//! Branching process in the natural numbers NN = {0, 1, 2, ...}.
//! It is characterized by a density p over NN.
//! Each individual leaves a number of descendents according to p
//! and these descendents are identical and independent.
//!
//! The resulting process is a Markov Chain in NN.

// Traits

use crate::traits::{BranchingTrait, MarkovChainTrait};
use num_traits::identities::{One, Zero};
use num_traits::sign::Unsigned;
use std::cmp::PartialOrd;
use std::ops::AddAssign;

/// Sub-stochastic Markov Chain.
// #[derive(Debug)]
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
    fn density(&self) -> I {
        self.density.clone()
    }
}

impl<T, I> MarkovChainTrait<T> for Branching<T, I>
where
    T: Clone + Unsigned + Zero + One + PartialOrd + AddAssign,
    I: IntoIterator<Item = (T, f64)> + Clone,
{
    fn state(&self) -> &T {
        &self.state
    }
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

        assert_eq!(1.5, branching_process.approx_generating_fun(1.0, 2));
        assert_eq!(
            0.5 + 2.0 * 0.5 + 4.0 * 0.5,
            branching_process.approx_generating_fun(2.0, 2)
        );
    }
}
