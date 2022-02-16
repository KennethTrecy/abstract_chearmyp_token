use crate::AbstractLineCommentToken;
use super::DynamicAbstractToken;

impl<T, U> AbstractLineCommentToken for U
where
	U: DynamicAbstractToken<Line = T> {
	type Line = T;

	fn line(&self) -> &Self::Line { DynamicAbstractToken::line(self) }

	fn consume(self) -> Self::Line { DynamicAbstractToken::consume_line(self) }
}
