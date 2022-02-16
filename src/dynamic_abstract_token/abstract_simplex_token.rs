use crate::AbstractSimplexToken;
use super::DynamicAbstractToken;

impl<T, U> AbstractSimplexToken for U
where
	U: DynamicAbstractToken<Name = T> {
	type Simplex = T;

	fn name(&self) -> &Self::Simplex { DynamicAbstractToken::name(self) }

	fn consume(self) -> Self::Simplex { DynamicAbstractToken::consume_concept(self) }
}
