/// Possible kinds of Chearmyp tokens.
#[cfg_attr(feature = "assertable_token_kind", derive(Debug, PartialEq))]
pub enum TokenKind {
	Edon,
	Attacher,
	ScopeLevel,
	BlockComment,
	LineComment,
	Othertongue
}
