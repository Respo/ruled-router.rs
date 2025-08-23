//! 路径解析器
//!
//! 提供路径模式匹配和参数提取功能

use crate::error::{ParseError, ParseResult};
use crate::utils::{split_path_segments, url_decode};
use std::collections::HashMap;

/// 路径解析器
///
/// 负责解析路径模式并从实际路径中提取参数
#[derive(Debug, Clone)]
pub struct PathParser {
  /// 路径模式，例如 "/user/:id/profile"
  pattern: String,
  /// 解析后的模式段
  pattern_segments: Vec<PathSegment>,
}

/// 路径段类型
#[derive(Debug, Clone, PartialEq)]
pub enum PathSegment {
  /// 字面量段，例如 "user"
  Literal(String),
  /// 参数段，例如 ":id"
  Parameter(String),
  /// 可选参数段，例如 "?:optional"
  OptionalParameter(String),
  /// 通配符段，例如 "*path"
  Wildcard(String),
}

impl PathParser {
  /// 创建新的路径解析器
  ///
  /// # 参数
  ///
  /// * `pattern` - 路径模式字符串
  ///
  /// # 返回值
  ///
  /// 解析器实例，如果模式无效则返回错误
  ///
  /// # 示例
  ///
  /// ```rust
  /// use ruled_router::parser::PathParser;
  ///
  /// let parser = PathParser::new("/user/:id/profile").unwrap();
  /// ```
  pub fn new(pattern: &str) -> ParseResult<Self> {
    let pattern_segments = Self::parse_pattern(pattern)?;
    Ok(Self {
      pattern: pattern.to_string(),
      pattern_segments,
    })
  }

  /// 解析路径模式
  fn parse_pattern(pattern: &str) -> ParseResult<Vec<PathSegment>> {
    let segments = split_path_segments(pattern);
    let mut parsed_segments = Vec::new();

    for segment in segments {
      // 处理复合段，如 ":id?:format"
      if segment.contains("?:") && segment.starts_with(':') {
        // 分割复合段
        let parts: Vec<&str> = segment.split("?:").collect();
        if parts.len() == 2 {
          // 第一部分是必需参数
          let param_name = parts[0].strip_prefix(':').unwrap();
          if param_name.is_empty() {
            return Err(ParseError::invalid_path("Parameter must have a name"));
          }
          parsed_segments.push(PathSegment::Parameter(param_name.to_string()));

          // 第二部分是可选参数
          let optional_name = parts[1];
          if optional_name.is_empty() {
            return Err(ParseError::invalid_path("Optional parameter must have a name"));
          }
          parsed_segments.push(PathSegment::OptionalParameter(optional_name.to_string()));
          continue;
        }
      }

      let parsed_segment = if let Some(name) = segment.strip_prefix('*') {
        // 通配符段
        if name.is_empty() {
          return Err(ParseError::invalid_path("Wildcard segment must have a name"));
        }
        PathSegment::Wildcard(name.to_string())
      } else if let Some(name) = segment.strip_prefix("?:") {
        // 可选参数段 (?:name)
        if name.is_empty() {
          return Err(ParseError::invalid_path("Optional parameter must have a name"));
        }
        PathSegment::OptionalParameter(name.to_string())
      } else if let Some(name) = segment.strip_prefix(':') {
        // 冒号参数段 (:name)
        if name.is_empty() {
          return Err(ParseError::invalid_path("Parameter must have a name"));
        }
        PathSegment::Parameter(name.to_string())
      } else if segment.starts_with('{') && segment.ends_with('}') {
        // 大括号参数段 ({name})
        let name = &segment[1..segment.len() - 1];
        if name.is_empty() {
          return Err(ParseError::invalid_path("Parameter must have a name"));
        }
        PathSegment::Parameter(name.to_string())
      } else {
        // 字面量段
        PathSegment::Literal(segment.to_string())
      };

      parsed_segments.push(parsed_segment);
    }

    Ok(parsed_segments)
  }

