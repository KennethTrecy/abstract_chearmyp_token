use crate::FromToken;

/// An abstraction of scope level token.
pub trait AbstractScopeLevelToken : FromToken {
	/// Returns the scope level it contains.
	fn level(&self) -> usize;
}
