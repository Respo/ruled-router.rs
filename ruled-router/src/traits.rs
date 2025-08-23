//! 核心 trait 定义
//!
//! 定义了路由解析和格式化的核心接口

use crate::error::ParseError;

/// 路由匹配选择的 trait
///
/// 实现此 trait 的枚举类型表示从多个可能的路由中选择一个匹配的路由。
/// 这是嵌套路由系统的核心，支持 RouteMatcher > Router > RouteMatcher > Router 的结构。
pub trait RouteMatcher: Sized {
  /// 尝试从路径解析出匹配的路由
  ///
  /// # 参数
  ///
  /// * `path` - 要解析的路径字符串
  ///
  /// # 返回值
  ///
  /// 成功时返回匹配的路由枚举变体，失败时返回 ParseError
  ///
  /// # 示例
  ///
  /// ```rust,ignore
  /// use ruled_router::RouterMatch;
  ///
  /// let route_match = AppRouterMatch::try_parse("/api/v1/users/123")?;
  /// ```
  fn try_parse(path: &str) -> Result<Self, ParseError>;

  /// 将路由匹配格式化为路径字符串
  ///
  /// # 返回值
  ///
  /// 格式化后的完整 URL 路径
  ///
  /// # 示例
  ///
  /// ```rust,ignore
  /// let url = route_match.format();
  /// assert_eq!(url, "/api/v1/users/123");
  /// ```
  fn format(&self) -> String;

  /// 获取所有可能的路由模式
  ///
  /// # 返回值
  ///
  /// 所有可能的路由模式列表，用于调试和文档生成
  ///
  /// # 示例
  ///
  /// ```rust,ignore
  /// let patterns = AppRouterMatch::patterns();
  /// assert!(patterns.contains(&"/api/:version/users/:id"));
  /// ```
  fn patterns() -> Vec<&'static str>;

  /// 尝试解析路径的剩余部分（用于嵌套路由）
  ///
  /// # 参数
  ///
  /// * `path` - 完整路径
  /// * `consumed_length` - 已消费的路径长度
  ///
  /// # 返回值
  ///
  /// 成功时返回匹配的路由和剩余路径，失败时返回 ParseError
  fn try_parse_with_remaining(path: &str, consumed_length: usize) -> Result<(Self, &str), ParseError> {
    let route = Self::try_parse(path)?;
    let remaining = if consumed_length < path.len() {
      &path[consumed_length..]
    } else {
      ""
    };
    Ok((route, remaining))
  }
}

/// 空的路由匹配类型，用于没有子路由的情况
#[derive(Debug, Clone, PartialEq)]
pub struct NoSubRouter;

impl RouteMatcher for NoSubRouter {
  fn try_parse(_path: &str) -> Result<Self, ParseError> {
    Err(ParseError::invalid_path("No sub-router available"))
  }

  fn format(&self) -> String {
    String::new()
  }

  fn patterns() -> Vec<&'static str> {
    vec![]
  }
}

/// 路由解析和格式化的核心 trait
///
/// 实现此 trait 的类型可以从 URL 路径字符串解析，也可以格式化为路径字符串。
/// 支持嵌套路由结构，通过 SubRouterMatch 关联类型定义子路由。
pub trait Router: Sized {
  /// 子路由匹配类型
  ///
  /// 如果路由支持子路由，则定义为具体的 RouterMatch 类型；
  /// 如果不支持子路由，则使用 NoSubRouter 类型。
  type SubRouterMatch: RouteMatcher;
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
  /// ```rust,ignore
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
  /// ```rust,ignore
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

  /// 解析路径并返回路由和可能的子路由
  ///
  /// # 参数
  ///
  /// * `path` - 要解析的路径字符串
  ///
  /// # 返回值
  ///
  /// 成功时返回路由实例和可能的子路由匹配，失败时返回 ParseError
  ///
  /// # 示例
  ///
  /// ```rust,ignore
  /// let (route, sub_match) = UserRoute::parse_with_sub("/users/123/posts/456")?;
  /// ```
  fn parse_with_sub(path: &str) -> Result<(Self, Option<Self::SubRouterMatch>), ParseError> {
    // 默认实现：只解析当前路由，不处理子路由
    let route = Self::parse(path)?;
    Ok((route, None))
  }

  /// 格式化路由和子路由为完整路径
  ///
  /// # 参数
  ///
  /// * `sub_route` - 可选的子路由匹配
  ///
  /// # 返回值
  ///
  /// 格式化后的完整路径字符串
  ///
  /// # 示例
  ///
  /// ```rust,ignore
  /// let url = route.format_with_sub(Some(&sub_match));
  /// ```
  fn format_with_sub(&self, sub_route: Option<&Self::SubRouterMatch>) -> String {
    let base_url = self.format();
    match sub_route {
      Some(sub) => {
        let sub_url = sub.format();
        if sub_url.is_empty() {
          base_url
        } else {
          format!("{}{}", base_url.trim_end_matches('/'), sub_url)
        }
      }
      None => base_url,
    }
  }

  /// 获取路径消费的长度（用于嵌套路由解析）
  ///
  /// # 参数
  ///
  /// * `path` - 原始路径
  ///
  /// # 返回值
  ///
  /// 当前路由消费的路径长度
  fn consumed_length(path: &str) -> Result<usize, ParseError> {
    // 默认实现：尝试解析并计算消费的长度
    let _route = Self::parse(path)?;
    // 这里需要具体的实现来计算实际消费的长度
    // 暂时返回整个路径的长度
    Ok(path.len())
  }
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
  /// ```rust,ignore
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
  /// ```rust,ignore
  /// let query_string = params.format();
  /// assert_eq!(query_string, "q=rust&page=2");
  /// ```
  fn format(&self) -> String;

  /// 从查询参数映射解析参数（用于 Router 派生宏）
  ///
  /// # 参数
  ///
  /// * `query_map` - 查询参数的键值对映射
  ///
  /// # 返回值
  ///
  /// 成功时返回解析后的查询参数对象，失败时返回 ParseError
  fn from_query_map(query_map: &std::collections::HashMap<String, Vec<String>>) -> Result<Self, ParseError>;

  /// 将参数格式化为查询字符串（用于 Router 派生宏）
  ///
  /// # 返回值
  ///
  /// 格式化后的查询字符串，不包含前导的 '?'
  fn to_query_string(&self) -> String;
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
  /// ```rust,ignore
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
  /// ```rust,ignore
  /// use ruled_router::ToParam;
  ///
  /// let id = 123u32;
  /// assert_eq!(id.to_param(), "123");
  /// ```
  fn to_param(&self) -> String;
}
