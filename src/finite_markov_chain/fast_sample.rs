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
///
/// # Warning
///
/// The user should make sure that the indexes resulting from random transitions 
/// correspond to a state in the state space. In other words, new indexes
/// should always be less than the length of the state space. 
///
/// # Examples
///
/// Random walk in the integers.
/// ```
/// # use markovian::{MarkovChain, prelude::*};
/// # use rand::prelude::*;
/// let init_state: i32 = 0;
/// let transition = |state: &i32| Raw::new(vec![(0.5, state + 1), (0.5, state - 1)]);
/// MarkovChain::new(init_state, &transition, thread_rng());
/// ```
#[derive(Debug, Clone)]
pub struct FiniteMarkovChain<T, W, R>
where
    W: Weight,
    Uniform<W>: Debug + Clone,
    T: Debug + PartialEq + Clone,
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
    T: Debug + PartialEq + Clone,
    R: Rng,
{
    /// Constructs a new `FiniteMarkovChain<T, W, R>`.
    /// 
    /// # Panics
    /// 
    /// In debug mode, if the length of `state_space` and `transition_matrix` do not match. 
    #[inline]
    pub fn new(
        state_index: usize,
        transition_matrix: Vec<WeightedIndex<W>>,
        state_space: Vec<T>,
        rng: R,
    ) -> Self {
        debug_assert_eq!(transition_matrix.len(), state_space.len());
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

impl<T, W, R> From<(usize, Vec<Vec<W>>, Vec<T>, R)> for FiniteMarkovChain<T, W, R>
where
    W: Weight,
    Uniform<W>: Debug + Clone,
    T: Debug + PartialEq + Clone,
    R: Rng,
{
    /// # Panics
    ///
    /// If [WeightedIndex::new] returns error for some transition.
    ///
    /// [WeightedIndex::new]: https://docs.rs/rand_distr/0.3.0/rand_distr/struct.WeightedIndex.html#method.new
    fn from((state_index, transition_matrix, state_space, rng): (usize, Vec<Vec<W>>, Vec<T>, R)) -> Self {
        let transition_matrix: Vec<WeightedIndex<W>> = transition_matrix.into_iter()
            .map(|weights| {
               WeightedIndex::new(weights).unwrap()
            })
            .collect();
        FiniteMarkovChain::new(state_index, transition_matrix, state_space, rng)
    }
}

impl<T, W, R> From<(usize, ndarray::Array2<W>, Vec<T>, R)> for FiniteMarkovChain<T, W, R>
where
    W: Weight,
    Uniform<W>: Debug + Clone,
    T: Debug + PartialEq + Clone,
    R: Rng,
{
    /// # Panics
    ///
    /// If [WeightedIndex::new] returns error for some transition.
    ///
    /// [WeightedIndex::new]: https://docs.rs/rand_distr/0.3.0/rand_distr/struct.WeightedIndex.html#method.new
    fn from((state_index, transition_matrix, state_space, rng): (usize, ndarray::Array2<W>, Vec<T>, R)) -> Self {
        let transition_matrix: Vec<WeightedIndex<W>> = transition_matrix.genrows()
            .into_iter()
            .map(|weights| {
                WeightedIndex::new(weights.to_vec()).unwrap()
            })
            .collect();
        FiniteMarkovChain::new(state_index, transition_matrix, state_space, rng)
    }
}

#[cfg(test)]
mod tests {

    use test_case::test_case;
    use super::*;
    use rand::prelude::*;
    use ndarray::{array, Array2};
    
    #[test_case(0, Vec::new(), vec![1], thread_rng() => panics ""; "not enough transitions")]
    fn construction(state_index: usize, transition_matrix: Vec<WeightedIndex<usize>>, state_space: Vec<u64>, rng: rand::prelude::ThreadRng) {
        FiniteMarkovChain::new(state_index, transition_matrix, state_space, rng);
    }

    #[test_case(0, Vec::new(), vec![1], thread_rng() => panics ""; "not enough transitions")]
    #[test_case(0, vec![Vec::new()], Vec::new(), thread_rng() => panics ""; "empty transition")]
    #[test_case(0, Vec::new(), Vec::new(), thread_rng(); "empty chain")]
    fn construction_vectors(state_index: usize, transition_matrix: Vec<Vec<usize>>, state_space: Vec<u64>, rng: rand::prelude::ThreadRng) {
        FiniteMarkovChain::from((state_index, transition_matrix, state_space, rng));
    }

    #[test_case(0, array![[]], vec![1], thread_rng() => panics ""; "not enough transitions")]
    #[test_case(0, array![[]], Vec::new(), thread_rng() => panics ""; "empty transition")]
    fn construction_array2(state_index: usize, transition_matrix: Array2<usize>, state_space: Vec<u64>, rng: rand::prelude::ThreadRng) {
        FiniteMarkovChain::from((state_index, transition_matrix, state_space, rng));
    }


    #[test]
    fn change_state() {
        let mut finite_mc = FiniteMarkovChain::from((0, vec![vec![1, 2], vec![2, 1]], vec![10, 20], thread_rng()));
        let previous_state = finite_mc.set_state(20).unwrap();
        assert_eq!(Some(10), previous_state);
    }
}