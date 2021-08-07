use crate::abstracts::AbstractSource;
use crate::TokenKind;

/// An abstraction of Chearmyp token.
pub trait AbstractToken {
	type Source: AbstractSource;

	/// Returns the kind of the token it holds.
	fn kind(&self) -> TokenKind;

	/// Creates new line comment token.
	fn new_line_comment(_: Self::Source) -> Self;

	/// Creates new scope level token.
	fn new_scope_level(_: usize) -> Self;
}
