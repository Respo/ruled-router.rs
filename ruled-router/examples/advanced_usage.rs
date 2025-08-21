//! 高级用法示例
//!
//! 演示 ruled-router 库的高级功能，包括：
//! - 复杂的路由模式
//! - 嵌套查询参数
//! - 自定义类型转换
//! - 错误处理
//! - URL 格式化

use ruled_router::{
  error::ParseError,
  formatter::{PathFormatter, QueryFormatter, UrlFormatter},
  parser::{PathParser, QueryParser},
  traits::{FromParam, Query, Router, ToParam},
};
use std::collections::HashMap;

/// 自定义枚举类型
#[derive(Debug, Clone, PartialEq)]
enum UserRole {
  Admin,
  User,
  Guest,
}

impl FromParam for UserRole {
  fn from_param(param: &str) -> Result<Self, ParseError> {
    match param.to_lowercase().as_str() {
      "admin" => Ok(UserRole::Admin),
      "user" => Ok(UserRole::User),
      "guest" => Ok(UserRole::Guest),
      _ => Err(ParseError::type_conversion(format!("Invalid role: {param}"))),
    }
  }
}

impl ToParam for UserRole {
  fn to_param(&self) -> String {
    match self {
      UserRole::Admin => "admin".to_string(),
      UserRole::User => "user".to_string(),
      UserRole::Guest => "guest".to_string(),
    }
  }
}

/// 复杂的 API 路由
#[derive(Debug, Clone, PartialEq)]
struct ApiRoute {
  version: String,
  resource: String,
  id: Option<u32>,
  action: Option<String>,
}

impl Router for ApiRoute {
  fn parse(path: &str) -> Result<Self, ParseError> {
    let (path_part, _) = ruled_router::utils::split_path_query(path);

    // 尝试匹配不同的路由模式
    if let Ok(parser) = PathParser::new("/api/:version/:resource/:id/:action") {
      if let Ok(params) = parser.match_path(path_part) {
        return Ok(Self {
          version: params.get("version").unwrap().clone(),
          resource: params.get("resource").unwrap().clone(),
          id: Some(
            params
              .get("id")
              .unwrap()
              .parse()
              .map_err(|_| ParseError::type_conversion("Invalid id".to_string()))?,
          ),
          action: Some(params.get("action").unwrap().clone()),
        });
      }
    }

    if let Ok(parser) = PathParser::new("/api/:version/:resource/:id") {
      if let Ok(params) = parser.match_path(path_part) {
        return Ok(Self {
          version: params.get("version").unwrap().clone(),
          resource: params.get("resource").unwrap().clone(),
          id: Some(
            params
              .get("id")
              .unwrap()
              .parse()
              .map_err(|_| ParseError::type_conversion("Invalid id".to_string()))?,
          ),
          action: None,
        });
      }
    }

    if let Ok(parser) = PathParser::new("/api/:version/:resource") {
      if let Ok(params) = parser.match_path(path_part) {
        return Ok(Self {
          version: params.get("version").unwrap().clone(),
          resource: params.get("resource").unwrap().clone(),
          id: None,
          action: None,
        });
      }
    }

    Err(ParseError::invalid_path("No matching API route pattern".to_string()))
  }

  fn format(&self) -> String {
    let mut params = HashMap::new();
    params.insert("version".to_string(), self.version.clone());
    params.insert("resource".to_string(), self.resource.clone());

    let pattern = match (&self.id, &self.action) {
      (Some(id), Some(action)) => {
        params.insert("id".to_string(), id.to_string());
        params.insert("action".to_string(), action.clone());
        "/api/:version/:resource/:id/:action"
      }
      (Some(id), None) => {
        params.insert("id".to_string(), id.to_string());
        "/api/:version/:resource/:id"
      }
      _ => "/api/:version/:resource",
    };

    let formatter = PathFormatter::new(pattern).unwrap();
    formatter.format(&params).unwrap()
  }

  fn pattern() -> &'static str {
    "/api/:version/:resource(/:id)(/:action)"
  }
}

