use crate::abstracts::{AbstractBoundary, AbstractBoundaryCollection};
use crate::AbstractToken;

/// An abstraction of Chearmyp token queue.
pub trait AbstractTokenQueue<T, U, V, W, X, Y>
where
	U: AbstractBoundary<T>,
	W: AbstractBoundary<V>,
	X: AbstractBoundaryCollection<V, W>,
	Y: AbstractToken<T, U, V, W, X> {
	/// Puts the token into the token queue.
	fn push_token(&mut self, _: Y);

	/// Takes the first token in the token queue.
	fn shift_token(&mut self) -> Option<Y>;
}

#[cfg(feature = "vecdeque_token_queue")]
mod vec_deque;
