//! 错误类型定义
//!
//! 定义了路由解析过程中可能出现的各种错误类型

use std::fmt;

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
