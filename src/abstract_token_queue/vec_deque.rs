#[cfg(feature = "no_std")]
use alloc::collections::VecDeque;

#[cfg(not(feature = "no_std"))]
use std::collections::VecDeque;

use crate::{ AbstractToken, AbstractTokenQueue };

impl<T> AbstractTokenQueue for VecDeque<T>
where
	T: AbstractToken {
	type Token = T;

	fn new() -> Self { VecDeque::new() }

	fn push_token(&mut self, token: Self::Token) {
		self.push_back(token)
	}

	fn shift_token(&mut self) -> Option<Self::Token> {
		self.pop_front()
	}
}

#[cfg(test)]
mod t {
	use crate::TokenKind;
	use super::{ AbstractToken, AbstractTokenQueue, VecDeque };

	#[cfg(feature = "no_std")]
	use alloc::vec::Vec;

	#[derive(Debug, PartialEq)]
	struct Token<'a> { _data: &'a str }
	impl<'a> Token<'a> {
		fn new() -> Self { Token { _data: "" } }
	}

	// Just a dummy token implementation
	impl<'a> AbstractToken for Token<'a> {
		type Source = &'a str;
		type SourceCollection = Vec<&'a str>;

		fn kind(&self) -> TokenKind { TokenKind::Simplex }

		fn new_scope_level(_: usize) -> Self { Token::new() }

		fn new_complex(_: Self::Source) -> Self { Token::new() }

		fn new_simplex(_: Self::Source) -> Self { Token::new() }

		fn new_attacher(_: Self::Source, _: Self::Source) -> Self { Token::new() }

		fn new_line_comment(_: Self::Source) -> Self { Token::new() }

		fn new_block_comment(_: Self::SourceCollection) -> Self { Token::new() }

		fn new_line_othertongue(_: Self::Source) -> Self { Token::new() }

		fn new_block_othertongue(_: Self::SourceCollection) -> Self { Token::new() }
	}

	#[test]
	fn should_push_token() {
		let mut queue = <VecDeque<Token> as AbstractTokenQueue>::new();
		let mut expected_queue = VecDeque::new();
		expected_queue.push_back(Token::new_scope_level(0));

		queue.push_token(Token::new_scope_level(0));

		assert_eq!(queue, expected_queue);
	}

	#[test]
	fn should_pop_none() {
		let mut queue = <VecDeque<Token> as AbstractTokenQueue>::new();
		let expected_token = None;

		let token = queue.shift_token();

		assert_eq!(token, expected_token);
	}

	#[test]
	fn should_pop_some() {
		let token = Token::new();
		let mut queue = <VecDeque<Token> as AbstractTokenQueue>::new();
		queue.push_back(token);

		let token = queue.shift_token();

		assert_eq!(token, Some(Token::new()));
	}
}
