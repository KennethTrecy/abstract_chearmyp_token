use crate::abstracts::{AbstractBoundary, AbstractBoundaryCollection};
use crate::{TokenKind, AbstractToken};
use super::SimpleAbstractToken;

impl<T, U, V, W> AbstractToken<T, U, T, U, V> for W
where
	U: AbstractBoundary<T>,
	V: AbstractBoundaryCollection<T, U>,
	W: SimpleAbstractToken<T, U, V> {
	fn kind(&self) -> TokenKind { SimpleAbstractToken::kind(self) }

	fn new_line_comment(line: U) -> Self { Self::new_line_comment(line) }

	fn new_scope_level(scope_level: usize) -> Self { Self::new_scope_level(scope_level) }

	fn new_simplex(concept: U) -> Self { Self::new_simplex(concept) }

	fn new_line_othertongue(line: U) -> Self { Self::new_line_othertongue(line) }

	fn new_block_comment(block: V) -> Self { Self::new_block_comment(block) }

	fn new_complex(concept: U) -> Self { Self::new_complex(concept) }

	fn new_attacher(label: U, content: U) -> Self { Self::new_attacher(label, content) }

	fn new_block_othertongue(block: V) -> Self { Self::new_block_othertongue(block) }
}
