use crate::AbstractToken;

/// An abstraction of scope level token.
pub trait AbstractScopeLevelToken {
	/// Returns the scope level it contains.
	fn level(&self) -> usize;

	/// Creates scope level token.
	fn from_token<T: AbstractToken>(_: T) -> Self;
}
