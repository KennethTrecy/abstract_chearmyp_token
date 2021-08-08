/// Possible kinds of Chearmyp tokens.
#[cfg_attr(feature = "assertable_token_kind", derive(Debug, PartialEq))]
pub enum TokenKind {
	Simplex,
	Complex,
	Attacher,
	ScopeLevel,
	BlockComment,
	LineComment,
	LineOthertongue,
	BlockOthertongue
}
