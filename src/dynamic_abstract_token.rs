/// An abstraction of dynamic Chearmyp token.
///
/// Assumes that the token can represent any specific token. The methods can panic if the they
/// trying to do an invalid operation. For example, getting the label from a simplex token must
/// panic. Getting the label can only be done for an attacher token.
///
/// Automatically implements [AbstractComplexToken], [AbstractSimplexToken],
/// [AbstractAttacherToken], [AbstractScopeLevelToken], [AbstractLineCommentToken],
/// [AbstractBlockCommentToken], [AbstractLineOthertongueToken], and
/// [AbstractBlockOthertongueToken].
///
/// [AbstractComplexToken]: crate::AbstractComplexToken
/// [AbstractSimplexToken]: crate::AbstractSimplexToken
/// [AbstractAttacherToken]: crate::AbstractAttacherToken
/// [AbstractScopeLevelToken]: crate::AbstractScopeLevelToken
/// [AbstractLineCommentToken]: crate::AbstractLineCommentToken
/// [AbstractBlockCommentToken]: crate::AbstractBlockCommentToken
/// [AbstractLineOthertongueToken]: crate::AbstractLineOthertongueToken
/// [AbstractBlockOthertongueToken]: crate::AbstractBlockOthertongueToken
pub trait DynamicAbstractToken {
	/// The type that represents the complex' name or simplex' name.
	type Name;

	/// The type of line in the line comment token or line othertongue token.
	type Line;

	/// The type of block in the block comment token or block othertongue token.
	type Block;

	/// The type of label in the attacher token.
	type Label;

	/// The type of content in the attacher token.
	type Content;

	/// Returns the name of the simplex token or complex token.
	fn name(&self) -> &Self::Name;

	/// Returns the line in line comment token or line othertongue token.
	fn line(&self) -> &Self::Line;

	/// Returns the block in the block comment token or block othertongue token.
	fn block(&self) -> &Self::Block;

	/// Returns the scope level of scope level token.
	fn level(&self) -> usize;

	/// Returns the label of the attacher token.
	fn label(&self) -> &Self::Label;

	/// Returns the content of the attacher token.
	fn content(&self) -> &Self::Content;

	/// Consumes the attacher token into tuple.
	fn consume_attacher(self) -> (Self::Label, Self::Content);

	/// Consumes the block comment or othertongue token into block.
	fn consume_block(self) -> Self::Block;

	/// Consumes the simplex or complex token into concept.
	fn consume_concept(self) -> Self::Name;

	/// Consumes the line comment or othertongue token into line.
	fn consume_line(self) -> Self::Line;
}

mod abstract_complex_token;
mod abstract_simplex_token;
mod abstract_attacher_token;
mod abstract_scope_level_token;
mod abstract_line_comment_token;
mod abstract_block_comment_token;
mod abstract_line_othertongue_token;
mod abstract_block_othertongue_token;
