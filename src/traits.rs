pub use self::exponential_clock::ExponentialClock;
pub use self::probability::Probability;
pub use self::state::State;
pub use self::state_iterator::StateIterator;
pub use self::transition::Transition;

mod exponential_clock;
mod probability;
mod state;
mod state_iterator;
mod transition;

// Implementation for interoperability with rand_distr. 
// use crate::macros::implement_distribution_once;