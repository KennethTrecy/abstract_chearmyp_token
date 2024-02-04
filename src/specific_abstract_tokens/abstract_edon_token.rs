/// An abstraction of edon token.
pub trait AbstractEdonToken {
	/// The type that represents the edon's name.
	type Edon;

	/// Returns the name of the edon token.
	fn name(&self) -> &Self::Edon;

	/// Consumes the edon token into concept.
	fn consume(self) -> Self::Edon;
}
