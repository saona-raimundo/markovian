use core::iter::Chain;
use crate::State;

/// An iterator with an internal state that can be counted as the "first" element.
pub trait StateIterator: Iterator + State + Sized {

	/// If Iterator and State uses the same type, then a direct implementation is the following.
	/// ```no-run
	/// self.state().cloned().into_iter().chain(self)
	/// ```
	/// 
	/// # Remarks
	/// 1. Cloned is needed because it is the underlying iterator still needs it to give its
	/// first element.
    /// 2. You might want to use #[inline], if there is any benefit for you. 
    fn trajectory(self) -> Chain<std::option::IntoIter<<Self as std::iter::Iterator>::Item>, Self>; 
}