use crate::AbstractAttacherToken;
use super::DynamicAbstractToken;

impl<T, U, V> AbstractAttacherToken for V
where
	V: DynamicAbstractToken<Label = T, Content = U> {
	type Label = T;
	type Content = U;

	fn label(&self) -> &Self::Label { DynamicAbstractToken::label(self) }

	fn content(&self) -> &Self::Content { DynamicAbstractToken::content(self) }

	fn consume(self) -> (Self::Label, Self::Content) { DynamicAbstractToken::consume_attacher(self) }
}
