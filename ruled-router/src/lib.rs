//! ruled-router - 基于宏的 Rust 路由解析库
//!
//! 这个库提供了一种声明式的方式来定义和解析 Web 路由，
//! 通过派生宏自动生成解析器和格式化器。

//! 模块定义
pub mod traits;
pub mod error;
pub mod parser;
pub mod formatter;
pub mod utils;
pub mod prelude;

// 重新导出核心类型
pub use error::{ParseError, ParseResult};
pub use traits::{Router, Query, FromParam, ToParam};
pub use parser::{PathParser, QueryParser};
pub use formatter::{PathFormatter, QueryFormatter, UrlFormatter};

// 重新导出派生宏（当启用 derive 特性时）
#[cfg(feature = "derive")]
pub use ruled_router_derive::{Router, Query};