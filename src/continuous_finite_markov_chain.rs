// Traits
use crate::{State, StateIterator};
use core::fmt::Debug;
use rand::Rng;
use rand_distr::{Float, weighted::alias_method::Weight, Distribution};

// Structs
use crate::errors::InvalidState;
use rand_distr::{Exp1, Exp, Uniform, weighted::alias_method::WeightedIndex};

// Functions
use core::mem;

/// Finite state Markov Chain in continuous time. 
/// 
/// # Costs
/// 
/// Construction cost: O(n), n: size of the state space.
/// Sample cost: O(1).
#[derive(Debug, Clone)]
pub struct ContFiniteMarkovChain<T, W, R>
where
    W: Float + Weight,
    Exp1: Distribution<W>,
    Uniform<W>: Debug + Clone,
    R: Rng,
{
    state_index: usize,
    transition_matrix: Vec<WeightedIndex<W>>,
    transiton_clock: Vec<W>,
    state_space: Vec<T>,
    rng: R,
}

impl<T, W, R> ContFiniteMarkovChain<T, W, R>
where
    W: Float + Weight + std::iter::Sum<W>,
    Exp1: Distribution<W>,
    Uniform<W>: Debug + Clone,
    R: Rng,
{
    #[inline]
    pub fn new(
        state_index: usize,
        transition_weights: Vec<Vec<W>>,
        state_space: Vec<T>,
        rng: R,
    ) -> Self {
        let transition_matrix: Vec<WeightedIndex<W>> = transition_weights.clone()
            .into_iter()
            .map(|weights| WeightedIndex::new(weights).unwrap())
            .collect();
        let transiton_clock: Vec<W> = transition_weights.into_iter()
            .map(|weights| weights.into_iter().sum::<W>())
            .collect();
        ContFiniteMarkovChain {
            state_index,
            transition_matrix,
            transiton_clock,
            state_space,
            rng,
        }
    }

    #[inline]
    fn sample_index(&mut self) -> usize {
        self.transition_matrix[self.state_index].sample(&mut self.rng)
    }

    #[inline]
    fn sample_clock(&mut self) -> W {
        let rate = self.transiton_clock[self.state_index];
        Exp::new(rate).unwrap().sample(&mut self.rng)
    }
}

impl<T, W, R> State for ContFiniteMarkovChain<T, W, R>
where
    W: Float + Weight,
    Exp1: Distribution<W>,
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

impl<T, W, R> Iterator for ContFiniteMarkovChain<T, W, R>
where
    W: Float + Weight,
    Exp1: Distribution<W>,
    Uniform<W>: Debug + Clone,
    T: Debug + PartialEq + Clone,
    R: Rng,
{
    type Item = (W, T);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.state_index = self.sample_index();
        let period = self.sample_clock();
        self.state().cloned().map(|x| (period, x))
    }
}

impl<T, W, R> StateIterator for ContFiniteMarkovChain<T, W, R>
where
    W: Float + Weight,
    Exp1: Distribution<W>,
    Uniform<W>: Debug + Clone,
    T: Debug + PartialEq + Clone,
    R: Rng,
{
    #[inline]
    fn state_as_item(&self) -> Option<<Self as std::iter::Iterator>::Item> {
        self.state().cloned().map(|x| (W::from(0.0), x))
    }
}

impl<T, W, R> Distribution<(W, T)> for ContFiniteMarkovChain<T, W, R>
where
    W: Float + Weight,
    Exp1: Distribution<W>,
    Uniform<W>: Debug + Clone,
    T: Debug + PartialEq + Clone,
    R: Rng,
{
    /// Sample a possible next state. 
    #[inline]
    fn sample<R2>(&self, rng: &mut R2) -> (W, T) 
    where
        R2: Rng + ?Sized,
    { 
        let new_index = self.transition_matrix[self.state_index].sample(rng);
        let rate = self.transiton_clock[self.state_index];
        let step = Exp::new(rate).unwrap().sample(rng);

        (step, self.state_space[new_index].clone())
    }
}