  /// 匹配路径并提取参数
  ///
  /// # 参数
  ///
  /// * `path` - 要匹配的路径字符串
  ///
  /// # 返回值
  ///
  /// 提取的参数映射，如果匹配失败则返回错误
  ///
  /// # 示例
  ///
  /// ```rust
  /// use ruled_router::parser::PathParser;
  ///
  /// let parser = PathParser::new("/user/:id/profile").unwrap();
  /// let params = parser.match_path("/user/123/profile").unwrap();
  /// assert_eq!(params.get("id"), Some(&"123".to_string()));
  /// ```
  pub fn match_path(&self, path: &str) -> ParseResult<HashMap<String, String>> {
    let path_segments = split_path_segments(path);
    let mut params = HashMap::new();
    let mut path_index = 0;

    for (pattern_index, pattern_segment) in self.pattern_segments.iter().enumerate() {
      match pattern_segment {
        PathSegment::Literal(expected) => {
          if path_index >= path_segments.len() {
            return Err(ParseError::segment_count_mismatch(self.pattern_segments.len(), path_segments.len()));
          }

          let actual = path_segments[path_index];
          if actual != expected {
            return Err(ParseError::segment_mismatch(expected.clone(), actual.to_string(), pattern_index));
          }
          path_index += 1;
        }
        PathSegment::Parameter(name) => {
          if path_index >= path_segments.len() {
            return Err(ParseError::missing_parameter(name.clone()));
          }

          let value = url_decode(path_segments[path_index])?;
          params.insert(name.clone(), value);
          path_index += 1;
        }
        PathSegment::OptionalParameter(name) => {
          if path_index < path_segments.len() {
            let value = url_decode(path_segments[path_index])?;
            params.insert(name.clone(), value);
            path_index += 1;
          }
          // 可选参数，如果没有对应的路径段也不报错
        }
        PathSegment::Wildcard(name) => {
          // 通配符匹配剩余的所有段
          let remaining_segments: Vec<String> = path_segments[path_index..]
            .iter()
            .map(|s| url_decode(s))
            .collect::<ParseResult<Vec<_>>>()?;

          let wildcard_path = remaining_segments.join("/");
          params.insert(name.clone(), wildcard_path);

          // 通配符消耗所有剩余段
          path_index = path_segments.len();
          break;
        }
      }
    }

    // 检查是否还有未匹配的路径段
    if path_index < path_segments.len() {
      return Err(ParseError::segment_count_mismatch(self.pattern_segments.len(), path_segments.len()));
    }

    Ok(params)
  }

  /// 格式化路径
  ///
  /// 根据参数映射生成路径字符串
  ///
  /// # 参数
  ///
  /// * `params` - 参数映射
  ///
  /// # 返回值
  ///
  /// 格式化后的路径字符串
  ///
  /// # 示例
  ///
  /// ```rust
  /// use ruled_router::parser::PathParser;
  /// use std::collections::HashMap;
  ///
  /// let parser = PathParser::new("/user/:id/profile").unwrap();
  /// let mut params = HashMap::new();
  /// params.insert("id".to_string(), "123".to_string());
  /// let path = parser.format_path(&params).unwrap();
  /// assert_eq!(path, "/user/123/profile");
  /// ```
  pub fn format_path(&self, params: &HashMap<String, String>) -> ParseResult<String> {
    let mut segments = Vec::new();

    for segment in &self.pattern_segments {
      match segment {
        PathSegment::Literal(literal) => {
          segments.push(literal.clone());
        }
        PathSegment::Parameter(name) => {
          let value = params.get(name).ok_or_else(|| ParseError::missing_parameter(name.clone()))?;
          segments.push(crate::utils::url_encode(value));
        }
        PathSegment::OptionalParameter(name) => {
          if let Some(value) = params.get(name) {
            segments.push(crate::utils::url_encode(value));
          }
        }
        PathSegment::Wildcard(name) => {
          let value = params.get(name).ok_or_else(|| ParseError::missing_parameter(name.clone()))?;
          // 通配符值可能包含多个段，用 '/' 分隔
          segments.push(crate::utils::url_encode(value));
        }
      }
    }

    if segments.is_empty() {
      Ok("/".to_string())
    } else {
      Ok(format!("/{}", segments.join("/")))
    }
  }

  /// 获取路径模式
  pub fn pattern(&self) -> &str {
    &self.pattern
  }

  /// 获取模式段
  pub fn segments(&self) -> &[PathSegment] {
    &self.pattern_segments
  }

