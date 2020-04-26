use thiserror::Error;

#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Error)]
#[error("the state {state:?} is not a valid assignation")]
pub struct InvalidState<T: std::fmt::Debug> {
	state: T,
}

impl<T: std::fmt::Debug> InvalidState<T> {
	pub fn new(state: T) -> Self {
		InvalidState { state }
	}
}