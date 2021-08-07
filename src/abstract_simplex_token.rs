use crate::FromToken;

/// An abstraction of simplex token.
pub trait AbstractSimplexToken : FromToken {
	/// The type that represents the simplex' name.
	type Simplex;

	/// Returns the name of the simplex token.
	fn name(&self) -> Self::Simplex;
}
