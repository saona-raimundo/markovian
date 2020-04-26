use core::iter::Chain;
use crate::State;

/// An iterator with an internal state that could be thought as the "first" element.
pub trait StateIterator: Iterator + State + Sized {

	/// If Iterator and State uses the same type, then a direct implementation is the following.
	/// ```no-run
	/// self.state().cloned()
	/// ```
	/// 
	/// # Remarks
	/// 1. Cloned is needed if the underlying iterator depends on its internal state.
    /// 2. You might want to use #[inline], if there is any benefit for you. 
	fn state_as_item(&self) -> Option<<Self as std::iter::Iterator>::Item>; 

    #[inline]
    fn trajectory(self) -> Chain<std::option::IntoIter<<Self as std::iter::Iterator>::Item>, Self> {
    	self.state_as_item().into_iter().chain(self)
    }
}