/// 复杂的搜索查询
#[derive(Debug, Clone, PartialEq, Default)]
struct SearchQuery {
  q: Option<String>,
  tags: Vec<String>,
  role: Option<UserRole>,
  page: Option<u32>,
  limit: Option<u32>,
  sort_by: Option<String>,
  order: Option<String>,
  active: Option<bool>,
}

impl Query for SearchQuery {
  fn parse(query: &str) -> Result<Self, ParseError> {
    let parser = QueryParser::new(query)?;

    Ok(Self {
      q: parser.get_optional("q")?,
      tags: parser.get_all("tag").iter().map(|s| s.to_string()).collect(),
      role: parser.get_optional("role")?,
      page: parser.get_optional("page")?,
      limit: parser.get_optional("limit")?,
      sort_by: parser.get_optional("sort_by")?,
      order: parser.get_optional("order")?,
      active: parser.get_optional("active")?,
    })
  }

  fn format(&self) -> String {
    let mut formatter = QueryFormatter::new();

    if let Some(ref q) = self.q {
      formatter.set("q", q.clone());
    }

    for tag in &self.tags {
      formatter.add("tag", tag.clone());
    }

    if let Some(ref role) = self.role {
      formatter.set("role", role.to_param());
    }

    if let Some(page) = self.page {
      formatter.set("page", page.to_string());
    }

    if let Some(limit) = self.limit {
      formatter.set("limit", limit.to_string());
    }

    if let Some(ref sort_by) = self.sort_by {
      formatter.set("sort_by", sort_by.clone());
    }

    if let Some(ref order) = self.order {
      formatter.set("order", order.clone());
    }

    if let Some(active) = self.active {
      formatter.set("active", active.to_string());
    }

    formatter.format()
  }
}

/// 完整的 URL 路由（路径 + 查询）
#[derive(Debug, Clone, PartialEq)]
struct FullRoute {
  api: ApiRoute,
  query: SearchQuery,
}

impl FullRoute {
  fn parse(url: &str) -> Result<Self, ParseError> {
    let (path_part, query_part) = ruled_router::utils::split_path_query(url);

    let api = ApiRoute::parse(path_part)?;
    let query = SearchQuery::parse(query_part.unwrap_or(""))?;

    Ok(Self { api, query })
  }

  fn format(&self) -> String {
    let path = self.api.format();
    let query = self.query.format();

    if query.is_empty() {
      path
    } else {
      format!("{path}?{query}")
    }
  }
}

