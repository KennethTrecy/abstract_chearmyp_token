/// An abstraction of block othertongue token.
pub trait AbstractBlockOthertongueToken {
	/// The type of block in the block othertongue token which could be a string, array of strings,
	/// or other type.
	type Block;

	/// Returns the block in the block othertongue token.
	fn block(&self) -> &Self::Block;

	/// Consumes the block othertongue token into block.
	fn consume(self) -> Self::Block;
}
