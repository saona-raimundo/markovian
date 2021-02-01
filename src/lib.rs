//! Simulation of (sub-)stochastic processes.
//!
//! # Goal
//!
//! Serve as an extension of the [rand crate](https://crates.io/crates/rand) for sub-stochastic processes.
//! 
//! # Examples
//!
//! ## Finite Markov Chains
//!
//! An absorbing Markov Chain with one transient state and one absorbing state.
//! ```
//! # use ndarray::array;
//! # use markovian::State;
//! let mut mc = markovian::FiniteMarkovChain::from((0, array![[0.5, 0.5], [0.0, 1.0]], rand::thread_rng()));
//! assert_eq!(mc.state(), Some(&0));
//! assert_eq!(mc.state_space(), &vec![0, 1]);
//! println!("At time {}, the state is {}", 1_000, mc.nth(1_000).unwrap()); // Most likely 1
//! ``` 
//!
//! ## Discrete time
//! 
//! Construction of a random walk in the integers.
//! ```
//! # #![allow(unused_mut)]
//! # use markovian::prelude::*;
//! # use rand::prelude::*;
//! let init_state: i32 = 0;
//! let transition = |state: &i32| raw_dist![(0.5, state + 1), (0.5, state - 1)];
//! let rng = thread_rng();
//! let mut mc = markovian::MarkovChain::new(init_state, transition, rng);
//! ```  
//! 
//! ## Branching process
//!
//! Construction using density p(0) = 0.3, p(1) = 0.4, p(2) = 0.3. 
//! ```
//! # #![allow(unused_mut)]
//! # use markovian::prelude::*;
//! # use rand::prelude::*;
//! let init_state: u32 = 1;
//! let base_distribution = raw_dist![(0.3, 0), (0.4, 1), (0.3, 2)];
//! let rng = thread_rng();
//! let mut branching_process = markovian::BranchingProcess::new(init_state, base_distribution, rng);
//! ``` 
//! ## Continuous time
//! 
//! Construction of a random walk in the integers, with expponential time for each transition.
//! ```
//! # #![allow(unused_mut)]
//! # use rand::prelude::*;
//! # use rand_distr::{Exp, Uniform};
//! # use markovian::prelude::*;
//! let init_state: i32 = 0;
//! struct MyTransition;
//! impl markovian::Transition<i32, (f64, i32)> for MyTransition {
//!     fn sample_from<R: ?Sized>(&self, state: &i32, rng: &mut R) -> (f64, i32)
//!     where
//!         R: Rng
//!     {
//!         let time = Exp::new(2.0).unwrap().sample(rng);
//!         let step = Uniform::from(0..=1).sample(rng) * 2 - 1;
//!         (time, state + step)
//!     }
//! }
//! let transition = MyTransition;
//! let rng = thread_rng();
//! let mut mc = markovian::TimedMarkovChain::new(init_state, transition, rng);
//! ``` 
//!
//! # Remarks
//!
//! All methods are `inline`, by design.
//! 
//! Non-trivial ways to use the crate are described below, including time dependence, 
//! continuous space and non-markovian processes.
//!
//! ## Time dependence
//!
//! Include the time as part of the state of the process.
//!
//! ### Examples
//! 
//! A random walk on the integers that tends to move more to the right as time goes by.  
//! ```
//! # #![allow(unused_mut)]
//! # use rand::prelude::*;
//! # use rand_distr::{Exp, Uniform};
//! # use markovian::prelude::*;
//! let init_state: (usize, i32) = (0, 0);
//! let transition = |(time, state): &(usize, i32)| raw_dist![
//!     (0.6 - 1.0 / (time + 2) as f64, (time + 1, state + 1)),
//!     (0.4 + 1.0 / (time + 2) as f64, (time + 1, state - 1))
//! ];
//! let rng = thread_rng();
//! let mut mc = markovian::MarkovChain::new(init_state, &transition, rng);
//! 
//! // Take a sample of 10 elements 
//! mc.take(10).map(|(_, state)| state).collect::<Vec<i32>>();
//! ```
//! 
//! ## Continuous space
//!
//! Randomize the transition: return a random element together with a probability one
//!
//! ### Examples
//! 
//! A random walk on the real line with variable step size.  
//! ```
//! # use rand_distr::Exp;
//! # use rand::prelude::*;
//! # use markovian::prelude::*;
//! let init_state: f64 = 0.0;
//! struct MyTransition;
//! impl markovian::Transition<f64, f64> for MyTransition {
//!     fn sample_from<R: ?Sized>(&self, state: &f64, rng: &mut R) -> f64
//!     where
//!         R: Rng
//!     {
//!         let step = Exp::new(2.0).unwrap().sample(rng);
//!         state + step
//!     }
//! }
//! let transition = MyTransition;
//! let rng = thread_rng();
//! let mut mc = markovian::MarkovChain::new(init_state, transition, rng);
//! mc.next();
//!  
//! // current_state is positive 
//! assert!(mc.state().unwrap() > &0.0);
//! ```
//! 
//! ## Non markovian
//!
//! Include history in the state. For example, instead of `i32`, use `Vec<i32>`. 
//!
//! ### Examples
//! 
//! A random walk on the integers that is atracted to zero in a non markovian
//! fashion. 
//! ```
//! # use rand::prelude::*;
//! # use markovian::prelude::*;
//! let init_state: Vec<i32> = vec![0];
//! let transition = |state: &Vec<i32>| {
//!     // New possible states
//!     let mut right = state.clone();
//!     right.push(state[state.len() - 1] + 1);
//!     let mut left = state.clone();
//!     left.push(state[state.len() - 1] - 1);
//! 
//!     // Some non markovian transtion
//!     let path_stadistic: i32 = state.iter().sum();
//!     if path_stadistic.is_positive() {
//!         raw_dist![
//!             (1.0 / (path_stadistic.abs() + 1) as f64, right), 
//!             (1.0 - 1.0 / (path_stadistic.abs() + 1) as f64, left)
//!         ]
//!     } else {
//!         raw_dist![
//!             (1.0 - 1.0 / (path_stadistic.abs() + 1) as f64, right), 
//!             (1.0 / (path_stadistic.abs() + 1) as f64, left)
//!         ]
//!     }
//! };
//! let rng = thread_rng();
//! let mut mc = markovian::MarkovChain::new(init_state, transition, rng);
//!  
//! // state has history
//! mc.next();
//! assert_eq!(mc.state().unwrap().len(), 2);
//! ```
//! 
pub use self::branching_process::BranchingProcess;
pub use self::continuous_finite_markov_chain::ContFiniteMarkovChain;
pub use self::finite_markov_chain::FiniteMarkovChain;
pub use self::markov_chain::MarkovChain;
pub use self::timed_markov_chain::TimedMarkovChain;
pub use self::traits::{State, StateIterator, Transition};

