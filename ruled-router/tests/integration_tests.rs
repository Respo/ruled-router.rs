//! 集成测试
//!
//! 测试整个库的功能集成

use ruled_router::{
  error::ParseError,
  formatter::{PathFormatter, QueryFormatter, UrlFormatter},
  parser::{PathParser, QueryParser},
  traits::{Query, RouterData, ToParam},
};
use ruled_router_derive::QueryDerive;
use std::collections::HashMap;

/// 简单的用户路由
#[derive(Debug, Clone, PartialEq)]
struct UserRoute {
  id: u32,
}

impl RouterData for UserRoute {
  type SubRouterMatch = ::ruled_router::NoSubRouter;

  fn parse(path: &str) -> Result<Self, ParseError> {
    let (path_part, _) = ruled_router::utils::split_path_query(path);
    let parser = PathParser::new("/users/:id")?;
    let params = parser.match_path(path_part)?;

    let id = params
      .get("id")
      .ok_or_else(|| ParseError::missing_parameter("id"))?
      .parse::<u32>()
      .map_err(|_| ParseError::type_conversion("Cannot convert id to u32".to_string()))?;

    Ok(Self { id })
  }

  fn format(&self) -> String {
    let mut params = HashMap::new();
    params.insert("id".to_string(), self.id.to_param());

    let formatter = PathFormatter::new("/users/:id").unwrap();
    formatter.format(&params).unwrap()
  }

  fn pattern() -> &'static str {
    "/users/:id"
  }
}

/// 简单的查询参数
#[derive(Debug, Clone, PartialEq, Default, QueryDerive)]
struct SimpleQuery {
  name: Option<String>,
  active: Option<bool>,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_basic_path_parsing() {
    let route = UserRoute::parse("/users/123").unwrap();
    assert_eq!(route.id, 123);

    let formatted = route.format();
    assert_eq!(formatted, "/users/123");
  }

  #[test]
  fn test_basic_query_parsing() {
    let query = SimpleQuery::parse("name=john&active=true").unwrap();
    assert_eq!(query.name, Some("john".to_string()));
    assert_eq!(query.active, Some(true));

    let formatted = query.format();
    assert!(formatted.contains("name=john"));
    assert!(formatted.contains("active=true"));
  }

  #[test]
  fn test_empty_query() {
    let query = SimpleQuery::parse("").unwrap();
    assert_eq!(query, SimpleQuery::default());

    let formatted = query.format();
    assert_eq!(formatted, "");
  }

  #[test]
  fn test_path_parser_direct() {
    let parser = PathParser::new("/api/:version/users/:id").unwrap();
    let params = parser.match_path("/api/v1/users/123").unwrap();

    assert_eq!(params.get("version"), Some(&"v1".to_string()));
    assert_eq!(params.get("id"), Some(&"123".to_string()));
  }

  #[test]
  fn test_query_parser_direct() {
    let parser = QueryParser::new("q=rust&page=2&tag=tutorial&tag=beginner").unwrap();

    assert_eq!(parser.get("q"), Some("rust"));
    assert_eq!(parser.get("page"), Some("2"));
    assert_eq!(parser.get_all("tag"), &["tutorial", "beginner"]);
  }

  #[test]
  fn test_path_formatter_direct() {
    let formatter = PathFormatter::new("/users/:id/posts/:post_id").unwrap();

    let mut params = HashMap::new();
    params.insert("id".to_string(), "123".to_string());
    params.insert("post_id".to_string(), "456".to_string());

    let result = formatter.format(&params).unwrap();
    assert_eq!(result, "/users/123/posts/456");
  }

  #[test]
  fn test_query_formatter_direct() {
    let mut formatter = QueryFormatter::new();
    formatter.set("q", "rust programming".to_string());
    formatter.set("page", "1".to_string());
    formatter.add("tag", "tutorial".to_string());
    formatter.add("tag", "beginner".to_string());

    let result = formatter.format();
    assert!(result.contains("q=rust%20programming"));
    assert!(result.contains("page=1"));
    assert!(result.contains("tag=tutorial"));
    assert!(result.contains("tag=beginner"));
  }

