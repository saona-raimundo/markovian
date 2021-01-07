// Traits
use crate::{State, StateIterator};
use core::fmt::Debug;
use rand::Rng;
use rand_distr::{weighted_alias::{WeightedAliasIndex, AliasableWeight}, Uniform, Distribution};

// Structs
use crate::errors::InvalidState;

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
/// The easiest way is construct a finite Markov Chain is from a transition matrix. 
/// This has been abstracted by using `from`. For example,
/// an absorbing Markov Chain with one transient state and one absorbing state.
/// ```
/// # use ndarray::array;
/// # use markovian::{FiniteMarkovChain, State};
/// let mut mc = FiniteMarkovChain::from((0, array![[0.5, 0.5], [0.0, 1.0]], rand::thread_rng()));
/// assert_eq!(mc.state(), Some(&0));
/// println!("At time {}, the state is {}", 1_000, mc.nth(1_000).unwrap()); // Most likely 1
/// ```
#[derive(Debug, Clone)]
pub struct FiniteMarkovChain<T, W, R>
where
    W: AliasableWeight + Debug + Clone,
    Uniform<W>: Debug + Clone,
    T: Debug + PartialEq + Clone,
    R: Rng + Debug + Clone,
{
    state_index: usize,
    transition_matrix: Vec<Vec<W>>,
    transition_matrix_variables: Vec<WeightedAliasIndex<W>>,
    state_space: Vec<T>,
    rng: R,
}

impl<T, W, R> FiniteMarkovChain<T, W, R>
where
    W: AliasableWeight + Debug + Clone,
    Uniform<W>: Debug + Clone,
    T: Debug + PartialEq + Clone,
    R: Rng + Debug + Clone,
{
    /// Constructs a new `FiniteMarkovChain<T, W, R>`.
    /// 
    /// # Panics
    /// 
    /// # Panics
    ///
    /// This method panics if: 
    /// - (In debug mode only) The dimensions of `state_space` and `transition_matrix` do not match.
    /// - Any vector of `transition_matrix` has more than u32::MAX columns.
    /// - For any entry w of any vector of `transition_matrix` v: 
    /// w < 0 or w > max where max = W::MAX / v.len().
    /// - For any vector of `transition_matrix` the sum of weights is zero.
    #[inline]
    pub fn new(
        state_index: usize,
        transition_matrix: Vec<Vec<W>>,
        state_space: Vec<T>,
        rng: R,
    ) -> Self {
        debug_assert_eq!(transition_matrix.len(), state_space.len());
        let transition_matrix_variables = transition_matrix.clone().into_iter()
        	.map(|v| WeightedAliasIndex::new(v).unwrap())
        	.collect();
        FiniteMarkovChain {
            state_index,
            transition_matrix,
            transition_matrix_variables,
            state_space,
            rng,
        }
    }

    #[inline]
    pub fn sample_index(&mut self) -> usize {
        self.transition_matrix_variables[self.state_index].sample(&mut self.rng)
    }

    #[inline]
    pub fn state_space(&self) -> &Vec<T> {
        &self.state_space
    }    

    #[inline]
    pub fn set_state_space<U>(self, new_state_space: Vec<U>) -> FiniteMarkovChain<U, W, R> 
    where
    	U: Debug + PartialEq + Clone,
    {
        FiniteMarkovChain{ 
		    state_index: self.state_index,
		    transition_matrix: self.transition_matrix,
		    transition_matrix_variables: self.transition_matrix_variables,
		    state_space: new_state_space,
		    rng: self.rng,
        }
    }

    /// Returns all absorbing state, if any.
    ///
    /// An absorbing state is a state such that, if the process starts there, 
    /// it will allways be there, i.e. the probability of moving to itself is one.
    #[inline]
    pub fn absorbing_states(&self) -> Vec<T> {
    	self.absorbing_states_index()
    		.iter()
    		.map(|&i| self.state_space()[i].clone())
    		.collect()
    }

    /// Returns the indexes indexes of all absorbing state, if any.
    ///
    /// An absorbing state is a state such that, if the process starts there, 
    /// it will allways be there, i.e. the probability of moving to itself is one.
    #[inline]
    pub fn absorbing_states_indexes(&self) -> Vec<usize> {
    	todo!()
    }

    /// Returns `true` if the Markov Chain may reach the state indexed by `query`, 
    /// from the current state.
    #[inline]
    pub fn may_achieve_index(&self, query: usize) -> bool {
    	todo!()
    }

    /// Returns `true` if the Markov Chain may reach the state `query`, 
    /// from the current state.
    #[inline]
    pub fn may_achieve(&self, query: T) -> bool {
    	todo!()
    }

    /// Returns `true` if the Markov Chain contains a recheable absorbing state, 
    /// from the current state.
    ///
    /// An absorbing state is a state such that, if the process starts there, 
    /// it will allways be there, i.e. the probability of moving to itself is one.
    /// A reacheable state is a state that can be reached with positive probability.
    #[inline]
    pub fn may_absorb(&self) -> bool {
    	self.absorbing_states_indexes()
    		.iter()
    		.map(|query| self.may_achieve_index(*query))
    		.any(|b| b)

    }
}

