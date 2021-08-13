/// An abstraction of complex token.
pub trait AbstractComplexToken {
	/// The type that represents the complex' name.
	type Complex;

	/// Returns the name of the complex token.
	fn name(&self) -> Self::Complex;
}
