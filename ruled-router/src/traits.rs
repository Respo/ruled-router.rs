//! 核心 trait 定义
//!
//! 定义了路由解析和格式化的核心接口

use crate::error::{ParseError, RouteState};
use std::fmt::Debug;

/// 嵌套路由解析结果
///
/// 包含当前路由和递归解析的子路由信息
#[derive(Debug, Clone)]
pub struct NestedRouteResult<T> {
  /// 当前层级的路由
  pub current: T,
  /// 子路由信息（如果存在）
  pub sub_route_info: Option<Box<RouteInfo>>,
}

/// 路由信息的通用表示
///
/// 用于表示任意层级的路由信息，支持递归嵌套
#[derive(Debug, Clone)]
pub struct RouteInfo {
  /// 路由的模式字符串
  pub pattern: &'static str,
  /// 路由的格式化字符串
  pub formatted: String,
  /// 子路由信息（如果存在）
  pub sub_route_info: Option<Box<RouteInfo>>,
}

/// 将路由匹配器转换为路由信息的 trait
pub trait ToRouteInfo {
  /// 将当前路由匹配器转换为路由信息
  fn to_route_info(&self) -> RouteInfo;
}

/// 路由匹配选择的 trait
///
/// **这是顶层路由的推荐实现方式。** 实现此 trait 的枚举类型表示从多个可能的路由中选择一个匹配的路由。
/// 这是嵌套路由系统的核心，支持 RouteMatcher > RouterData > RouteMatcher > RouterData 的结构。
///
/// # 用法
///
/// - 使用 `#[derive(RouterMatch)]` 为包含多个 RouterData 的 enum 实现此 trait
/// - 通常用作应用程序的根路由器，管理所有顶层路由
pub trait RouteMatcher: Sized + ToRouteInfo {
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

  /// 格式化路由结构为调试用的嵌套树形结构
  ///
  /// 此方法用于开发过程中验证路由结构，以树形格式显示嵌套的路由层次。
  /// 对于复杂的嵌套路由结构，这个方法可以帮助开发者直观地理解路由的组织方式。
  ///
  /// # 参数
  ///
  /// * `indent` - 缩进级别，用于控制嵌套显示的深度
  ///
  /// # 返回值
  ///
  /// 格式化后的树形结构字符串
  ///
  /// # 示例
  ///
  /// ```rust,ignore
  /// let route_match = AppRouterMatch::try_parse("/users/123/profile/basic")?;
  /// println!("{}", route_match.debug_format(0));
  /// // 输出类似：
  /// // AppRouterMatch::User
  /// //   ├─ UserRoute { id: 123 }
  /// //   └─ UserSubRouterMatch::Profile
  /// //       ├─ ProfileRoute
  /// //       └─ ProfileDetailRouterMatch::Basic
  /// //           └─ BasicRoute
  /// ```
  fn debug_format(&self, indent: usize) -> String {
    // 默认实现：显示基本的路由信息
    let indent_str = "  ".repeat(indent);
    let route_info = self.to_route_info();
    format!(
      "{}RouterMatch\n{}├─ Pattern: {}\n{}└─ Formatted: {}",
      indent_str, indent_str, route_info.pattern, indent_str, route_info.formatted
    )
  }
}

/// 空的路由匹配类型，用于没有子路由的情况
#[derive(Debug, Clone, PartialEq)]
pub struct NoSubRouter;

impl RouteMatcher for NoSubRouter {
  fn try_parse(_path: &str) -> Result<Self, ParseError> {
    Err(ParseError::invalid_path("No sub router available"))
  }

  fn format(&self) -> String {
    String::new()
  }

  fn patterns() -> Vec<&'static str> {
    vec![]
  }

  fn debug_format(&self, indent: usize) -> String {
    let indent_str = "  ".repeat(indent);
    format!("{indent_str}NoSubRouter (empty)")
  }
}

impl ToRouteInfo for NoSubRouter {
  fn to_route_info(&self) -> RouteInfo {
    RouteInfo {
      pattern: "",
      formatted: String::new(),
      sub_route_info: None,
    }
  }
}

/// 路由解析和格式化的核心 trait
///
/// **注意**: `RouterData` 用于定义单个路由类型，不能直接用作顶层路由。
/// 顶层路由应该使用实现了 `RouteMatcher` trait 的 enum 结构。
///
/// 实现此 trait 的类型可以从 URL 路径字符串解析，也可以格式化为路径字符串。
/// 支持嵌套路由结构，通过 SubRouterMatch 关联类型定义子路由。
///
/// # 用法
///
/// - 使用 `#[derive(RouterData)]` 为单个路由结构体实现此 trait
/// - 顶层路由使用 `#[derive(RouterMatch)]` 的 enum 来组合多个 RouterData
pub trait RouterData: Sized {
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
  /// use ruled_router::RouterData;
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

  /// 将路由和可能的子路由格式化为完整路径字符串
  ///
  /// 此方法专门用于处理包含子路由的 RouterData 结构。
  /// 由于 RouterData 不作为顶级路由使用，此方法更适合于子路由的格式化。
  ///
  /// # 返回值
  ///
  /// 格式化后的完整 URL 路径，包括子路由和查询参数（如果有）
  ///
  /// # 示例
  ///
  /// ```rust,ignore
  /// let url = route.format_sub_router();
  /// assert_eq!(url, "/user/123/profile?tab=advanced");
  /// ```
  fn format_sub_router(&self) -> String {
    // 默认实现：调用基本的 format 方法
    self.format()
  }