  #[test]
  fn test_url_formatter_direct() {
    let mut formatter = UrlFormatter::new("/api/:version/search").unwrap();
    formatter.query_formatter_mut().set("q", "rust".to_string());
    formatter.query_formatter_mut().set("limit", "10".to_string());

    let mut params = HashMap::new();
    params.insert("version".to_string(), "v1".to_string());

    let result = formatter.format(&params).unwrap();
    assert!(result.starts_with("/api/v1/search?"));
    assert!(result.contains("q=rust"));
    assert!(result.contains("limit=10"));
  }

  #[test]
  fn test_utils_functions() {
    // 测试路径和查询分离
    let (path, query) = ruled_router::utils::split_path_query("/users/123?name=john&active=true");
    assert_eq!(path, "/users/123");
    assert_eq!(query, Some("name=john&active=true"));

    // 测试路径段分离
    let segments = ruled_router::utils::split_path_segments("/api/v1/users");
    assert_eq!(segments, vec!["api", "v1", "users"]);

    // 测试 URL 编码/解码
    let encoded = ruled_router::utils::url_encode("hello world");
    assert_eq!(encoded, "hello%20world");

    let decoded = ruled_router::utils::url_decode("hello%20world").unwrap();
    assert_eq!(decoded, "hello world");

    // 测试查询字符串解析
    let params = ruled_router::utils::parse_query_string("name=john&age=30&tag=rust&tag=programming").unwrap();
    assert_eq!(params.get("name"), Some(&vec!["john".to_string()]));
    assert_eq!(params.get("age"), Some(&vec!["30".to_string()]));
    assert_eq!(params.get("tag"), Some(&vec!["rust".to_string(), "programming".to_string()]));
  }

  #[test]
  fn test_type_conversions() {
    let parser = QueryParser::new("id=123&active=true&score=98.5&name=john").unwrap();

    // 测试各种类型转换
    let id: u32 = parser.get_parsed("id").unwrap();
    assert_eq!(id, 123);

    let active: bool = parser.get_parsed("active").unwrap();
    assert!(active);

    let score: f64 = parser.get_parsed("score").unwrap();
    assert_eq!(score, 98.5);

    let name: String = parser.get_parsed("name").unwrap();
    assert_eq!(name, "john");

    // 测试可选类型
    let optional_id: Option<u32> = parser.get_optional("id").unwrap();
    assert_eq!(optional_id, Some(123));

    let missing: Option<u32> = parser.get_optional("missing").unwrap();
    assert_eq!(missing, None);
  }

  #[test]
  fn test_error_handling() {
    // 测试路径解析错误
    assert!(UserRoute::parse("/posts/123").is_err());
    assert!(UserRoute::parse("/users/abc").is_err());

    // 测试查询解析错误
    let parser = QueryParser::new("id=abc").unwrap();
    let result: Result<u32, _> = parser.get_parsed("id");
    assert!(result.is_err());

    // 测试缺少参数错误
    let parser = PathParser::new("/users/:id").unwrap();
    assert!(parser.match_path("/users").is_err());
  }

  #[test]
  fn test_roundtrip_consistency() {
    // 测试路由的往返一致性
    let original_route = UserRoute { id: 456 };
    let formatted = original_route.format();
    let parsed_route = UserRoute::parse(&formatted).unwrap();
    assert_eq!(original_route, parsed_route);

    // 测试查询的往返一致性
    let original_query = SimpleQuery {
      name: Some("test user".to_string()),
      active: Some(true),
    };
    let formatted = original_query.format();
    let parsed_query = SimpleQuery::parse(&formatted).unwrap();
    assert_eq!(original_query, parsed_query);
  }
}
