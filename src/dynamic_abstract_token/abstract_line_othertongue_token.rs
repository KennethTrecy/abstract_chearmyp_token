use crate::AbstractLineOthertongueToken;
use super::DynamicAbstractToken;

impl<T, U> AbstractLineOthertongueToken for U
where
	U: DynamicAbstractToken<Line = T> {
	type Line = T;

	fn line(&self) -> &Self::Line { DynamicAbstractToken::line(self) }

	fn consume(self) -> Self::Line { DynamicAbstractToken::consume_line(self) }
}