  /// 获取路由模式（用于调试和文档生成）
  ///
  /// # 返回值
  ///
  /// 路由的模式字符串，例如 "/user/:id"
  fn pattern() -> &'static str;

  /// 解析路径并返回路由和子路由状态
  ///
  /// # 参数
  ///
  /// * `path` - 要解析的路径字符串
  ///
  /// # 返回值
  ///
  /// 成功时返回路由实例和子路由状态，失败时返回 ParseError
  /// 子路由状态可以是：
  /// - `RouteState::NoSubRoute`: 没有子路由（预期的叶子节点）
  /// - `RouteState::SubRoute(sub)`: 成功解析到子路由
  /// - `RouteState::ParseFailed { ... }`: 子路由解析失败，包含调试信息
  ///
  /// # 示例
  ///
  /// ```rust,ignore
  /// let (route, sub_state) = UserRoute::parse_with_sub("/users/123/posts/456")?;
  /// match sub_state {
  ///   RouteState::NoSubRoute => println!("叶子节点"),
  ///   RouteState::SubRoute(sub) => println!("有子路由: {:?}", sub),
  ///   RouteState::ParseFailed { remaining_path, .. } => {
  ///     println!("子路由解析失败，剩余路径: {}", remaining_path);
  ///   }
  /// }
  /// ```
  fn parse_with_sub(path: &str) -> Result<(Self, RouteState<Self::SubRouterMatch>), ParseError> {
    // 默认实现：只解析当前路由，不处理子路由
    let route = Self::parse(path)?;
    Ok((route, RouteState::NoSubRoute))
  }

  /// 格式化路由和子路由为完整路径
  ///
  /// # 参数
  ///
  /// * `sub_route_state` - 子路由状态
  ///
  /// # 返回值
  ///
  /// 格式化后的完整路径字符串
  ///
  /// # 示例
  ///
  /// ```rust,ignore
  /// let url = route.format_with_sub(&sub_state);
  /// ```
  fn format_with_sub(&self, sub_route_state: &RouteState<Self::SubRouterMatch>) -> String {
    let base_url = self.format();
    match sub_route_state {
      RouteState::SubRoute(sub) => {
        let sub_url = sub.format();
        if sub_url.is_empty() {
          base_url
        } else {
          format!("{}{}", base_url.trim_end_matches('/'), sub_url)
        }
      }
      RouteState::NoSubRoute | RouteState::ParseFailed { .. } => base_url,
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

  /// 递归解析嵌套路由（自动化版本）
  ///
  /// 这个方法会自动递归解析所有层级的嵌套路由，返回一个包含完整路由信息的结构。
  /// 相比手动解析，这个方法牺牲了一些灵活性，但提供了更简洁的使用方式。
  ///
  /// # 参数
  ///
  /// * `path` - 要解析的完整路径字符串
  ///
  /// # 返回值
  ///
  /// 成功时返回 `NestedRouteResult`，包含当前路由和递归解析的子路由信息
  ///
  /// # 示例
  ///
  /// ```rust,ignore
  /// let result = UserRoute::parse_recursive("/users/123/profile/basic/456")?;
  /// println!("Current route: {:?}", result.current);
  /// println!("Sub route info: {:?}", result.sub_route_info);
  /// ```
  fn parse_recursive(path: &str) -> Result<NestedRouteResult<Self>, ParseError> {
    let (current, sub_route_state) = Self::parse_with_sub(path)?;

    let sub_route_info = match sub_route_state {
      RouteState::SubRoute(sub) => Some(Box::new(sub.to_route_info())),
      RouteState::NoSubRoute | RouteState::ParseFailed { .. } => None,
    };

    Ok(NestedRouteResult { current, sub_route_info })
  }

  /// 从完整路径自动解析多层嵌套路由
  ///
  /// 这个方法提供了一个通用的解决方案，可以从任意深度的路径中自动解析出对应的路由结构。
  /// 它会尝试匹配当前路由，然后递归解析剩余的路径。
  ///
  /// # 参数
  ///
  /// * `full_path` - 完整的路径字符串
  ///
  /// # 返回值
  ///
  /// 成功时返回解析结果和剩余未解析的路径
  ///
  /// # 示例
  ///
  /// ```rust,ignore
  /// let (result, remaining) = UserRoute::parse_from_full_path("/users/profile/basic/123")?;
  /// ```
  fn parse_from_full_path(full_path: &str) -> Result<(NestedRouteResult<Self>, &str), ParseError> {
    // 使用现有的 consumed_length 方法来计算消费的路径长度
    let consumed = Self::consumed_length(full_path)?;

    // 构建当前层级的路径
    let current_path = &full_path[..consumed];

    // 解析当前层级
    let (current, sub_route_state) = Self::parse_with_sub(current_path)?;

    // 获取剩余路径
    let remaining_path = &full_path[consumed..];

    // 处理子路由信息
    let sub_route_info = match sub_route_state {
      RouteState::SubRoute(sub) => Some(Box::new(sub.to_route_info())),
      RouteState::NoSubRoute | RouteState::ParseFailed { .. } => None,
    };

    let result = NestedRouteResult { current, sub_route_info };

    Ok((result, remaining_path))
  }

  /// 获取查询参数的字段名称列表
  ///
  /// # 返回值
  ///
  /// 包含所有查询参数字段名称的向量
  fn query_keys() -> Vec<&'static str> {
    // 默认实现返回空向量
    vec![]
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

  /// 获取查询参数的字段名称列表
  ///
  /// # 返回值
  ///
  /// 包含所有查询参数字段名称的向量
  fn query_keys() -> Vec<&'static str>;
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
