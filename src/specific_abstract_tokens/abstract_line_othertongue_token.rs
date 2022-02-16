/// An abstraction of line othertongue token.
pub trait AbstractLineOthertongueToken {
	/// The line in line othertongue token which could be a string, array of bytes, or other type.
	type Line;

	/// Returns the line in line othertongue token.
	fn line(&self) -> &Self::Line;

	/// Consumes the line othertongue token into line.
	fn consume(self) -> Self::Line;
}
