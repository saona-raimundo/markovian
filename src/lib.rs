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

pub use continuous_time::*;
pub use discrete_time::*;

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
