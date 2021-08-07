use crate::abstracts::AbstractSource;
use crate::TokenKind;

/// An abstraction of Chearmyp token.
pub trait AbstractToken {
	/// The type of source where all tokens will be built from.
	type Source: AbstractSource;

	/// Returns the kind of the token it holds.
	fn kind(&self) -> TokenKind;

	/// Creates new line comment token.
	fn new_line_comment(_: Self::Source) -> Self;

	/// Creates new scope level token.
	fn new_scope_level(_: usize) -> Self;

	/// Creates new simplex token.
	fn new_simplex(_: usize) -> Self;

	/// Creates new line othertongue token.
	fn new_line_othertongue(_: Self::Source) -> Self;
}
