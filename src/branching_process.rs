// Traits
use core::iter::Chain;
use rand_distr::Distribution;
use rand::Rng;
use crate::{State, StateIterator};
use core::fmt::Debug;
use num_traits::{One, Zero, sign::Unsigned};

// Structs
use crate::errors::InvalidState;

// Functions
use core::mem;

#[derive(Debug, Clone)]
pub struct BranchingProcess<T, D, R>
{
	state: T,
	base_distribution: D,
	rng: R,
}

impl<T, D, R> BranchingProcess<T, D, R> 
where
	T: Unsigned,
	D: Distribution<T>, 
	R: Rng,
{
	pub fn new(state: T, base_distribution: D, rng: R) -> Self {
		BranchingProcess {state, base_distribution, rng }
	}
}

impl<T, D, R> State for BranchingProcess<T, D, R> 
where
	T: std::fmt::Debug + Unsigned + Clone,
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

impl<T, D, R> Iterator for BranchingProcess<T, D, R> 
where
	T: Debug + PartialEq + Clone + One + Zero + PartialOrd,
	D: Distribution<T>,
	R: Rng,
{
	type Item = T;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
    	let count = T::one();
    	let mut acc = T::zero();
    	while count < self.state {
    		acc = acc + self.base_distribution.sample(&mut self.rng); 
    	}
		Some(acc)
	}
}

impl<T, D, R> StateIterator for BranchingProcess<T, D, R> 
where
	T: Debug + PartialEq + Clone + One + Zero + PartialOrd + Unsigned,
	D: Distribution<T>,
	R: Rng,
{

	#[inline]
    fn trajectory(self) -> Chain<std::option::IntoIter<<Self as std::iter::Iterator>::Item>, Self> {
    	self.state().cloned().into_iter().chain(self)
    }

}
