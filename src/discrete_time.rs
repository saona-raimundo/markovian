//! Discrete time Markovian processes. 
//!
//! # Remarks
//!
//! To define a continuous space process, you will need to make the 
//! transition function random. Otherwise, the state space is at most 
//! countable, for each initial condition. 

pub use branching::*;

pub mod branching;

use crate::traits::MarkovChainTrait;

/// Markov Chain which is sub-stochastic and time-homogeneous.
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
/// a density function over the state space by pairs of (state, probability). 
/// 
/// A MarkovChain can be seen as: 
/// - State-iterator: an iterator with a current state, changing randomly to another 
/// state when ``next`` method is called. See 
/// [Iterator](https://doc.rust-lang.org/nightly/core/iter/trait.Iterator.html)
/// and [MarkovChainTrait](file:///C:/Users/rasau/projects/markovian/target/doc/markovian/discrete_time/struct.MarkovChain.html) 
/// implementation. 
/// 
#[derive(Debug)]
pub struct MarkovChain<T, I, F>
where
    T: Clone,
    I: IntoIterator<Item = (T, f64)>,
    F: Fn(T) -> I,
{
    state: T,
    transition: F,
}

impl<T, I, F> MarkovChain<T, I, F>
where
    T: Clone,
    I: IntoIterator<Item = (T, f64)>,
    F: Fn(T) -> I,
{
    /// Creates a new MarkovChain. 
    /// 
    /// # Examples
    /// 
    /// Construction of a random walk in the integers, using a closure.
    /// ```
    /// # #![allow(unused_mut)]
    /// let init_state: i32 = 0;
    /// let transition = |state: i32| vec![(state + 1, 0.5), (state - 1, 0.5)];
    /// let mut mc = markovian::MarkovChain::new(init_state, &transition);
    /// ``` 
    /// Construction of a random walk in the integers, using a function.
    /// ```
    /// # #![allow(unused_mut)]
    /// let init_state: i32 = 0;
    /// fn transition(state: i32) -> Vec<(i32, f64)> { vec![(state + 1, 0.5),(state - 1, 0.5)] }
    /// let mut mc = markovian::MarkovChain::new(init_state, &transition);
    /// ``` 
    ///
    pub fn new(state: T, transition: F) -> Self {
        MarkovChain { state, transition }
    }
}

impl<T, I, F> MarkovChainTrait<T> for MarkovChain<T, I, F>
where
    T: Clone,
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

impl<T, I, F> Iterator for MarkovChain<T, I, F>
where
    T: Clone,
    I: IntoIterator<Item = (T, f64)>,
    F: Fn(T) -> I,
{
    type Item = T;

    /// Changes the state of the MarkovChain to a new state, chosen 
    /// according to the transition of the chain, and returns the new state. 
    /// 
    /// # Examples
    /// 
    ///  ```
    /// let init_state: i32 = 0;
    /// let transition = |state: i32| vec![(state + 1, 0.5), (state - 1, 0.5)];
    /// let mut mc = markovian::MarkovChain::new(init_state, &transition);
    ///
    /// // The next state is -1 or 1 with equal probability. 
    /// let new_state = mc.next();
    /// assert!( (new_state == Some(-1)) || (new_state == Some(1)) );
    /// 
    /// use markovian::traits::MarkovChainTrait;
    /// assert_eq!(new_state, Some(mc.state()).copied()) ;
    /// ``` 
    fn next(&mut self) -> Option<Self::Item> {
        let uniform: f64 = rand::random();
        let mut cummulative = 0.;

        let neigbours = (self.transition)(self.state.clone());
        for (state, prob) in neigbours {
            cummulative += prob;
            if uniform <= cummulative {
                self.state = state.clone();
                return Some(self.state.clone());
            }
        }
        None
    }
}
