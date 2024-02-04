use crate::AbstractOthertongueToken;
use super::DynamicAbstractToken;

impl<T, U> AbstractOthertongueToken for U
where
	U: DynamicAbstractToken<Block = T> {
	type Block = T;

	fn block(&self) -> &Self::Block { DynamicAbstractToken::block(self) }

	fn consume(self) -> Self::Block { DynamicAbstractToken::consume_block(self) }
}
