// Traits
use crate::traits::{State, StateIterator, Transition};
use core::fmt::Debug;
use rand::Rng;

// Structs
use crate::errors::InvalidState;
use core::marker::PhantomData;

// Functions
use core::mem;

/// Markov Chain in continuous time, with arbitrary space.
///
/// Rust allows for more than only exponential time in the transitions, so 
/// does this crate. 
/// 
/// # Remarks
/// 
/// If your transition function `transition` could reuse of structs that implement
/// the `Distribution<T>` trait in order to sample the next state, then, 
/// for the best performance possible, create your own struct that implements
/// the `Transition<T, (N, T)>` trait.
#[derive(Debug, Clone)]
pub struct TimedMarkovChain<N, T, F, R> {
    state: T,
    transition: F,
    rng: R,
    phantom: PhantomData<N>,
}

impl<N, T, F, R> TimedMarkovChain<N, T, F, R>
where
    R: Rng,
    F: Transition<T, (N, T)>,
    N: From<f64>,
{
    #[inline]
    pub fn new(state: T, transition: F, rng: R) -> Self {
        TimedMarkovChain {
            state,
            transition,
            rng,
            phantom: PhantomData,
        }
    }
}

impl<N, T, F, R> State for TimedMarkovChain<N, T, F, R>
where
    T: Debug + Clone,
{
    type Item = T;

    #[inline]
    fn state(&self) -> Option<&Self::Item> {
        Some(&self.state)
    }

    #[inline]
    fn state_mut(&mut self) -> Option<&mut Self::Item> {
        Some(&mut self.state)
    }

    #[inline]
    fn set_state(
        &mut self,
        mut new_state: Self::Item,
    ) -> Result<Option<Self::Item>, InvalidState<Self::Item>> {
        mem::swap(&mut self.state, &mut new_state);
        Ok(Some(new_state))
    }
}

impl<N, T, F, R> Iterator for TimedMarkovChain<N, T, F, R>
where
    T: Debug + Clone,
    F: Transition<T, (N, T)>,
    R: Rng,
{
    type Item = (N, T);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let (period, state) = self.transition.sample_from(&self.state, &mut self.rng);
        self.state = state;
        self.state().cloned().map(|state| (period, state))
    }
}

impl<N, T, F, R> StateIterator for TimedMarkovChain<N, T, F, R>
where
    T: Debug + Clone,
    F: Transition<T, (N, T)>,
    R: Rng,
    N: From<f64>,
{
    #[inline]
    fn state_as_item(&self) -> Option<<Self as std::iter::Iterator>::Item> {
        self.state().cloned().map(|state| (N::from(0.0), state))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::distributions::Raw;
    // use pretty_assertions::assert_eq;

    // use approx::abs_diff_eq;

    #[test]
    fn sampling_stability() {
        let rng = crate::tests::rng(1);
        let expected = 1;
        let transition = |_: &u64| Raw::new(vec![(1.0, (1.0, expected))]);
        let mc = TimedMarkovChain::new(0, transition, rng);
        for (period, state) in mc.take(100) {
            assert_eq!(period, 1.);
            assert_eq!(state, expected);
        }

        let rng = crate::tests::rng(2);
        let transition = |_: &u64| Raw::new(vec![(0.5, (1.0, 1)), (0.5, (1.0, 2))]);
        let mc = TimedMarkovChain::new(0, transition, rng);
        for (period, state) in mc.take(100) {
            assert_eq!(period, 1.);
            assert!(state == 1 || state == 2);
        }
    }

    #[test]
    fn value_stability() {
        let rng = crate::tests::rng(3);
        let expected = vec![(1., 1), (1., 2), (1., 1), (1., 1)];
        let transition = |_: &u64| Raw::new(vec![(0.5, (1.0, 1)), (0.5, (1.0, 2))]);
        let mc = TimedMarkovChain::new(0, transition, rng);
        let sample: Vec<(f64, u64)> = mc.take(4).collect();

        assert_eq!(sample, expected);
    }
}
