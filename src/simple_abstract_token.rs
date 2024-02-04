use crate::abstracts::{AbstractBoundary, AbstractBoundaryCollection};
use crate::TokenKind;

/// An simple abstraction of Chearmyp token.
///
/// Unlike [AbstractToken], [SimpleAbstractToken] has fewer template arguments. It assumes that the
/// innermost contents of [AbstractBoundaryCollection] will have the same type as the
/// [AbstractBoundary]. Implementing this trait automatically implements the [AbstractToken].
///
/// [AbstractToken]: crate::AbstractToken
pub trait SimpleAbstractToken<T, U, V>
where
	U: AbstractBoundary<T>,
	V: AbstractBoundaryCollection<T, U> {
	/// Returns the kind of the token it holds.
	fn kind(&self) -> TokenKind;

	/// Creates new line comment token.
	fn new_line_comment(_: U) -> Self;

	/// Creates new scope level token.
	fn new_scope_level(_: usize) -> Self;

	/// Creates new line othertongue token.
	fn new_line_othertongue(_: U) -> Self;

	/// Creates new block comment token.
	fn new_block_comment(_: V) -> Self;

	/// Creates new edon token.
	fn new_edon(_: U) -> Self;

	/// Creates new attacher token.
	///
	/// First parameter is the label, and the second parameter is the content.
	fn new_attacher(_: U, _: U) -> Self;

	/// Creates new othertongue token.
	fn new_othertongue(_: V) -> Self;
}

mod abstract_token;
