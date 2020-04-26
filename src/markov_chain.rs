// Traits
use rand_distr::Distribution;
use rand::Rng;
use crate::{StateIterator, State};
use core::fmt::Debug;

// Structs
use crate::errors::InvalidState;
use core::marker::PhantomData;

// Functions
use core::mem;


/// # Remarks
/// 
/// The use of trait objects allows different distributions at each state.
#[derive(Debug, Clone)]
pub struct MarkovChain<T, F, D, R> 
{
	state: T,
	transition: F, 
	rng: R,
	phantom: PhantomData<D>,
}

impl<T, F, D, R> MarkovChain<T, F, D, R> 
where
	R: Rng,
	F: FnMut(&T) -> D,
	D: Distribution<T>,
{
	pub fn new(state: T, transition: F, rng: R) -> Self {
		MarkovChain { state, transition, rng, phantom: PhantomData }
	}
}

impl<T, F, D, R> State for MarkovChain<T, F, D, R>
where
	T: Debug + Clone, 
{
	type Item = T;

    #[inline]
    fn state(&self) -> Option<&Self::Item> {
    	Some(&self.state)
    }

    #[inline]
    fn set_state(&mut self, mut new_state: Self::Item) -> Result<Option<Self::Item>, InvalidState<Self::Item>> {
    	mem::swap(&mut self.state, &mut new_state);
		Ok(Some(new_state))
    }
}

impl<T, F, D, R> Iterator for MarkovChain<T, F, D, R>
where
	T: Debug + Clone,
	F: FnMut(&T) -> D,
	D: Distribution<T>,
	R: Rng,
{
	type Item = T;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		self.state = (self.transition)(&self.state).sample(&mut self.rng);
		self.state().cloned()
	}
}

impl<T, F, D, R> StateIterator for MarkovChain<T, F, D, R>
where
	T: Debug + Clone,
	F: FnMut(&T) -> D,
	D: Distribution<T>,
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
    use pretty_assertions::assert_eq;
    use crate::distributions::Raw;
    // use approx::abs_diff_eq;

    #[test]
    fn sampling_stability() {
        let rng = crate::test::rng(1);
        let expected = 1;
        let mc = MarkovChain::new(0, |_| Raw::from(vec![(1.0, expected)]), rng);
        for x in mc.take(100) {
        	assert_eq!(x, expected);
        }

        let rng = crate::test::rng(2);
        let mc = MarkovChain::new(0, |_| Raw::from(vec![(0.5, 1), (0.5, 2)]), rng);
        for x in mc.take(100) {
        	assert!(x == 1 || x == 2);
        }
    }

    #[test]
    fn value_stability() {
        let rng = crate::test::rng(3);
        let expected = vec![1, 2, 1, 1];
        let mc = MarkovChain::new(0, |_| Raw::from(vec![(0.5, 1), (0.5, 2)]), rng);
        let sample: Vec<u64> = mc.take(4).collect();

        assert_eq!(sample, expected);
    }
}