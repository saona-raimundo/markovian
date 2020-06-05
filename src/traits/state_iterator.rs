use crate::State;
use core::iter::Chain;

/// Iterator with an internal state that is thought as the "zero" element.
pub trait StateIterator: Iterator + State + Sized {
    /// If Iterator and State uses the same type, then a direct implementation is the following.
    /// ```no-run
    /// self.state().cloned()
    /// ```
    ///
    /// # Remarks
    /// 1. Cloned is needed if the underlying iterator depends on its internal state.
    /// 2. You should use #[inline] when implementing this method.
    fn state_as_item(&self) -> Option<<Self as std::iter::Iterator>::Item>;

    /// Returns a new iterator whose first element is the state (seen as an item of the Iteratior) 
    /// and then follows with the iterator. 
    #[inline]
    fn trajectory(self) -> Chain<std::option::IntoIter<<Self as std::iter::Iterator>::Item>, Self> {
        self.state_as_item().into_iter().chain(self)
    }
}
