pub use branching::*;

pub mod branching;

use crate::traits::MarkovChainTrait;

/// Sub-stochastic Markov Chain.
// #[derive(Debug)]
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
    fn state(&self) -> &T {
        &self.state
    }
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
