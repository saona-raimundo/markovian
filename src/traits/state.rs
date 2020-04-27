use crate::errors::InvalidState;

pub trait State {
    type Item: core::fmt::Debug;

    #[inline]
    fn state(&self) -> Option<&Self::Item> {
        None
    }

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