impl<T, W, R> State for FiniteMarkovChain<T, W, R>
where
    W: AliasableWeight + Debug + Clone,
    Uniform<W>: Debug + Clone,
    T: Debug + PartialEq + Clone,
    R: Rng + Debug + Clone,
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
    W: AliasableWeight + Debug + Clone,
    Uniform<W>: Debug + Clone,
    T: Debug + PartialEq + Clone,
    R: Rng + Debug + Clone,
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
    W: AliasableWeight + Debug + Clone,
    Uniform<W>: Debug + Clone,
    T: Debug + PartialEq + Clone,
    R: Rng + Debug + Clone,
{
    #[inline]
    fn state_as_item(&self) -> Option<<Self as std::iter::Iterator>::Item> {
        self.state().cloned()
    }
}

impl<T, W, R> Distribution<T> for FiniteMarkovChain<T, W, R>
where
    W: AliasableWeight + Debug + Clone,
    Uniform<W>: Debug + Clone,
    T: Debug + PartialEq + Clone,
    R: Rng + Debug + Clone,
{
    /// Sample a possible next state. 
    #[inline]
    fn sample<R2>(&self, rng: &mut R2) -> T
    where
        R2: Rng + ?Sized,
    { 
        let new_index = self.transition_matrix_variables[self.state_index].sample(rng);

        self.state_space[new_index].clone()
    }
}

impl<W, R> From<(usize, Vec<Vec<W>>, R)> for FiniteMarkovChain<usize, W, R>
where
    W: AliasableWeight + Debug + Clone,
    Uniform<W>: Debug + Clone,
    R: Rng + Debug + Clone,
{
	/// Performs the conversion.
	///
    /// # Panics
    ///
    /// This method panics if: 
    /// - Any vector of `transition_matrix` has more than u32::MAX columns.
    /// - For any entry w of any vector of `transition_matrix` v: 
    /// w < 0 or w > max where max = W::MAX / v.len().
    /// - For any vector of `transition_matrix` the sum of weights is zero.
    fn from((state_index, transition_matrix, rng): (usize, Vec<Vec<W>>, R)) -> Self {
        let state_space: Vec<usize> = (0..transition_matrix.len()).collect();
        FiniteMarkovChain::new(state_index, transition_matrix, state_space, rng)
    }
}

