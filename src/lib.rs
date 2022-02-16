#![cfg_attr(feature = "no_std", no_std)]

//! # Abstract Chearmyp Token
//! Please read the README.md for more information.
//!
//! ## Features available
//! - `no_std`: Uses the `core` crate instead of `std` crate.
#![cfg_attr(
	feature = "no_std",
	doc = "- `vecdeque_token_queue`: Implements [AbstractTokenQueue]
												for [alloc::collections::VecDeque].",
)]
#![cfg_attr(
	not(feature = "no_std"),
	doc = "- `vecdeque_token_queue`: Implements [AbstractTokenQueue]
												for [std::collections::VecDeque].",
)]
//! - `assertable_token_kind`: Allows token kind to be tested.

#[cfg(feature = "no_std")]
extern crate alloc;

mod abstracts {
	pub use abstract_chearmyp_boundary::{AbstractBoundary, AbstractBoundaryCollection};
}

#[cfg(test)]
mod native {
	#[cfg(feature = "no_std")]
	pub use core::ops::Range;

	#[cfg(feature = "no_std")]
	pub use alloc::vec::Vec;

	#[cfg(not(feature = "no_std"))]
	pub use std::{
		vec::Vec,
		ops::Range
	};
}

mod token_kind;
mod abstract_token;
mod abstract_token_queue;
mod simple_abstract_token;
mod dynamic_abstract_token;
mod specific_abstract_tokens;

pub use token_kind::TokenKind;
pub use abstract_token::AbstractToken;
pub use abstract_token_queue::AbstractTokenQueue;
pub use simple_abstract_token::SimpleAbstractToken;
pub use dynamic_abstract_token::DynamicAbstractToken;
pub use specific_abstract_tokens::{
	AbstractComplexToken,
	AbstractSimplexToken,
	AbstractAttacherToken,
	AbstractScopeLevelToken,
	AbstractLineCommentToken,
	AbstractBlockCommentToken,
	AbstractLineOthertongueToken,
	AbstractBlockOthertongueToken
};
