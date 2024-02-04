#[cfg(feature = "no_std")]
use alloc::collections::VecDeque;

#[cfg(not(feature = "no_std"))]
use std::collections::VecDeque;

use crate::abstracts::{AbstractBoundary, AbstractBoundaryCollection};
use crate::{AbstractToken, AbstractTokenQueue};

/// This is only available if `vecdeque_token_queue` feature has been activated.
///
/// It implements [AbstractTokenQueue] for [VecDeque].
impl<T, U, V, W, X, Y> AbstractTokenQueue<T, U, V, W, X, Y> for VecDeque<Y>
where
	U: AbstractBoundary<T>,
	W: AbstractBoundary<V>,
	X: AbstractBoundaryCollection<V, W>,
	Y: AbstractToken<T, U, V, W, X> {
	fn push_token(&mut self, token: Y) {
		self.push_back(token)
	}

	fn shift_token(&mut self) -> Option<Y> {
		self.pop_front()
	}
}

#[cfg(test)]
mod t {
	use crate::native::{Range, Vec};
	use crate::{TokenKind, SimpleAbstractToken};
	use super::{AbstractTokenQueue, VecDeque};

	#[derive(Debug, PartialEq)]
	struct Token<'a> { _data: &'a str }
	impl<'a> Token<'a> {
		fn new() -> Self { Token { _data: "" } }
	}

	// Just a dummy token implementation
	impl<'a> SimpleAbstractToken<u8, Range<u8>, Vec<Range<u8>>> for Token<'a> {
		fn kind(&self) -> TokenKind { TokenKind::Edon }

		fn new_scope_level(_: usize) -> Self { Token::new() }

		fn new_edon(_: Range<u8>) -> Self { Token::new() }

		fn new_attacher(_: Range<u8>, _: Range<u8>) -> Self { Token::new() }

		fn new_line_comment(_: Range<u8>) -> Self { Token::new() }

		fn new_block_comment(_: Vec<Range<u8>>) -> Self { Token::new() }

		fn new_line_othertongue(_: Range<u8>) -> Self { Token::new() }

		fn new_othertongue(_: Vec<Range<u8>>) -> Self { Token::new() }
	}

	#[test]
	fn should_push_token() {
		let mut queue = VecDeque::<Token>::new();
		let mut expected_queue = VecDeque::new();
		expected_queue.push_back(Token::new_scope_level(0));

		queue.push_token(Token::new_scope_level(0));

		assert_eq!(queue, expected_queue);
	}

	#[test]
	fn should_pop_none() {
		let mut queue = VecDeque::<Token>::new();
		let expected_token = None;

		let token = queue.shift_token();

		assert_eq!(token, expected_token);
	}

	#[test]
	fn should_pop_some() {
		let token = Token::new();
		let mut queue = VecDeque::<Token>::new();
		queue.push_back(token);

		let token = queue.shift_token();

		assert_eq!(token, Some(Token::new()));
	}
}
