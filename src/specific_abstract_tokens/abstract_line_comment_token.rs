/// An abstraction of line comment token.
pub trait AbstractLineCommentToken {
	/// The line in line comment token which could be a string, array of bytes, or other type.
	type Line;

	/// Returns the line in line comment token.
	fn line(&self) -> &Self::Line;

	/// Consumes the line comment token into line.
	fn consume(self) -> Self::Line;
}
