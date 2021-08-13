/// An abstraction of simplex token.
pub trait AbstractSimplexToken {
	/// The type that represents the simplex' name.
	type Simplex;

	/// Returns the name of the simplex token.
	fn name(&self) -> Self::Simplex;
}
