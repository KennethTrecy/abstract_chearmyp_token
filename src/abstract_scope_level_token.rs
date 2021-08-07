/// An abstraction of scope level token.
pub trait AbstractScopeLevelToken {
	/// Returns the scope level it contains.
	fn level(&self) -> usize;
}
