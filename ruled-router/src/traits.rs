//! 核心 trait 定义
//!
//! 定义了路由解析和格式化的核心接口

use crate::error::ParseError;

/// 路由解析和格式化的核心 trait
/// 
/// 实现此 trait 的类型可以从 URL 路径字符串解析，也可以格式化为路径字符串
pub trait Router: Sized {
    /// 从路径字符串解析路由
    /// 
    /// # 参数
    /// 
    /// * `path` - 要解析的路径字符串，可能包含查询参数
    /// 
    /// # 返回值
    /// 
    /// 成功时返回解析后的路由对象，失败时返回 ParseError
    /// 
    /// # 示例
    /// 
    /// ```rust
    /// use ruled_router::Router;
    /// 
    /// let route = MyRoute::parse("/user/123?tab=profile")?;
    /// ```
    fn parse(path: &str) -> Result<Self, ParseError>;
    
    /// 将路由格式化为路径字符串
    /// 
    /// # 返回值
    /// 
    /// 格式化后的完整 URL 路径，包括查询参数（如果有）
    /// 
    /// # 示例
    /// 
    /// ```rust
    /// let url = route.format();
    /// assert_eq!(url, "/user/123?tab=profile");
    /// ```
    fn format(&self) -> String;
    
    /// 获取路由模式（用于调试和文档生成）
    /// 
    /// # 返回值
    /// 
    /// 路由的模式字符串，例如 "/user/:id"
    fn pattern() -> &'static str;
}

/// 查询参数解析和格式化的 trait
/// 
/// 实现此 trait 的类型可以从查询字符串解析，也可以格式化为查询字符串
pub trait Query: Sized {
    /// 从查询字符串解析参数
    /// 
    /// # 参数
    /// 
    /// * `query` - 查询字符串，不包含前导的 '?'
    /// 
    /// # 返回值
    /// 
    /// 成功时返回解析后的查询参数对象，失败时返回 ParseError
    /// 
    /// # 示例
    /// 
    /// ```rust
    /// use ruled_router::Query;
    /// 
    /// let params = SearchParams::parse("q=rust&page=2")?;
    /// ```
    fn parse(query: &str) -> Result<Self, ParseError>;
    
    /// 将参数格式化为查询字符串
    /// 
    /// # 返回值
    /// 
    /// 格式化后的查询字符串，不包含前导的 '?'
    /// 
    /// # 示例
    /// 
    /// ```rust
    /// let query_string = params.format();
    /// assert_eq!(query_string, "q=rust&page=2");
    /// ```
    fn format(&self) -> String;
}

/// 类型转换 trait，用于路径参数的类型转换
/// 
/// 实现此 trait 的类型可以从字符串参数转换而来
pub trait FromParam: Sized {
    /// 从字符串参数转换为目标类型
    /// 
    /// # 参数
    /// 
    /// * `param` - 路径参数的字符串值
    /// 
    /// # 返回值
    /// 
    /// 成功时返回转换后的值，失败时返回 ParseError
    /// 
    /// # 示例
    /// 
    /// ```rust
    /// use ruled_router::FromParam;
    /// 
    /// let id: u32 = u32::from_param("123")?;
    /// assert_eq!(id, 123);
    /// ```
    fn from_param(param: &str) -> Result<Self, ParseError>;
}

/// 类型格式化 trait，用于将参数转换为字符串
/// 
/// 实现此 trait 的类型可以转换为字符串用于 URL 路径
pub trait ToParam {
    /// 将值转换为字符串参数
    /// 
    /// # 返回值
    /// 
    /// 值的字符串表示，用于 URL 路径
    /// 
    /// # 示例
    /// 
    /// ```rust
    /// use ruled_router::ToParam;
    /// 
    /// let id = 123u32;
    /// assert_eq!(id.to_param(), "123");
    /// ```
    fn to_param(&self) -> String;
}