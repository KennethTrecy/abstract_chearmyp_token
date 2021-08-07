use crate::AbstractToken;

/// An abstraction of simplex token.
pub trait AbstractSimplexToken {
	/// The type that represents the simplex' name.
	type Simplex;

	/// Returns the name of the simplex token.
	fn name(&self) -> Self::Simplex;

	/// Creates simplex token.
	fn from_token<T: AbstractToken>(_: T) -> Self;
}
