//! Continuous time Markovian processes. 
//!
//! # Remarks
//!
//! To define a continuous space process, you will need to make the 
//! transition function random. Otherwise, the state space is at most 
//! countable, for each initial condition. 

// Traits

use crate::traits::CMarkovChainTrait;
use rand::distributions::weighted::alias_method::WeightedIndex;
use rand::distributions::Distribution;

// Structs

use rand_distr::Exp;

/// Continuous Markov Chain which is sub-stochastic and time-homogeneous.
/// 
/// By sub-stochastic we mean that the process is allowed to dissapear, i.e.,  
/// the transition matrix needs only be sub-stochastic. 
/// By time-homogeneous we mean that the transitions do not change over time. 
/// To implement a time dependent Markov process, include the time in the state varaible. 
/// 
/// It has three associated types for ease of use. 
///  - T, type of the values the process take. 
///  - F, function that gives, for each state, an iterator representing 
/// the corresponding transition to be used from that state. 
///  - I, something that can be converted into iterator that represents 
/// a rates of transition to other states, its elements are
/// pairs of (state, rate). Rates should be positive. 
/// 
/// A CMarkovChain can be seen as: 
/// - State-iterator: an iterator with a current state, changing randomly to another 
/// state when ``next`` method is called. See 
/// [Iterator](https://doc.rust-lang.org/nightly/core/iter/trait.Iterator.html)
/// and [MarkovChainTrait](file:///C:/Users/rasau/projects/markovian/target/doc/markovian/discrete_time/struct.MarkovChain.html) 
/// implementation. 
/// 
#[derive(Clone)]
pub struct CMarkovChain<T, I, F>
where
    T: Clone,
    I: IntoIterator<Item = (T, f64)>,
    F: Fn(T) -> I,
{
    state: T,
    transition: F,
}

impl<T, I, F> CMarkovChain<T, I, F>
where
    T: Clone,
    I: IntoIterator<Item = (T, f64)>,
    F: Fn(T) -> I,
{
    /// Creates a new CMarkovChain. 
    /// 
    /// # Examples
    /// 
    /// Construction of a random walk in the integers, using a closure.
    /// ```
    /// # #![allow(unused_mut)]
    /// let init_state: i32 = 0;
    /// let transition = |state: i32| vec![(state + 1, 1.), (state - 1, 1.)];
    /// let mut mc = markovian::CMarkovChain::new(init_state, &transition);
    /// ``` 
    /// Construction of a random walk in the integers, using a function.
    /// ```
    /// # #![allow(unused_mut)]
    /// let init_state: i32 = 0;
    /// fn transition(state: i32) -> Vec<(i32, f64)> { vec![(state + 1, 1.),(state - 1, 1.)] }
    /// let mut mc = markovian::CMarkovChain::new(init_state, &transition);
    /// ``` 
    ///
    pub fn new(state: T, transition: F) -> Self {
        CMarkovChain { state, transition }
    }
}

impl<T, I, F> CMarkovChainTrait<T> for CMarkovChain<T, I, F>
where
    T: Copy,
    I: IntoIterator<Item = (T, f64)>,
    F: Fn(T) -> I,
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

impl<T, I, F> Iterator for CMarkovChain<T, I, F>
where
    T: Clone,
    I: IntoIterator<Item = (T, f64)>,
    F: Fn(T) -> I,
{
    type Item = (f64, T);

    /// Changes the state of the MarkovChain to a new state, chosen 
    /// according to the transition of the chain, and returns the new state. 
    /// 
    /// # Examples
    /// 
    ///  ```
    /// let init_state: i32 = 0;
    /// let transition = |state: i32| vec![(state + 1, 1.0), (state - 1, 1.0)];
    /// let mut mc = markovian::CMarkovChain::new(init_state, &transition);
    ///
    /// // The next state is -1 or 1 with equal probability. 
    /// let (_t, new_state) = mc.next().expect("The chain dissapeared!");
    /// assert!( (new_state == -1) || (new_state == 1) );
    /// 
    /// use markovian::traits::CMarkovChainTrait;
    /// assert_eq!(&new_state, mc.state()) ;
    /// ``` 
    fn next(&mut self) -> Option<Self::Item> {
        let mut lambdas = Vec::new();
        let mut states = Vec::new();

        let neigbours = (self.transition)(self.state.clone());
        for (state, lambda) in neigbours {
            states.push(state);
            lambdas.push(lambda);
        }

        // Simulate time step

        let rate = lambdas.iter().sum();
        let exp = Exp::new(rate).unwrap();
        let time_step = exp.sample(&mut rand::thread_rng());

        // Choose between possible transitions

        let dist = WeightedIndex::new(lambdas).unwrap();
        let new_state = states[dist.sample(&mut rand::thread_rng())].clone();

        // Update chain

        self.state = new_state;

        Some((time_step, self.state.clone()))
    }
}
