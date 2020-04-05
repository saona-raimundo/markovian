//! Simulation of sub-stochastic Markov processes in discrete and continuous time.  
//! 
//! # Examples
//! 
//! ## Discrete time
//! 
//! Construction of a random walk in the integers, using a closure.
//! ```
//! # #![allow(unused_mut)]
//! let init_state: i32 = 0;
//! let transition = |state: i32| vec![(state + 1, 0.5), (state - 1, 0.5)];
//! let mut mc = markovian::MarkovChain::new(init_state, &transition);
//! ```  
//! 
//! ## Continuous time
//! 
//! Construction of a random walk in the integers, using a closure.
//! ```
//! # #![allow(unused_mut)]
//! let init_state: i32 = 0;
//! let transition = |state: i32| vec![(state + 1, 1.), (state - 1, 1.)];
//! let mut mc = markovian::CMarkovChain::new(init_state, &transition);
//! ``` 
//! 
//! ## Branching process
//!
//! Construction using density p(0) = 0.3, p(1) = 0.4, p(2) = 0.3. 
//! ```
//! # #![allow(unused_mut)]
//! let init_state: u32 = 1;
//! let density = vec![(0, 0.3), (1, 0.4), (2, 0.3)];
//! let mut branching_process = markovian::Branching::new(init_state, density);
//! ``` 
//! 
//! # Remarks
//! 
//! Non-trivial ways to use the crate are described below, including time dependence, 
//! continuous space and non-markovian processes.
//!
//! ## Time dependence
//!
//! Include the time as part of the state of the process. Then, the process is 
//! "homogeneous in time". 
//!
//! ### Examples
//! 
//! A random walk on the integers that tends to move more to the right as time goes by.  
//! ```
//! let init_state: (usize, i32) = (0, 0);
//! let transition = |(time, state): (usize, i32)| vec![
//! 	((time + 1, state + 1), 0.6 - 1.0 / (time + 1) as f64),
//! 	((time + 1, state - 1), 0.4 + 1.0 / (time + 1) as f64)
//! ];
//! let mc = markovian::MarkovChain::new(init_state, &transition);
//! 
//! // Take a sample of 10 elements 
//! mc.take(10).map(|(_, state)| state).collect::<Vec<i32>>();
//! ```
//! 
//! ## Continuous space
//!
//! Randomize the transition: return a random element together with a probability one
//! instead of returning a density over possible transtions in the form of an iterator.
//! Note that this will make some methods lose sense, since the transition no longer 
//! encodes all possible transitions.  
//!
//! ### Examples
//! 
//! A random walk on the real line with variable step size.  
//! ```
//! use markovian::MarkovChainTrait;
//! 
//! let init_state: f64 = 0.0;
//! let transition = |_| {
//! 	let next_state = rand::random::<f64>() * 2.0 - 1.0;
//! 	vec![(next_state, 1.0)]
//! };
//! let mut mc = markovian::MarkovChain::new(init_state, &transition);
//! mc.next();
//! let current_state: f64 = mc.state().clone();
//!  
//! // current_state is between -1.0 and 1.0 
//! assert!(current_state != 0.0);
//! assert!(current_state < 1.0 && current_state >= -1.0);
//! ```
//! 
//! ## Non markovian
//!
//! Include history in the state. For example, instead of a u32, let it be a Vec<u32>. 
//!
//! ### Examples
//! 
//! A random walk on the integers that is atracted to zero in a non markovian
//! fashion. 
//! ```
//! use markovian::MarkovChainTrait;
//! 
//! let init_state: Vec<i32> = vec![0];
//! let transition = |state: Vec<i32>| {
//! 	// New possible states
//! 	let mut right = state.clone();
//! 	right.push(state[state.len() - 1] + 1);
//! 	let mut left = state.clone();
//! 	left.push(state[state.len() - 1] - 1);
//! 
//! 	// Some non markovian transtion
//! 	let path_stadistic: i32 = state.iter().sum();
//! 	if path_stadistic.is_positive() {
//! 		vec![
//! 			(right, 1.0 / (path_stadistic.abs() + 1) as f64), 
//! 			(left, 1.0 - 1.0 / (path_stadistic.abs() + 1) as f64), 
//! 		]
//! 	} else {
//! 		vec![
//! 			(right, 1.0 - 1.0 / (path_stadistic.abs() + 1) as f64), 
//! 			(left, 1.0 / (path_stadistic.abs() + 1) as f64), 
//! 		]
//! 	}
//! };
//! let mut mc = markovian::MarkovChain::new(init_state, &transition);
//! mc.next();
//! let current_state = mc.state().clone();
//!  
//! // current_state has history
//! assert_eq!(current_state.len(), 2);
//! ```
//! 

pub use continuous_time::*;
pub use discrete_time::*;
pub use traits::*;

pub mod continuous_time;

pub mod discrete_time;

pub mod traits;



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
