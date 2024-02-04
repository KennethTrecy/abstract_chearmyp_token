use crate::AbstractEdonToken;
use super::DynamicAbstractToken;

impl<T, U> AbstractEdonToken for U
where
	U: DynamicAbstractToken<Name = T> {
	type Edon = T;

	fn name(&self) -> &Self::Edon { DynamicAbstractToken::name(self) }

	fn consume(self) -> Self::Edon { DynamicAbstractToken::consume_concept(self) }
}