  /// 检查模式是否包含通配符
  pub fn has_wildcard(&self) -> bool {
    self.pattern_segments.iter().any(|s| matches!(s, PathSegment::Wildcard(_)))
  }

  /// 获取所有参数名
  pub fn parameter_names(&self) -> Vec<&str> {
    self
      .pattern_segments
      .iter()
      .filter_map(|s| match s {
        PathSegment::Parameter(name) | PathSegment::OptionalParameter(name) | PathSegment::Wildcard(name) => Some(name.as_str()),
        PathSegment::Literal(_) => None,
      })
      .collect()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_simple_pattern() {
    let parser = PathParser::new("/user/:id").unwrap();
    assert_eq!(parser.pattern(), "/user/:id");

    let segments = parser.segments();
    assert_eq!(segments.len(), 2);
    assert_eq!(segments[0], PathSegment::Literal("user".to_string()));
    assert_eq!(segments[1], PathSegment::Parameter("id".to_string()));
  }

  #[test]
  fn test_parse_complex_pattern() {
    let parser = PathParser::new("/api/:version/users/:id?:format/*path").unwrap();

    let segments = parser.segments();
    assert_eq!(segments.len(), 6);
    assert_eq!(segments[0], PathSegment::Literal("api".to_string()));
    assert_eq!(segments[1], PathSegment::Parameter("version".to_string()));
    assert_eq!(segments[2], PathSegment::Literal("users".to_string()));
    assert_eq!(segments[3], PathSegment::Parameter("id".to_string()));
    assert_eq!(segments[4], PathSegment::OptionalParameter("format".to_string()));
    assert_eq!(segments[5], PathSegment::Wildcard("path".to_string()));
  }

  #[test]
  fn test_match_simple_path() {
    let parser = PathParser::new("/user/:id").unwrap();
    let params = parser.match_path("/user/123").unwrap();

    assert_eq!(params.get("id"), Some(&"123".to_string()));
  }

  #[test]
  fn test_match_complex_path() {
    let parser = PathParser::new("/api/:version/users/:id").unwrap();
    let params = parser.match_path("/api/v1/users/456").unwrap();

    assert_eq!(params.get("version"), Some(&"v1".to_string()));
    assert_eq!(params.get("id"), Some(&"456".to_string()));
  }

  #[test]
  fn test_match_with_optional() {
    let parser = PathParser::new("/user/:id?:format").unwrap();

    // 有可选参数
    let params = parser.match_path("/user/123/json").unwrap();
    assert_eq!(params.get("id"), Some(&"123".to_string()));
    assert_eq!(params.get("format"), Some(&"json".to_string()));

    // 没有可选参数
    let params = parser.match_path("/user/123").unwrap();
    assert_eq!(params.get("id"), Some(&"123".to_string()));
    assert_eq!(params.get("format"), None);
  }

  #[test]
  fn test_match_with_wildcard() {
    let parser = PathParser::new("/files/*path").unwrap();
    let params = parser.match_path("/files/docs/readme.txt").unwrap();

    assert_eq!(params.get("path"), Some(&"docs/readme.txt".to_string()));
  }

  #[test]
  fn test_format_path() {
    let parser = PathParser::new("/user/:id/profile").unwrap();
    let mut params = HashMap::new();
    params.insert("id".to_string(), "123".to_string());

    let path = parser.format_path(&params).unwrap();
    assert_eq!(path, "/user/123/profile");
  }

  #[test]
  fn test_match_errors() {
    let parser = PathParser::new("/user/:id").unwrap();

    // 段数不匹配
    assert!(parser.match_path("/user").is_err());
    assert!(parser.match_path("/user/123/extra").is_err());

    // 字面量不匹配
    assert!(parser.match_path("/admin/123").is_err());
  }

  #[test]
  fn test_parameter_names() {
    let parser = PathParser::new("/api/:version/users/:id?:format/*path").unwrap();
    let names = parser.parameter_names();

    assert_eq!(names, vec!["version", "id", "format", "path"]);
  }

  #[test]
  fn test_has_wildcard() {
    let parser1 = PathParser::new("/user/:id").unwrap();
    assert!(!parser1.has_wildcard());

    let parser2 = PathParser::new("/files/*path").unwrap();
    assert!(parser2.has_wildcard());
  }
}
