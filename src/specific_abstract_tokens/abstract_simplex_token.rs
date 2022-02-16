/// An abstraction of simplex token.
pub trait AbstractSimplexToken {
	/// The type that represents the simplex' name.
	type Simplex;

	/// Returns the name of the simplex token.
	fn name(&self) -> &Self::Simplex;

	/// Consumes the simplex token into concept.
	fn consume(self) -> Self::Simplex;
}
