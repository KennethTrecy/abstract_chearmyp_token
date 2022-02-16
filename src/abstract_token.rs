use crate::abstracts::{AbstractBoundary, AbstractBoundaryCollection};
use crate::TokenKind;

/// An abstraction of Chearmyp token.
pub trait AbstractToken<T, U, V, W, X>
where
	U: AbstractBoundary<T>,
	W: AbstractBoundary<V>,
	X: AbstractBoundaryCollection<V, W> {
	/// Returns the kind of the token it holds.
	fn kind(&self) -> TokenKind;

	/// Creates new line comment token.
	fn new_line_comment(_: U) -> Self;

	/// Creates new scope level token.
	fn new_scope_level(_: usize) -> Self;

	/// Creates new simplex token.
	fn new_simplex(_: U) -> Self;

	/// Creates new line othertongue token.
	fn new_line_othertongue(_: U) -> Self;

	/// Creates new block comment token.
	fn new_block_comment(_: X) -> Self;

	/// Creates new complex token.
	fn new_complex(_: U) -> Self;

	/// Creates new attacher token.
	///
	/// First parameter is the label, and the second parameter is the content.
	fn new_attacher(_: U, _: U) -> Self;

	/// Creates new block othertongue token.
	fn new_block_othertongue(_: X) -> Self;
}
