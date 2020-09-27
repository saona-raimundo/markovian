use crate::errors::InvalidState;

/// Public state.
/// 
/// `State` should be implemented when it is absolutely clear what a `state` for your your struct means.
/// 
/// # Examples
/// 
/// [Arithmetic sequence].
/// ```
/// struct ArithmeticSequence {
/// state: f64,
/// step: f64,
/// }
/// impl Iterator for ArithmeticSequence {
///     type Item = f64;
///     fn next(&mut self) -> Option<Self::Item> {
///         self.state += self.step;
///         Some(self.state)
///     }
/// }
/// impl markovian::State for ArithmeticSequence {
///     type Item = f64;
///     #[inline]
///     fn state(&self) -> Option<&Self::Item> {
///         Some(&self.state)
///     }
/// 
///     #[inline]
///     fn state_mut(&mut self) -> Option<&mut Self::Item> {
///         Some(&mut self.state)
///     }
///     #[inline]
///     fn set_state(
///         &mut self,
///         mut new_state: Self::Item,
///     ) -> Result<Option<Self::Item>, markovian::errors::InvalidState<Self::Item>> {
///         std::mem::swap(&mut self.state, &mut new_state); // Change places
///         Ok(Some(new_state)) // Return the previous state value
///     }
/// }
/// ```
///
/// [Arithmetic sequence]: https://en.wikipedia.org/wiki/Arithmetic_progression

pub trait State {
    type Item: core::fmt::Debug;

    #[inline]
    fn state(&self) -> Option<&Self::Item> {
        None
    }

    #[inline]
    fn state_mut(&mut self) -> Option<&mut Self::Item> {
        None
    }

    /// Changes the `state` of the struct.
    /// 
    /// If changing is succesful, this method should return the old state as an `Option<Self::Item>`.
    /// 
    /// # Remarks
    ///
    /// You might want to use [core::mem::swap](https://doc.rust-lang.org/core/mem/fn.swap.html).
    #[inline]
    fn set_state(
        &mut self,
        new_state: Self::Item,
    ) -> Result<Option<Self::Item>, InvalidState<Self::Item>> {
        Err(InvalidState::new(new_state))
    }
}
