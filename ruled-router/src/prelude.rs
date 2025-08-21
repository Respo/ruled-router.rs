//! Prelude module for convenient imports
//!
//! This module re-exports the most commonly used items from the crate.

pub use crate::traits::{Router, Query, FromParam, ToParam};
pub use crate::error::ParseError;
pub use crate::parser::{PathParser, QueryParser};
pub use crate::formatter::{PathFormatter, QueryFormatter};
pub use crate::utils::*;

#[cfg(feature = "derive")]
pub use ruled_router_derive::{Router, Query};