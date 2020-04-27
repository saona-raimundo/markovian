// Traits
use crate::traits::{State, StateIterator, Transition};
use core::fmt::Debug;
use rand::Rng;

// Structs
use crate::errors::InvalidState;

// Functions
use core::mem;

/// # Remarks
///
/// The use of trait objects allows different distributions at each state.
#[derive(Debug, Clone)]
pub struct MarkovChain<T, F, R> {
    state: T,
    transition: F,
    rng: R,
}

impl<T, F, R> MarkovChain<T, F, R>
where
    R: Rng,
    F: Transition<T, T>,
{
    fn new(state: T, transition: F, rng: R) -> Self {
        MarkovChain {
            state,
            transition,
            rng,
        }
    }
}

impl<T, F, R> State for MarkovChain<T, F, R>
where
    T: Debug + Clone,
{
    type Item = T;

    #[inline]
    fn state(&self) -> Option<&Self::Item> {
        Some(&self.state)
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

impl<T, F, R> Iterator for MarkovChain<T, F, R>
where
    T: Debug + Clone,
    F: Transition<T, T>,
    R: Rng,
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.state = self.transition.sample_from(&self.state, &mut self.rng);
        self.state().cloned()
    }
}

impl<T, F, R> StateIterator for MarkovChain<T, F, R>
where
    T: Debug + Clone,
    F: Transition<T, T>,
    R: Rng,
{
    #[inline]
    fn state_as_item(&self) -> Option<<Self as std::iter::Iterator>::Item> {
        self.state().cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::distributions::Raw;
    use pretty_assertions::assert_eq;

    // use approx::abs_diff_eq;

    #[test]
    fn sampling_stability() {
        let rng = crate::test::rng(1);
        let expected = 1;
        let transition = |_: &u64| Raw::new(vec![(1.0, expected)]);
        let mc = MarkovChain::new(0, transition, rng);
        for x in mc.take(100) {
            assert_eq!(x, expected);
        }

        let rng = crate::test::rng(2);
        let transition = |_: &u64| Raw::new(vec![(0.5, 1), (0.5, 2)]);
        let mc = MarkovChain::new(0, transition, rng);
        for x in mc.take(100) {
            assert!(x == 1 || x == 2);
        }
    }

    #[test]
    fn value_stability() {
        let rng = crate::test::rng(3);
        let expected = vec![1, 2, 1, 1];
        let transition = |_: &u64| Raw::new(vec![(0.5, 1), (0.5, 2)]);
        let mc = MarkovChain::new(0, transition, rng);
        let sample: Vec<u64> = mc.take(4).collect();

        assert_eq!(sample, expected);
    }
}
