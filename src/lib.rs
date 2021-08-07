#![no_std]

//! # Chearmyp Abstract Token
//! Please read the README.md for more information.

mod abstracts {
	pub use chearmyp_abstract_source::AbstractSource;
}

mod token_kind;
mod abstract_token;
mod abstract_simplex_token;
mod abstract_scope_level_token;
mod abstract_line_comment_token;

pub use token_kind::TokenKind;
pub use abstract_token::AbstractToken;
pub use abstract_simplex_token::AbstractSimplexToken;
pub use abstract_scope_level_token::AbstractScopeLevelToken;
pub use abstract_line_comment_token::AbstractLineCommentToken;
