use crate::AbstractToken;

/// Allows general Chearmyp token to converted into specific token.
pub trait FromToken {
	/// Converts the general token into specific token.
	fn from_token<T: AbstractToken>(_: T) -> Self;
}
