//! Prelude module for convenient imports
//!
//! This module re-exports the most commonly used items from the crate.

pub use crate::error::ParseError;
pub use crate::formatter::{PathFormatter, QueryFormatter};
pub use crate::parser::{PathParser, QueryParser};
pub use crate::traits::{FromParam, Query, RouteMatcher, Router, ToParam};
pub use crate::utils::*;

#[cfg(feature = "derive")]
pub use ruled_router_derive::{Query, Router};

// DOM 功能导出（只有在启用 dom feature 时才导出）
#[cfg(feature = "dom")]
pub use crate::dom::{helpers, DomRouter};
