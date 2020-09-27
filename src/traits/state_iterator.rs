use crate::State;
use core::iter::Chain;





/// Iterator with an internal state. 
/// 
/// Internals states can be thought as the "initial" element of an iterator, see [trajectory].
/// 
/// # Example
/// 
/// Usual case: A struct that uses  the same type for [Iterator] and [State] traits.
/// ```
/// # struct MyStateIterator;
/// # impl Iterator for MyStateIterator {
/// #     type Item = f64;
/// #     fn next(&mut self) -> Option<Self::Item> {
/// #         None
/// #     }
/// # }
/// # impl markovian::State for MyStateIterator {
/// #     type Item = f64;
/// # }
/// # use markovian::State;
/// impl markovian::StateIterator for MyStateIterator {
///     #[inline]
///     fn state_as_item(&self) -> Option<<Self as std::iter::Iterator>::Item> {
///         self.state().cloned()
///     }
/// }
/// ```
///
/// [trajectory]: trait.StateIterator.html#method.trajectory
/// [Iterator]: https://doc.rust-lang.org/nightly/core/iter/trait.Iterator.html
/// [State]: trait.State.html
pub trait StateIterator: Iterator + State + Sized {
    /// # Remarks
    /// 
    /// You should use ``#[inline]`` when implementing this method.
    fn state_as_item(&self) -> Option<<Self as std::iter::Iterator>::Item>;

    /// Returns a new iterator whose first element is the state (seen as an item of the Iterator) 
    /// and then follows with the elements of the iterator. 
    #[inline]
    fn trajectory(self) -> Chain<std::option::IntoIter<<Self as std::iter::Iterator>::Item>, Self> {
        self.state_as_item().into_iter().chain(self)
    }
}