impl<T, W, R> From<(usize, ndarray::Array2<W>, Vec<T>, R)> for FiniteMarkovChain<T, W, R>
where
    W: AliasableWeight + Debug + Clone,
    Uniform<W>: Debug + Clone,
    T: Debug + PartialEq + Clone,
    R: Rng + Debug + Clone,
{
	/// Performs the conversion.
	///
    /// # Panics
    ///
    /// This method panics if: 
    /// - (In debug mode only) The dimensions of `state_space` and `transition_matrix` do not match.
    /// - `transition_matrix` has more than u32::MAX columns.
    /// - For any entry of `transition_matrix` w: 
    /// w < 0 or w > max where max = W::MAX / transition_matrix.ncols().
    /// - For any row of `transition_matrix` the sum of weights is zero.
	fn from((state_index, transition_matrix, state_space, rng): (usize, ndarray::Array2<W>, Vec<T>, R)) -> Self {
        let transition_matrix: Vec<Vec<W>> = transition_matrix.genrows()
            .into_iter()
            .map(|weights| {
                weights.to_vec()
            })
            .collect();
        FiniteMarkovChain::new(state_index, transition_matrix, state_space, rng)
    }
}

impl<W, R> From<(usize, ndarray::Array2<W>, R)> for FiniteMarkovChain<usize, W, R>
where
    W: AliasableWeight + Debug + Clone,
    Uniform<W>: Debug + Clone,
    R: Rng + Debug + Clone,
{
	/// Performs the conversion.
	///
    /// # Panics
    ///
    /// This method panics if: 
    /// - `transition_matrix` has more than u32::MAX columns.
    /// - For any entry of `transition_matrix` w: 
    /// w < 0 or w > max where max = W::MAX / transition_matrix.ncols().
    /// - For any row of `transition_matrix` the sum of weights is zero.
    ///
    /// # Example
    ///
    /// An absorbing Markov Chain with one transient state and one absorbing state.
    /// ```
    /// # use ndarray::array;
    /// # use markovian::FiniteMarkovChain;
    /// # use markovian::State;
    /// let mut mc = FiniteMarkovChain::from((0, array![[0.5, 0.5], [0.0, 1.0]], rand::thread_rng()));
    /// assert_eq!(mc.state(), Some(&0));
    /// println!("At time {}, the state is {}", 1_000, mc.nth(1_000).unwrap()); // Most likely 1
    /// ``` 
    fn from((state_index, transition_matrix, rng): (usize, ndarray::Array2<W>, R)) -> Self {
        let state_space: Vec<usize> = (0..transition_matrix.nrows()).collect();
        FiniteMarkovChain::from((state_index, transition_matrix, state_space, rng))
    }
}

#[cfg(test)]
mod tests {

    use test_case::test_case;
    use super::*;
    use rand::prelude::*;
    use ndarray::{array, Array2};

    #[test_case(0, Vec::new(), vec![1], thread_rng() => panics ""; "not enough transitions")]
    #[test_case(0, vec![Vec::new()], Vec::new(), thread_rng() => panics ""; "empty transition")]
    #[test_case(0, Vec::new(), Vec::new(), thread_rng(); "empty chain")]
    fn construction_vectors(state_index: usize, transition_matrix: Vec<Vec<usize>>, state_space: Vec<u64>, rng: rand::prelude::ThreadRng) {
        FiniteMarkovChain::new(state_index, transition_matrix, state_space, rng);
    }

    #[test_case(0, array![[]], vec![1], thread_rng() => panics ""; "not enough transitions")]
    #[test_case(0, array![[]], Vec::new(), thread_rng() => panics ""; "empty transition")]
    fn construction_array2(state_index: usize, transition_matrix: Array2<usize>, state_space: Vec<u64>, rng: rand::prelude::ThreadRng) {
        FiniteMarkovChain::from((state_index, transition_matrix, state_space, rng));
    }


    #[test]
    fn change_state() {
        let mut finite_mc = FiniteMarkovChain::new(0, vec![vec![1, 2], vec![2, 1]], vec![10, 20], thread_rng());
        let previous_state = finite_mc.set_state(20).unwrap();
        assert_eq!(Some(10), previous_state);
    }
}