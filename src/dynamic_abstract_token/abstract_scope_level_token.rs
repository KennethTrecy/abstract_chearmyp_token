use crate::AbstractScopeLevelToken;
use super::DynamicAbstractToken;

impl<T> AbstractScopeLevelToken for T
where
	T: DynamicAbstractToken {
	fn level(&self) -> usize { DynamicAbstractToken::level(self) }
}
