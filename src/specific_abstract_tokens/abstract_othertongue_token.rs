/// An abstraction of othertongue token.
pub trait AbstractOthertongueToken {
	/// The type of block in the othertongue token which could be a string, array of strings,
	/// or other type.
	type Block;

	/// Returns the block in the block othertongue token.
	fn block(&self) -> &Self::Block;

	/// Consumes the block othertongue token into block.
	fn consume(self) -> Self::Block;
}
