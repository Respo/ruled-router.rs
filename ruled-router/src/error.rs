//! 错误类型定义
//!
//! 定义了路由解析过程中可能出现的各种错误类型

use std::fmt;

/// 路由状态枚举
///
/// 用于替代 Option<SubRouterMatch>，提供更明确的路由解析状态信息
/// 支持 serde 序列化/反序列化
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RouteState<T> {
  /// 没有子路由（预期的叶子节点）
  ///
  /// 表示当前路由是一个叶子节点，不应该有子路由
  NoSubRoute,

  /// 有子路由
  ///
  /// 表示成功解析到子路由
  SubRoute(T),

  /// 解析失败
  ///
  /// 表示尝试解析子路由时失败，包含详细的调试信息
  ParseFailed {
    /// 剩余的路径
    remaining_path: String,
    /// 尝试匹配的模式列表
    attempted_patterns: Vec<String>,
    /// 最接近的匹配信息（可选）
    closest_match: Option<ClosestMatch>,
  },
}

/// 最接近的匹配信息
///
/// 用于提供更好的调试信息，帮助开发者理解为什么路由匹配失败
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClosestMatch {
  /// 匹配的模式
  pub pattern: String,
  /// 匹配的路径长度
  pub matched_length: usize,
  /// 失败的原因
  pub failure_reason: String,
}

/// 路由调试信息
///
/// 包含路由解析过程中的详细信息，用于调试和错误报告
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RouteDebugInfo {
  /// 失败的层级
  pub failed_at_level: usize,
  /// 已消费的路径
  pub consumed_path: String,
  /// 剩余的路径
  pub remaining_path: String,
  /// 可用的路由列表
  pub available_routes: Vec<String>,
  /// 建议的修复方案
  pub suggestion: Option<String>,
}

/// 解析错误类型
///
/// 表示在路由解析过程中可能出现的各种错误情况
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
  /// 无效的路径格式
  ///
  /// 当路径不符合预期格式时返回此错误
  InvalidPath(String),

  /// 缺少必需的参数
  ///
  /// 当路径中缺少必需的参数时返回此错误
  MissingParameter(String),

  /// 类型转换失败
  ///
  /// 当无法将字符串参数转换为目标类型时返回此错误
  TypeConversion(String),

  /// 无效的查询参数
  ///
  /// 当查询参数格式不正确时返回此错误
  InvalidQuery(String),

  /// URL 编码/解码错误
  ///
  /// 当 URL 编码或解码失败时返回此错误
  UrlEncoding(String),

  /// 路径段数量不匹配
  ///
  /// 当实际路径段数量与模式不匹配时返回此错误
  SegmentCountMismatch { expected: usize, actual: usize },

  /// 路径段内容不匹配
  ///
  /// 当路径段的字面量内容不匹配时返回此错误
  SegmentMismatch { expected: String, actual: String, position: usize },
}

impl fmt::Display for ParseError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      ParseError::InvalidPath(msg) => {
        write!(f, "Invalid path: {msg}")
      }
      ParseError::MissingParameter(param) => {
        write!(f, "Missing required parameter: {param}")
      }
      ParseError::TypeConversion(msg) => {
        write!(f, "Type conversion error: {msg}")
      }
      ParseError::InvalidQuery(msg) => {
        write!(f, "Invalid query parameter: {msg}")
      }
      ParseError::UrlEncoding(msg) => {
        write!(f, "URL encoding error: {msg}")
      }
      ParseError::SegmentCountMismatch { expected, actual } => {
        write!(f, "Path segment count mismatch: expected {expected} segments, found {actual}")
      }
      ParseError::SegmentMismatch {
        expected,
        actual,
        position,
      } => {
        write!(
          f,
          "Path segment mismatch at position {position}: expected '{expected}', found '{actual}'"
        )
      }
    }
  }
}

impl std::error::Error for ParseError {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    // 目前没有嵌套的错误源
    None
  }
}

/// 解析结果类型别名
///
/// 为了方便使用，定义了常用的 Result 类型别名
pub type ParseResult<T> = Result<T, ParseError>;

/// 错误构造辅助函数
impl ParseError {
  /// 创建无效路径错误
  pub fn invalid_path<S: Into<String>>(msg: S) -> Self {
    ParseError::InvalidPath(msg.into())
  }

  /// 创建缺少参数错误
  pub fn missing_parameter<S: Into<String>>(param: S) -> Self {
    ParseError::MissingParameter(param.into())
  }

  /// 创建类型转换错误
  pub fn type_conversion<S: Into<String>>(msg: S) -> Self {
    ParseError::TypeConversion(msg.into())
  }

  /// 创建无效查询错误
  pub fn invalid_query<S: Into<String>>(msg: S) -> Self {
    ParseError::InvalidQuery(msg.into())
  }

  /// 创建 URL 编码错误
  pub fn url_encoding<S: Into<String>>(msg: S) -> Self {
    ParseError::UrlEncoding(msg.into())
  }

  /// 创建段数量不匹配错误
  pub fn segment_count_mismatch(expected: usize, actual: usize) -> Self {
    ParseError::SegmentCountMismatch { expected, actual }
  }

  /// 创建段内容不匹配错误
  pub fn segment_mismatch<S: Into<String>>(expected: S, actual: S, position: usize) -> Self {
    ParseError::SegmentMismatch {
      expected: expected.into(),
      actual: actual.into(),
      position,
    }
  }
}

/// RouteState 的实用方法实现
impl<T> RouteState<T> {
  /// 创建一个没有子路由的状态
  pub fn no_sub_route() -> Self {
    RouteState::NoSubRoute
  }

