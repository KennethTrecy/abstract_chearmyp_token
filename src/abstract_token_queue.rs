use crate::AbstractToken;

/// An abstraction of Chearmyp token queue.
pub trait AbstractTokenQueue {
	/// The type of token that the queue contains.
	type Token: AbstractToken;

	/// Creates an empty token queue.
	fn new() -> Self;

	/// Puts the token into the token queue.
	fn push_token(&mut self, _: Self::Token);

	/// Takes the first token in the token queue.
	fn shift_token(&mut self) -> Option<Self::Token>;
}

#[cfg(feature = "vecdeque_token_queue")]
mod vec_deque;
