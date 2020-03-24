use crate::traits::CMarkovChainTrait;
use rand::distributions::weighted::alias_method::WeightedIndex;
use rand::distributions::Distribution;
use rand_distr::Exp;

/// Sub-stochastic continuous time Markov Chain: exponential clocks for each transition.
///
/// Only finite possible transitions are allowed.
// #[derive(Clone)]
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
    fn state(&self) -> &T {
        &self.state
    }

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