mod branching_process;
mod continuous_finite_markov_chain;
mod finite_markov_chain;
mod markov_chain;
mod timed_markov_chain;
mod traits;
mod macros;

/// Ease interoperability with rand_distr crate.
pub mod distributions;
/// Errors of this crate.
pub mod errors;

/// Ease of use of this crate in general.
pub mod prelude {
    pub use crate::traits::*;
    pub use crate::{raw_dist};
    pub use crate::distributions::Raw;
}


/// Testing random variables.
#[cfg(test)]
pub mod tests {
    // Notes on testing
    //
    // Testing random number distributions correctly is hard. The following
    // testing is desired:
    //
    // - Construction: test initialisation with a few valid parameter sets.
    // - Erroneous usage: test that incorrect usage generates an error.
    // - Vector: test that usage with fixed inputs (including RNG) generates a
    //   fixed output sequence on all platforms.
    // - Correctness at fixed points (optional): using a specific mock RNG,
    //   check that specific values are sampled (e.g. end-points and median of
    //   distribution).
    // - Correctness of PDF (extra): generate a histogram of samples within a
    //   certain range, and check this approximates the PDF. These tests are
    //   expected to be expensive, and should be behind a feature-gate.
    //
    // TODO: Vector and correctness tests are largely absent so far.
    // NOTE: Some distributions have tests checking only that samples can be
    // generated. This is redundant with vector and correctness tests.

    /// Construct a deterministic RNG with the given seed
    pub fn rng(seed: u64) -> impl rand::Rng {
        // For tests, we want a statistically good, fast, reproducible RNG.
        // PCG32 will do fine, and will be easy to embed if we ever need to.
        const INC: u64 = 11634580027462260723;
        rand_pcg::Pcg32::new(seed, INC)
    }
}
