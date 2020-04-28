//! Simulation of sub-stochastic processes.
//!
//! # Examples
//!
//! # Remarks
//!
//! All methods are `inline`, by design. 

pub use self::branching_process::BranchingProcess;
pub use self::continuous_markov_chain::ContMarkovChain;
pub use self::continuous_finite_markov_chain::ContFiniteMarkovChain;
pub use self::finite_markov_chain::FiniteMarkovChain;
pub use self::markov_chain::MarkovChain;
pub use self::traits::{ExponentialClock, State, StateIterator};

mod branching_process;
mod continuous_markov_chain;
mod continuous_finite_markov_chain;
mod finite_markov_chain;
mod markov_chain;
mod traits;

/// Ease interoperability with rand_distr crate.
pub mod distributions;
/// Errors of this crate.
pub mod errors;


/// Ease of use of this crate in general.
pub mod prelude {
    pub use crate::traits::*;
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
    pub fn rng(seed: u64) -> impl rand::RngCore {
        // For tests, we want a statistically good, fast, reproducible RNG.
        // PCG32 will do fine, and will be easy to embed if we ever need to.
        const INC: u64 = 11634580027462260723;
        rand_pcg::Pcg32::new(seed, INC)
    }
}
