// Traits
use crate::{State, StateIterator};
use core::fmt::Debug;
use rand::Rng;
use rand_distr::{weighted_alias::{WeightedAliasIndex, AliasableWeight}, Uniform, Distribution};

// Structs
use crate::errors::InvalidState;
use petgraph::graph::DiGraph;

// Functions
use core::mem;

/// Finite state Markov Chain in discrete time. 
/// 
/// # Costs
/// 
/// **Construction**: O(n^2), where n is the size of the state space.
/// 
/// **Sample**: O(1).
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
    /// - The`state_space` vector has repeated elements
    /// (defined by PartialEq).
    /// - The dimensions of `state_space` and `transition_matrix` do not match.
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
        let transition_matrix_variables = transition_matrix.clone().into_iter()
        	.map(|v| WeightedAliasIndex::new(v).unwrap())
        	.collect();

        FiniteMarkovChain::new_raw(
            state_index,
            transition_matrix,
            transition_matrix_variables,
            state_space,
            rng
        )
    }

    #[inline]
    fn new_raw(
        state_index: usize,
        transition_matrix: Vec<Vec<W>>,
        transition_matrix_variables: Vec<WeightedAliasIndex<W>>,     
        state_space: Vec<T>,
        rng: R,
    ) -> Self {
        let state_space_len_true: usize = state_space.iter()
            .map(|x| state_space.iter().filter(|&y| x == y).count())
            .sum();
        assert_eq!(state_space_len_true, state_space.len());
        assert_eq!(transition_matrix.len(), state_space.len());
        FiniteMarkovChain {
            state_index,
            transition_matrix,
            transition_matrix_variables,
            state_space,
            rng,
        }
    }

    /// Samples a possible index for the next state.
    ///
    /// # Remarks
    ///
    /// Although the state the Markov Chain does not change, 
    /// its random number generator does. That is why this method needs `&mut self`.
    ///
    /// # Examples
    ///
    /// From the current state, the next index has equal probability of being `0` or `1`.
    /// ```
    /// # use ndarray::array;
    /// # use markovian::{FiniteMarkovChain, State};
    /// let mut mc = FiniteMarkovChain::from((0, array![[0.5, 0.5], [0.0, 1.0]], rand::thread_rng()));
    /// println!("The next index could be {}", mc.sample_index());  // 50% 0 and 50% 1.
    /// ```
    #[inline]
    pub fn sample_index(&mut self) -> usize {
        self.transition_matrix_variables[self.state_index].sample(&mut self.rng)
    }

    /// Returns the state space of the Markov Chain.
    ///
    /// The state space is the collection of all values the chain might ever take,
    /// even if they are not recheable from the current state.
    ///
    /// # Examples
    ///
    /// The state space can be more than one state, 
    /// even if the Markov Chain is already absorbed. 
    /// ```
    /// # use ndarray::array;
    /// # use markovian::{FiniteMarkovChain, State};
    /// let mc = FiniteMarkovChain::from((1, array![[0.5, 0.5], [0.0, 1.0]], rand::thread_rng()));
    /// assert_eq!(mc.state_space(), &vec![0, 1]);
    /// ```
    #[inline]
    pub fn state_space(&self) -> &Vec<T> {
        &self.state_space
    }    

    /// Returns the size of the state space.
    ///
    /// The state space is the collection of all values the chain might ever take,
    /// even if they are not recheable from the current state.
    ///
    /// # Examples
    ///
    /// A Markov Chain with two states. 
    /// ```
    /// # use ndarray::array;
    /// # use markovian::FiniteMarkovChain;
    /// let mc = FiniteMarkovChain::from((1, array![[0.5, 0.5], [0.0, 1.0]], rand::thread_rng()));
    /// assert_eq!(mc.nstates(), 2);
    /// ```
    #[inline]
    pub fn nstates(&self) -> usize {
        self.state_space().len()
    }   

    /// Changes the state space of the Markov Chain.
    ///
    /// The state space is the collection of all values the chain might ever take,
    /// even if they are not recheable from the current state.
    ///
    /// # Panics
    ///
    /// In debug mode, if `new_state_space` is not as long as the current state space.  
    ///
    /// # Examples
    ///
    /// Changing from numbers to letters.
    /// ```
    /// # use ndarray::array;
    /// # use markovian::{FiniteMarkovChain, State};
    /// let mc = FiniteMarkovChain::from((1, array![[0.5, 0.5], [0.0, 1.0]], rand::thread_rng()));
    /// assert_eq!(mc.state(), Some(&1));
    /// let mc = mc.set_state_space(vec!['a', 'b']);
    /// assert_eq!(mc.state(), Some(&'b'));
    /// ```
    #[inline]
    pub fn set_state_space<U>(self, new_state_space: Vec<U>) -> FiniteMarkovChain<U, W, R> 
    where
    	U: Debug + PartialEq + Clone,
    {
        FiniteMarkovChain::new_raw( 
		    self.state_index,
		    self.transition_matrix,
		    self.transition_matrix_variables,
		    new_state_space,
		    self.rng,
        )
    }

    /// Returns all absorbing state, if any.
    ///
    /// An absorbing state is a state such that, if the process starts there, 
    /// it will allways be there, i.e. the probability of moving to itself is one.
    ///
    /// # Examples
    ///
    /// There is one absorbing state: state `b`.
    /// ```
    /// # use ndarray::array;
    /// # use markovian::{FiniteMarkovChain, State};
    /// let mc = FiniteMarkovChain::from((0, array![[0.5, 0.5], [0.0, 1.0]], rand::thread_rng()))
    ///     .set_state_space(vec!['a', 'b']);
    /// assert_eq!(mc.absorbing_states(), vec![&'b']);
    /// ```
    #[inline]
    pub fn absorbing_states(&self) -> Vec<&T> {
    	self.absorbing_states_indexes()
    		.iter()
    		.map(|&i| &self.state_space()[i])
    		.collect()
    }

    /// Returns the indexes indexes of all absorbing state, if any.
    ///
    /// An absorbing state is a state such that, if the process starts there, 
    /// it will allways be there, i.e. the probability of moving to itself is one.
    ///
    /// # Examples
    ///
    /// There is one absorbing state: state `b`, which has index `1`.
    /// ```
    /// # use ndarray::array;
    /// # use markovian::{FiniteMarkovChain, State};
    /// let mc = FiniteMarkovChain::from((0, array![[0.5, 0.5], [0.0, 1.0]], rand::thread_rng()))
    ///     .set_state_space(vec!['a', 'b']);
    /// assert_eq!(mc.absorbing_states_indexes(), vec![1]);
    /// ```
    #[inline]
    pub fn absorbing_states_indexes(&self) -> Vec<usize> {
        let transition_matrix = &self.transition_matrix;
    	(0..self.state_space.len())
            .filter(|&i| {
                let quantities_check = transition_matrix[i].iter()
                    .enumerate()
                    .map(|(j, w)| {
                        if j == i {
                            w > &W::ZERO
                        } else {
                            w == &W::ZERO
                        }
                    })
                    .all(|b| b);
                let existence_check = transition_matrix[i].len() > i;
                quantities_check && existence_check
            })
            .collect()

    }

    /// Returns `true` if the Markov Chain may reach the state indexed by `query`, 
    /// from the current state.
    ///
    /// # Examples
    ///
    /// Checking the possibility of achieving a state from different initial states.
    /// ```
    /// # use ndarray::array;
    /// # use markovian::{FiniteMarkovChain, State};
    /// let mut mc = FiniteMarkovChain::from((0, array![[0.5, 0.5], [0.0, 1.0]], rand::thread_rng()));
    /// assert!(mc.may_achieve_index(0));
    /// assert!(mc.may_achieve_index(1));
    /// mc.set_state(1);
    /// assert!(!mc.may_achieve_index(0));
    /// assert!(mc.may_achieve_index(1));
    /// ```
    #[inline]
    pub fn may_achieve_index(&self, query: usize) -> bool {
    	let (graph, node) = self.clone().into();
        let mut bfs = petgraph::visit::Bfs::new(&graph, node);
        while let Some(other_node) = bfs.next(&graph) {
            if other_node.index() == query {
                return true
            } 
        }
        false
    }

    /// Returns `true` if the Markov Chain may reach the state `query`, 
    /// from the current state.
    ///
    /// # Examples
    ///
    /// Checking the possibility of achieving a state from different initial states.
    /// ```
    /// # use ndarray::array;
    /// # use markovian::{FiniteMarkovChain, State};
    /// let mut mc = FiniteMarkovChain::from((0, array![[0.5, 0.5], [0.0, 1.0]], rand::thread_rng()))
    ///     .set_state_space(vec!['x', 'y']);
    /// assert!(mc.may_achieve('x').unwrap());
    /// assert!(mc.may_achieve('y').unwrap());
    /// mc.set_state('y');
    /// assert!(!mc.may_achieve('x').unwrap());
    /// assert!(mc.may_achieve('y').unwrap());
    /// ```
    #[inline]
    pub fn may_achieve(&self, query: T) -> Result<bool, InvalidState<T>> {
        match self.state_space.iter().position(|s| *s == query) {
            Some(state_index) => {
                Ok(self.may_achieve_index(state_index))
            },
            None => Err(InvalidState::new(query)),
        }
    }

    /// Returns `true` if the Markov Chain contains a recheable absorbing state, 
    /// from the current state.
    ///
    /// An absorbing state is a state such that, if the process starts there, 
    /// it will allways be there, i.e. the probability of moving to itself is one.
    /// A reacheable state is a state that can be reached with positive probability.
    ///
    /// # Examples
    ///
    /// Checking the possibility of achieving a state from different initial states.
    /// ```
    /// # use ndarray::array;
    /// # use markovian::{FiniteMarkovChain, State};
    /// let mut mc = FiniteMarkovChain::from((0, array![[0.5, 0.5], [0.0, 1.0]], rand::thread_rng()));
    /// assert!(mc.may_absorb());
    /// ```    
    #[inline]
    pub fn may_absorb(&self) -> bool {
        let set: std::collections::HashSet<_> = self.absorbing_states_indexes().into_iter().collect();
        let (graph, node) = self.clone().into();
        let mut bfs = petgraph::visit::Bfs::new(&graph, node);
        while let Some(other_node) = bfs.next(&graph) {
            if set.contains(&other_node.index()) {
                return true
            } 
        }
        false
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

impl<T, W, R> Into<(DiGraph<T, W>, petgraph::graph::NodeIndex)> for FiniteMarkovChain<T, W, R>
where
    W: AliasableWeight + Debug + Clone,
    Uniform<W>: Debug + Clone,
    T: Debug + PartialEq + Clone,
    R: Rng + Debug + Clone,
{
    /// Performs the conversion.
    ///
    /// # Examples
    ///
    /// An absorbing Markov Chain with one transient state and one absorbing state.
    /// ```
    /// # use ndarray::array;
    /// # use markovian::{FiniteMarkovChain, State};
    /// # use petgraph::graph::DiGraph;
    /// let mc = FiniteMarkovChain::from((0, array![[0.5, 0.5], [0.0, 1.0]], rand::thread_rng()));
    /// let (graph, node) = mc.into();
    /// assert_eq!(graph[node], 0);
    /// assert_eq!(graph.neighbors(node).count(), 2);
    /// assert_eq!(graph.edge_count(), 3);
    /// assert_eq!(graph.node_count(), 2);
    /// ``` 
    fn into(self) -> (DiGraph<T, W>, petgraph::graph::NodeIndex) { 
        let mut graph = DiGraph::<T, W>::new();
        let vertices: Vec<_> = self.state_space.iter()
            .map(|state| graph.add_node(state.clone()))
            .collect();
        for i in 0..self.nstates() {
            for j in 0..self.transition_matrix[i].len() {
                if self.transition_matrix[i][j] > W::ZERO {
                    graph.add_edge(vertices[i], vertices[j], self.transition_matrix[i][j]);
                }
            }
        }
        (graph, petgraph::graph::NodeIndex::new(self.state_index))
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