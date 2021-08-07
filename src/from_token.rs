use crate::AbstractToken;

/// Allows general Chearmyp token to converted into more specific token.
pub trait FromToken {
	/// Converts the general token into more specific token.
	fn from_token<T: AbstractToken>(_: T) -> Self;
}
