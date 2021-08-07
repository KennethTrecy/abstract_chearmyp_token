use crate::TokenKind;

/// An abstraction of Chearmyp token.
pub trait AbstractToken {
	/// Returns the kind of the token it holds.
	fn kind(&self) -> TokenKind;
}