fn main() {
  println!("=== ruled-router 高级用法示例 ===");

  // 1. 复杂路由解析
  println!("\n1. 复杂路由解析:");

  let routes = vec![
    "/api/v1/users",
    "/api/v1/users/123",
    "/api/v1/users/123/edit",
    "/api/v2/posts/456/delete",
  ];

  for route_str in routes {
    match ApiRoute::parse(route_str) {
      Ok(route) => {
        println!("  {route_str} -> {route:?}");
        println!("    格式化: {}", route.format());
      }
      Err(e) => println!("  {route_str} -> 错误: {e:?}"),
    }
  }

  // 2. 复杂查询解析
  println!("\n2. 复杂查询解析:");

  let queries = vec![
    "q=rust&tag=tutorial&tag=beginner&role=admin&page=1&limit=10&sort_by=date&order=desc&active=true",
    "q=hello%20world&role=user&page=2",
    "tag=programming&tag=rust&tag=web&active=false",
    "",
  ];

  for query_str in queries {
    match SearchQuery::parse(query_str) {
      Ok(query) => {
        println!("  {query_str} -> {query:?}");
        println!("    格式化: {}", query.format());
      }
      Err(e) => println!("  {query_str} -> 错误: {e:?}"),
    }
  }

  // 3. 完整 URL 解析
  println!("\n3. 完整 URL 解析:");

  let urls = vec![
    "/api/v1/users?q=john&role=admin&page=1",
    "/api/v1/posts/123?tag=tutorial&tag=rust&active=true",
    "/api/v2/comments/456/edit?sort_by=date&order=asc",
  ];

  for url in urls {
    match FullRoute::parse(url) {
      Ok(full_route) => {
        println!("  {} -> API: {:?}, Query: {:?}", url, full_route.api, full_route.query);
        println!("    格式化: {}", full_route.format());
      }
      Err(e) => println!("  {url} -> 错误: {e:?}"),
    }
  }

  // 4. 自定义类型转换
  println!("\n4. 自定义类型转换:");

  let role_tests = vec!["admin", "user", "guest", "invalid"];

  for role_str in role_tests {
    match UserRole::from_param(role_str) {
      Ok(role) => {
        println!("  {} -> {:?} -> {}", role_str, role, role.to_param());
      }
      Err(e) => println!("  {role_str} -> 错误: {e:?}"),
    }
  }

  // 5. URL 格式化器使用
  println!("\n5. URL 格式化器使用:");

  let mut url_formatter = UrlFormatter::new("/api/:version/:resource/:id").unwrap();

  // 设置查询参数
  url_formatter.query_formatter_mut().set("q", "rust programming".to_string());
  url_formatter.query_formatter_mut().add("tag", "tutorial".to_string());
  url_formatter.query_formatter_mut().add("tag", "beginner".to_string());
  url_formatter.query_formatter_mut().set("active", "true".to_string());

  // 设置路径参数
  let mut path_params = HashMap::new();
  path_params.insert("version".to_string(), "v1".to_string());
  path_params.insert("resource".to_string(), "tutorials".to_string());
  path_params.insert("id".to_string(), "123".to_string());

  match url_formatter.format(&path_params) {
    Ok(url) => println!("  完整 URL: {url}"),
    Err(e) => println!("  格式化错误: {e:?}"),
  }

  // 6. 错误处理演示
  println!("\n6. 错误处理演示:");

  let error_cases = vec![
    "/invalid/path",
    "/api/v1/users/abc", // 无效的 ID
    "/api",              // 路径太短
  ];

  for case in error_cases {
    match ApiRoute::parse(case) {
      Ok(_) => println!("  {case} -> 意外成功"),
      Err(e) => println!("  {case} -> 错误: {e:?}"),
    }
  }

  println!("\n=== 示例完成 ===");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_user_role_conversion() {
    assert_eq!(UserRole::from_param("admin").unwrap(), UserRole::Admin);
    assert_eq!(UserRole::from_param("USER").unwrap(), UserRole::User);
    assert_eq!(UserRole::from_param("Guest").unwrap(), UserRole::Guest);
    assert!(UserRole::from_param("invalid").is_err());

    assert_eq!(UserRole::Admin.to_param(), "admin");
    assert_eq!(UserRole::User.to_param(), "user");
    assert_eq!(UserRole::Guest.to_param(), "guest");
  }

  #[test]
  fn test_api_route_parsing() {
    let route = ApiRoute::parse("/api/v1/users/123/edit").unwrap();
    assert_eq!(route.version, "v1");
    assert_eq!(route.resource, "users");
    assert_eq!(route.id, Some(123));
    assert_eq!(route.action, Some("edit".to_string()));

    let formatted = route.format();
    assert_eq!(formatted, "/api/v1/users/123/edit");
  }

  #[test]
  fn test_search_query_parsing() {
    let query = SearchQuery::parse("q=rust&tag=tutorial&tag=beginner&role=admin&page=1&active=true").unwrap();
    assert_eq!(query.q, Some("rust".to_string()));
    assert_eq!(query.tags, vec!["tutorial", "beginner"]);
    assert_eq!(query.role, Some(UserRole::Admin));
    assert_eq!(query.page, Some(1));
    assert_eq!(query.active, Some(true));
  }

  #[test]
  fn test_full_route_roundtrip() {
    let original_url = "/api/v1/users/123?q=john&role=admin&page=1";
    let full_route = FullRoute::parse(original_url).unwrap();
    let formatted_url = full_route.format();

    // 解析格式化后的 URL 应该得到相同的结果
    let reparsed_route = FullRoute::parse(&formatted_url).unwrap();
    assert_eq!(full_route.api, reparsed_route.api);
    // 注意：查询参数的顺序可能不同，所以我们分别检查关键字段
    assert_eq!(full_route.query.q, reparsed_route.query.q);
    assert_eq!(full_route.query.role, reparsed_route.query.role);
    assert_eq!(full_route.query.page, reparsed_route.query.page);
  }
}
