#![cfg_attr(feature = "no_std", no_std)]

//! # Chearmyp Abstract Token
//! Please read the README.md for more information.
//!
//! ## Features available
//! - `no_std`: Uses the `core` crate instead of `std` crate.

#[cfg(feature = "no_std")]
extern crate alloc;

mod abstracts {
	pub use chearmyp_abstract_source::{AbstractSource, AbstractSourceCollection};
}

mod from_token;
mod token_kind;
mod abstract_token;
mod abstract_token_queue;
mod abstract_complex_token;
mod abstract_simplex_token;
mod abstract_attacher_token;
mod abstract_scope_level_token;
mod abstract_line_comment_token;
mod abstract_block_comment_token;
mod abstract_line_othertongue_token;
mod abstract_block_othertongue_token;

pub use from_token::FromToken;
pub use token_kind::TokenKind;
pub use abstract_token::AbstractToken;
pub use abstract_token_queue::AbstractTokenQueue;
pub use abstract_complex_token::AbstractComplexToken;
pub use abstract_simplex_token::AbstractSimplexToken;
pub use abstract_attacher_token::AbstractAttacherToken;
pub use abstract_scope_level_token::AbstractScopeLevelToken;
pub use abstract_line_comment_token::AbstractLineCommentToken;
pub use abstract_block_comment_token::AbstractBlockCommentToken;
pub use abstract_line_othertongue_token::AbstractLineOthertongueToken;
pub use abstract_block_othertongue_token::AbstractBlockOthertongueToken;
