// Traits
use crate::{State, StateIterator};
use core::fmt::Debug;
use rand::Rng;
use rand_distr::{weighted::alias_method::Weight, Distribution};

// Structs
use crate::errors::InvalidState;
use rand_distr::uniform::Uniform;
use rand_distr::weighted::alias_method::WeightedIndex;

// Functions
use core::mem;

/// Finite state Markov Chain in discrete time. 
/// 
/// # Costs
/// 
/// Construction cost: O(n), n: size of the state space.
/// Sample cost: O(1).
#[derive(Debug, Clone)]
pub struct FiniteMarkovChain<T, W, R>
where
    W: Weight,
    Uniform<W>: Debug + Clone,
    R: Rng,
{
    state_index: usize,
    transition_matrix: Vec<WeightedIndex<W>>,
    state_space: Vec<T>,
    rng: R,
}

impl<T, W, R> FiniteMarkovChain<T, W, R>
where
    W: Weight,
    Uniform<W>: Debug + Clone,
    R: Rng,
{
    #[inline]
    pub fn new(
        state_index: usize,
        transition_matrix: Vec<WeightedIndex<W>>,
        state_space: Vec<T>,
        rng: R,
    ) -> Self {
        FiniteMarkovChain {
            state_index,
            transition_matrix,
            state_space,
            rng,
        }
    }

    #[inline]
    fn sample_index(&mut self) -> usize {
        self.transition_matrix[self.state_index].sample(&mut self.rng)
    }
}

impl<T, W, R> State for FiniteMarkovChain<T, W, R>
where
    W: Weight,
    Uniform<W>: Debug + Clone,
    T: Debug + PartialEq + Clone,
    R: Rng,
{
    type Item = T;

    #[inline]
    fn state(&self) -> Option<&Self::Item> {
        Some(&self.state_space[self.state_index])
    }

    #[inline]
    fn state_mut(&mut self) -> Option<&mut Self::Item> {
        Some(&mut self.state_space[self.state_index])
    }

    #[inline]
    fn set_state(
        &mut self,
        new_state: Self::Item,
    ) -> Result<Option<Self::Item>, InvalidState<Self::Item>> {
        match self.state_space.iter().position(|s| *s == new_state) {
            Some(mut state_index) => {
                mem::swap(&mut self.state_index, &mut state_index);
                Ok(Some(self.state_space[state_index].clone()))
            }
            None => Err(InvalidState::new(new_state)),
        }
    }
}

impl<T, W, R> Iterator for FiniteMarkovChain<T, W, R>
where
    W: Weight,
    Uniform<W>: Debug + Clone,
    T: Debug + PartialEq + Clone,
    R: Rng,
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.state_index = self.sample_index();
        self.state().cloned()
    }
}

impl<T, W, R> StateIterator for FiniteMarkovChain<T, W, R>
where
    W: Weight,
    Uniform<W>: Debug + Clone,
    T: Debug + PartialEq + Clone,
    R: Rng,
{
    #[inline]
    fn state_as_item(&self) -> Option<<Self as std::iter::Iterator>::Item> {
        self.state().cloned()
    }
}

impl<T, W, R> Distribution<T> for FiniteMarkovChain<T, W, R>
where
    W: Weight,
    Uniform<W>: Debug + Clone,
    T: Debug + PartialEq + Clone,
    R: Rng,
{
    /// Sample a possible next state. 
    #[inline]
    fn sample<R2>(&self, rng: &mut R2) -> T
    where
        R2: Rng + ?Sized,
    { 
        let new_index = self.transition_matrix[self.state_index].sample(rng);

        self.state_space[new_index].clone()
    }
}
