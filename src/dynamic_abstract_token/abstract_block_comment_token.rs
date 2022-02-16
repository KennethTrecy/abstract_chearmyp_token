use crate::AbstractBlockCommentToken;
use super::DynamicAbstractToken;

impl<T, U> AbstractBlockCommentToken for U
where
	U: DynamicAbstractToken<Block = T> {
	type Block = T;

	fn block(&self) -> &Self::Block { DynamicAbstractToken::block(self) }

	fn consume(self) -> Self::Block { DynamicAbstractToken::consume_block(self) }
}