  /// 创建一个有子路由的状态
  pub fn sub_route(sub_router: T) -> Self {
    RouteState::SubRoute(sub_router)
  }

  /// 创建一个解析失败的状态
  pub fn parse_failed<S: Into<String>>(
    remaining_path: S,
    attempted_patterns: Vec<String>,
    closest_match: Option<ClosestMatch>,
  ) -> Self {
    RouteState::ParseFailed {
      remaining_path: remaining_path.into(),
      attempted_patterns,
      closest_match,
    }
  }

  /// 检查是否没有子路由
  pub fn is_no_sub_route(&self) -> bool {
    matches!(self, RouteState::NoSubRoute)
  }

  /// 检查是否有子路由
  pub fn is_sub_route(&self) -> bool {
    matches!(self, RouteState::SubRoute(_))
  }

  /// 检查是否解析失败
  pub fn is_parse_failed(&self) -> bool {
    matches!(self, RouteState::ParseFailed { .. })
  }

  /// 获取子路由的引用（如果存在）
  pub fn as_sub_route(&self) -> Option<&T> {
    match self {
      RouteState::SubRoute(sub) => Some(sub),
      _ => None,
    }
  }

  /// 获取子路由的可变引用（如果存在）
  pub fn as_sub_route_mut(&mut self) -> Option<&mut T> {
    match self {
      RouteState::SubRoute(sub) => Some(sub),
      _ => None,
    }
  }

  /// 将 RouteState 转换为 Option（向后兼容）
  pub fn into_option(self) -> Option<T> {
    match self {
      RouteState::SubRoute(sub) => Some(sub),
      _ => None,
    }
  }

  /// 从 Option 创建 RouteState（向后兼容）
  pub fn from_option(option: Option<T>) -> Self {
    match option {
      Some(sub) => RouteState::SubRoute(sub),
      None => RouteState::NoSubRoute,
    }
  }

  /// 映射子路由类型
  pub fn map<U, F>(self, f: F) -> RouteState<U>
  where
    F: FnOnce(T) -> U,
  {
    match self {
      RouteState::NoSubRoute => RouteState::NoSubRoute,
      RouteState::SubRoute(sub) => RouteState::SubRoute(f(sub)),
      RouteState::ParseFailed {
        remaining_path,
        attempted_patterns,
        closest_match,
      } => RouteState::ParseFailed {
        remaining_path,
        attempted_patterns,
        closest_match,
      },
    }
  }

  /// 获取调试信息（如果是解析失败状态）
  pub fn debug_info(&self) -> Option<RouteDebugInfo> {
    match self {
      RouteState::ParseFailed {
        remaining_path,
        attempted_patterns,
        closest_match,
      } => Some(RouteDebugInfo {
        failed_at_level: 0,           // 默认值，可以在具体使用时设置
        consumed_path: String::new(), // 默认值，可以在具体使用时设置
        remaining_path: remaining_path.clone(),
        available_routes: attempted_patterns.clone(),
        suggestion: closest_match.as_ref().map(|m| {
          format!(
            "Did you mean '{}'? (matched {} characters, failed because: {})",
            m.pattern, m.matched_length, m.failure_reason
          )
        }),
      }),
      _ => None,
    }
  }
}

/// ClosestMatch 的实用方法实现
impl ClosestMatch {
  /// 创建一个新的最接近匹配信息
  pub fn new<S1: Into<String>, S2: Into<String>>(pattern: S1, matched_length: usize, failure_reason: S2) -> Self {
    ClosestMatch {
      pattern: pattern.into(),
      matched_length,
      failure_reason: failure_reason.into(),
    }
  }
}

/// RouteDebugInfo 的实用方法实现
impl RouteDebugInfo {
  /// 创建一个新的路由调试信息
  pub fn new<S1: Into<String>, S2: Into<String>>(
    failed_at_level: usize,
    consumed_path: S1,
    remaining_path: S2,
    available_routes: Vec<String>,
    suggestion: Option<String>,
  ) -> Self {
    RouteDebugInfo {
      failed_at_level,
      consumed_path: consumed_path.into(),
      remaining_path: remaining_path.into(),
      available_routes,
      suggestion,
    }
  }

  /// 生成人类可读的错误消息
  pub fn to_error_message(&self) -> String {
    let mut message = format!(
      "Route parsing failed at level {}: consumed '{}', remaining '{}'",
      self.failed_at_level, self.consumed_path, self.remaining_path
    );

    if !self.available_routes.is_empty() {
      message.push_str(&format!("\nAvailable routes: {}", self.available_routes.join(", ")));
    }

    if let Some(suggestion) = &self.suggestion {
      message.push_str(&format!("\nSuggestion: {suggestion}"));
    }

    message
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_error_display() {
    let error = ParseError::invalid_path("test path");
    assert_eq!(error.to_string(), "Invalid path: test path");

    let error = ParseError::missing_parameter("id");
    assert_eq!(error.to_string(), "Missing required parameter: id");

    let error = ParseError::segment_count_mismatch(3, 2);
    assert_eq!(error.to_string(), "Path segment count mismatch: expected 3 segments, found 2");

    let error = ParseError::segment_mismatch("user", "admin", 1);
    assert_eq!(
      error.to_string(),
      "Path segment mismatch at position 1: expected 'user', found 'admin'"
    );
  }

  #[test]
  fn test_error_equality() {
    let error1 = ParseError::invalid_path("test");
    let error2 = ParseError::invalid_path("test");
    let error3 = ParseError::invalid_path("other");

    assert_eq!(error1, error2);
    assert_ne!(error1, error3);
  }
}
