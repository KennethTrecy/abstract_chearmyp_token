/// An abstraction of block comment token.
pub trait AbstractBlockCommentToken {
	/// The type of block in the block comment token which could be a string, array of strings, or
	/// other type.
	type Block;

	/// Returns the block in the block comment token.
	fn block(&self) -> &Self::Block;

	/// Consumes the block comment token into block.
	fn consume(self) -> Self::Block;
}
