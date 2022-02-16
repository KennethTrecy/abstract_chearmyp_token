use crate::AbstractComplexToken;
use super::DynamicAbstractToken;

impl<T, U> AbstractComplexToken for U
where
	U: DynamicAbstractToken<Name = T> {
	type Complex = T;

	fn name(&self) -> &Self::Complex { DynamicAbstractToken::name(self) }

	fn consume(self) -> Self::Complex { DynamicAbstractToken::consume_concept(self) }
}
