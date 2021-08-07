use crate::FromToken;

/// An abstraction of attacher token.
pub trait AbstractAttacherToken : FromToken {
	/// The type of label in the attacher token.
	type Label;

	/// The type of content in the attacher token.
	type Content;

	/// Returns the label of the attacher.
	fn label(&self) -> Self::Label;

	/// Returns the content of the attacher.
	fn content(&self) -> Self::Content;
}
