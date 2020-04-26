//! Simulation of sub-stochastic processes.
//! 
//! # Examples
//! 


pub use self::branching_process::BranchingProcess;
pub use self::finite_markov_chain::FiniteMarkovChain;
pub use self::traits::{ExponentialClock, State, StateIterator};

mod branching_process;
mod finite_markov_chain;
mod traits;

pub mod errors;

// pub use continuous_time::*;
// pub use discrete_time::*;
// pub use traits::*;

// mod continuous_time;
// mod discrete_time;
// mod traits;


pub mod prelude {
	pub use crate::traits::*;
}

