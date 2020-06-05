// Traits
use crate::{State, StateIterator};
use core::fmt::Debug;
use num_traits::{sign::Unsigned, One, Zero};
use rand::Rng;
use rand_distr::Distribution;

// Structs
use crate::errors::InvalidState;

// Functions
use core::mem;

#[derive(Debug, Clone)]
pub struct BranchingProcess<T, D, R> 
where
    T: Debug + PartialEq + Clone + One + Zero + PartialOrd + Unsigned,
    D: Distribution<T>,
    R: Rng,
{
    state: T,
    base_distribution: D,
    rng: R,
}

impl<T, D, R> BranchingProcess<T, D, R>
where
    T: Debug + PartialEq + Clone + One + Zero + PartialOrd + Unsigned,
    D: Distribution<T>,
    R: Rng,
{
    #[inline]
    pub fn new(state: T, base_distribution: D, rng: R) -> Self {
        BranchingProcess {
            state,
            base_distribution,
            rng,
        }
    }
}

impl<T, D, R> State for BranchingProcess<T, D, R>
where
    T: Debug + PartialEq + Clone + One + Zero + PartialOrd + Unsigned,
    D: Distribution<T>,
    R: Rng,
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

impl<T, D, R> Iterator for BranchingProcess<T, D, R>
where
    T: Debug + PartialEq + Clone + One + Zero + PartialOrd + Unsigned,
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
        self.state = acc.clone();
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
    fn state_as_item(&self) -> Option<<Self as std::iter::Iterator>::Item> {
        self.state().cloned()
    }
}

impl<T, D, R> Distribution<T> for BranchingProcess<T, D, R>
where
    T: Debug + PartialEq + Clone + One + Zero + PartialOrd + Unsigned,
    D: Distribution<T>,
    R: Rng,
{
    /// Sample a possible next state. 
    #[inline]
    fn sample<R2>(&self, rng: &mut R2) -> T 
    where
        R2: Rng + ?Sized,
    { 
        let count = T::one();
        let mut acc = T::zero();
        while count < self.state {
            acc = acc + self.base_distribution.sample(rng);
        }
        acc
    }
}

