//! 嵌套路由使用示例
//!
//! 这个示例展示了如何使用 #[derive(Router)] 和 #[derive(Query)] 实现嵌套路由结构。
//! 主要特性：
//! - 展示路径参数的嵌套结构
//! - 演示如何组合使用 Router 和 Query
//! - 支持复杂的 URL 解析和构建
//! - 提供实用的路由组合模式

use ruled_router::prelude::*;
use std::fmt;

// ===== 查询参数定义 =====

/// 用户查询参数
#[derive(Debug, Clone, PartialEq, Default)]
struct UserQuery {
  /// 包含的字段列表
  include_fields: Vec<String>,
  /// 是否包含敏感信息
  include_sensitive: Option<bool>,
  /// 响应格式
  format: Option<String>,
}

impl Query for UserQuery {
  fn parse(query: &str) -> Result<Self, ParseError> {
    let parser = QueryParser::new(query)?;
    Ok(Self {
      include_fields: parser.get_all("include").iter().map(|s| s.to_string()).collect(),
      include_sensitive: parser.get_optional("include_sensitive")?,
      format: parser.get_optional("format")?,
    })
  }

  fn format(&self) -> String {
    let mut formatter = QueryFormatter::new();
    for field in &self.include_fields {
      formatter.add("include", field.clone());
    }
    if let Some(sensitive) = self.include_sensitive {
      formatter.set("include_sensitive", sensitive.to_string());
    }
    if let Some(format) = &self.format {
      formatter.set("format", format.clone());
    }
    formatter.format()
  }

  fn from_query_map(query_map: &std::collections::HashMap<String, Vec<String>>) -> Result<Self, ParseError> {
    Ok(Self {
      include_fields: query_map
        .get("include")
        .map(|values| values.iter().map(|s| s.to_string()).collect())
        .unwrap_or_default(),
      include_sensitive: query_map
        .get("include_sensitive")
        .and_then(|values| values.first())
        .and_then(|s| s.parse().ok()),
      format: query_map.get("format").and_then(|values| values.first()).map(|s| s.to_string()),
    })
  }

  fn to_query_string(&self) -> String {
    self.format()
  }
}

/// 博客文章查询参数
#[derive(Debug, Clone, PartialEq, Default)]
struct PostQuery {
  /// 是否包含评论
  include_comments: Option<bool>,
  /// 评论排序方式
  comment_sort: Option<String>,
  /// 评论页码
  comment_page: Option<u32>,
  /// 是否包含草稿
  include_draft: Option<bool>,
}

impl Query for PostQuery {
  fn parse(query: &str) -> Result<Self, ParseError> {
    let parser = QueryParser::new(query)?;
    Ok(Self {
      include_comments: parser.get_optional("include_comments")?,
      comment_sort: parser.get_optional("comment_sort")?,
      comment_page: parser.get_optional("comment_page")?,
      include_draft: parser.get_optional("include_draft")?,
    })
  }

  fn format(&self) -> String {
    let mut formatter = QueryFormatter::new();
    if let Some(comments) = self.include_comments {
      formatter.set("include_comments", comments.to_string());
    }
    if let Some(sort) = &self.comment_sort {
      formatter.set("comment_sort", sort.clone());
    }
    if let Some(page) = self.comment_page {
      formatter.set("comment_page", page.to_string());
    }
    if let Some(draft) = self.include_draft {
      formatter.set("include_draft", draft.to_string());
    }
    formatter.format()
  }

  fn from_query_map(query_map: &std::collections::HashMap<String, Vec<String>>) -> Result<Self, ParseError> {
    Ok(Self {
      include_comments: query_map
        .get("include_comments")
        .and_then(|values| values.first())
        .and_then(|s| s.parse().ok()),
      comment_sort: query_map
        .get("comment_sort")
        .and_then(|values| values.first())
        .map(|s| s.to_string()),
      comment_page: query_map
        .get("comment_page")
        .and_then(|values| values.first())
        .and_then(|s| s.parse().ok()),
      include_draft: query_map
        .get("include_draft")
        .and_then(|values| values.first())
        .and_then(|s| s.parse().ok()),
    })
  }

  fn to_query_string(&self) -> String {
    self.format()
  }
}

/// API 版本查询参数
#[derive(Debug, Clone, PartialEq, Default)]
struct ApiQuery {
  /// API 版本
  version: Option<String>,
  /// 调试模式
  debug: Option<bool>,
  /// 响应格式
  format: Option<String>,
}

impl Query for ApiQuery {
  fn parse(query: &str) -> Result<Self, ParseError> {
    let parser = QueryParser::new(query)?;
    Ok(Self {
      version: parser.get_optional("version")?,
      debug: parser.get_optional("debug")?,
      format: parser.get_optional("format")?,
    })
  }

  fn format(&self) -> String {
    let mut formatter = QueryFormatter::new();
    if let Some(version) = &self.version {
      formatter.set("version", version.clone());
    }
    if let Some(debug) = self.debug {
      formatter.set("debug", debug.to_string());
    }
    if let Some(format) = &self.format {
      formatter.set("format", format.clone());
    }
    formatter.format()
  }

  fn from_query_map(query_map: &std::collections::HashMap<String, Vec<String>>) -> Result<Self, ParseError> {
    Ok(Self {
      version: query_map.get("version").and_then(|values| values.first()).map(|s| s.to_string()),
      debug: query_map
        .get("debug")
        .and_then(|values| values.first())
        .and_then(|s| s.parse().ok()),
      format: query_map.get("format").and_then(|values| values.first()).map(|s| s.to_string()),
    })
  }

  fn to_query_string(&self) -> String {
    self.format()
  }
}

// ===== 路由定义 =====

/// 用户路由
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/users/:id")]
struct UserRoute {
  id: u32,
  #[query]
  query: UserQuery,
}

/// 博客文章路由
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/blog/:year/:month/:slug")]
struct PostRoute {
  year: u32,
  month: u32,
  slug: String,
}

/// 用户博客路由 - 嵌套路径参数
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/users/:user_id/blog/:year/:month/:slug")]
struct UserPostRoute {
  user_id: u32,
  year: u32,
  month: u32,
  slug: String,
}

/// API 根路由
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/api/:version")]
struct ApiRoute {
  version: String,
}

/// API 用户路由 - 嵌套 API 版本和用户ID
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/api/:version/users/:id")]
struct ApiUserRoute {
  version: String,
  id: u32,
}

/// 复杂嵌套路由 - 多层路径参数
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/api/:version/users/:user_id/posts/:post_id/comments/:comment_id")]
struct NestedCommentRoute {
  version: String,
  user_id: u32,
  post_id: u32,
  comment_id: u32,
}

// ===== 路由和查询的组合结构 =====

/// 用户路由和查询的组合
#[derive(Debug, Clone, PartialEq)]
struct UserRouteWithQuery {
  route: UserRoute,
  query: UserQuery,
}

/// 博客文章路由和查询的组合
#[derive(Debug, Clone, PartialEq)]
struct PostRouteWithQuery {
  route: PostRoute,
  query: PostQuery,
}

/// API 用户路由和查询的组合
#[derive(Debug, Clone, PartialEq)]
struct ApiUserRouteWithQuery {
  route: ApiUserRoute,
  query: UserQuery,
}

// ===== 实用函数 =====

/// 解析完整的 URL（路径 + 查询参数）
fn parse_full_url<R, Q>(url: &str) -> Result<(R, Q), Box<dyn std::error::Error>>
where
  R: Router,
  Q: Query + Default,
{
  let (path, query_str) = if let Some(pos) = url.find('?') {
    (&url[..pos], &url[pos + 1..])
  } else {
    (url, "")
  };

  let route = R::parse(path)?;
  let query = if !query_str.is_empty() {
    Q::parse(query_str)?
  } else {
    Q::default()
  };

  Ok((route, query))
}

/// 构建完整的 URL
fn build_full_url<R, Q>(route: &R, query: &Q) -> String
where
  R: Router,
  Q: Query,
{
  let path = route.format();
  let query_str = query.format();

  if query_str.is_empty() {
    path
  } else {
    format!("{}?{}", path, query_str)
  }
}

/// 显示路由和查询信息
fn display_route_query_info<R, Q>(route: &R, query: &Q, description: &str)
where
  R: Router + fmt::Debug,
  Q: Query + fmt::Debug,
{
  println!("\n=== {} ===", description);
  println!("路由结构: {:#?}", route);
  println!("查询结构: {:#?}", query);
  println!("路径: {}", route.format());
  println!("查询: {}", query.format());
  println!("完整URL: {}", build_full_url(route, query));
}

/// 显示单独的路由信息
fn display_route_info<R>(route: &R, description: &str)
where
  R: Router + fmt::Debug,
{
  println!("\n=== {} ===", description);
  println!("路由结构: {:#?}", route);
  println!("路径: {}", route.format());
}

fn main() {
  println!("=== 嵌套路由使用示例 ===");

  // 1. 基本用户路由示例
  println!("\n1. 基本用户路由示例:");
  let user_url = "/users/123?include=profile&include=settings&include_sensitive=true&format=json";
  println!("  URL: {}", user_url);

  match parse_full_url::<UserRoute, UserQuery>(user_url) {
    Ok((route, query)) => {
      display_route_query_info(&route, &query, "用户路由解析结果");

      // 验证往返转换
      let reconstructed = build_full_url(&route, &query);
      println!("  往返转换: {}", reconstructed);
    }
    Err(e) => println!("  解析失败: {}", e),
  }

  // 2. 博客文章路由示例
  println!("\n2. 博客文章路由示例:");
  let post_url = "/blog/2024/01/rust-async-programming?include_comments=true&comment_sort=date&comment_page=2";
  println!("  URL: {}", post_url);

  match parse_full_url::<PostRoute, PostQuery>(post_url) {
    Ok((route, query)) => {
      display_route_query_info(&route, &query, "博客文章路由解析结果");
    }
    Err(e) => println!("  解析失败: {}", e),
  }

  // 3. 用户博客路由示例（嵌套路由）
  println!("\n3. 用户博客路由示例:");
  let user_post_url = "/users/456/blog/2024/01/my-first-post?include_comments=false&include_draft=true";
  println!("  URL: {}", user_post_url);

  match parse_full_url::<UserPostRoute, PostQuery>(user_post_url) {
    Ok((route, query)) => {
      display_route_query_info(&route, &query, "用户博客路由解析结果");
    }
    Err(e) => println!("  解析失败: {}", e),
  }

  // 4. API 路由示例
  println!("\n4. API 路由示例:");
  let api_url = "/api/v2?debug=true&format=xml";
  println!("  URL: {}", api_url);

  match parse_full_url::<ApiRoute, ApiQuery>(api_url) {
    Ok((route, query)) => {
      display_route_query_info(&route, &query, "API 路由解析结果");
    }
    Err(e) => println!("  解析失败: {}", e),
  }

  // 5. API 用户路由示例（嵌套 API + 用户）
  println!("\n5. API 用户路由示例:");
  let api_user_url = "/api/v1/users/789?include=profile&include=permissions&format=json";
  println!("  URL: {}", api_user_url);

  match parse_full_url::<ApiUserRoute, UserQuery>(api_user_url) {
    Ok((route, query)) => {
      display_route_query_info(&route, &query, "API 用户路由解析结果");
    }
    Err(e) => println!("  解析失败: {}", e),
  }

  // 6. 复杂嵌套路由示例
  println!("\n6. 复杂嵌套路由示例:");
  let nested_url = "/api/v3/users/100/posts/200/comments/300?version=beta&debug=false";
  println!("  URL: {}", nested_url);

  match parse_full_url::<NestedCommentRoute, ApiQuery>(nested_url) {
    Ok((route, query)) => {
      display_route_query_info(&route, &query, "复杂嵌套路由解析结果");
    }
    Err(e) => println!("  解析失败: {}", e),
  }

  // 7. 手动构建路由示例
  println!("\n7. 手动构建路由示例:");

  let manual_route = UserRoute { 
    id: 999,
    query: UserQuery {
      include_fields: vec!["profile".to_string(), "settings".to_string(), "preferences".to_string()],
      include_sensitive: Some(false),
      format: Some("xml".to_string()),
    },
  };
  let manual_query = UserQuery {
    include_fields: vec!["profile".to_string(), "settings".to_string(), "preferences".to_string()],
    include_sensitive: Some(false),
    format: Some("xml".to_string()),
  };

  display_route_query_info(&manual_route, &manual_query, "手动构建的用户路由");

  // 8. 组合结构示例
  println!("\n8. 组合结构示例:");

  let combined = UserRouteWithQuery {
    route: UserRoute { 
      id: 555,
      query: UserQuery {
        include_fields: vec!["profile".to_string(), "avatar".to_string()],
        include_sensitive: Some(true),
        format: Some("json".to_string()),
      },
    },
    query: UserQuery {
      include_fields: vec!["profile".to_string(), "avatar".to_string()],
      include_sensitive: Some(true),
      format: Some("json".to_string()),
    },
  };

  println!("  组合结构: {:#?}", combined);
  println!("  路径: {}", combined.route.format());
  println!("  查询: {}", combined.query.format());
  println!("  完整URL: {}", build_full_url(&combined.route, &combined.query));

  // 9. 性能测试
  println!("\n9. 性能测试:");

  let test_urls = vec![
    "/users/1?include=profile",
    "/blog/2024/01/test?include_comments=true",
    "/api/v1/users/2?format=json",
    "/api/v2/users/3/posts/4/comments/5?debug=true",
  ];

  let iterations = 1000;
  let start = std::time::Instant::now();

  for _ in 0..iterations {
    for url in &test_urls {
      if url.starts_with("/users") {
        let _ = parse_full_url::<UserRoute, UserQuery>(url);
      } else if url.starts_with("/blog") {
        let _ = parse_full_url::<PostRoute, PostQuery>(url);
      } else if url.contains("/users/") && url.contains("/posts/") {
        let _ = parse_full_url::<NestedCommentRoute, ApiQuery>(url);
      } else if url.contains("/api/") && url.contains("/users/") {
        let _ = parse_full_url::<ApiUserRoute, UserQuery>(url);
      }
    }
  }

  let duration = start.elapsed();
  println!(
    "  {} 次嵌套路由解析耗时: {:?} (平均: {:?})",
    iterations * test_urls.len(),
    duration,
    duration / (iterations * test_urls.len()) as u32
  );

  println!("\n=== 嵌套路由示例完成 ===");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_user_route_with_query() {
    let url = "/users/123?include=profile&format=json";
    let (route, query) = parse_full_url::<UserRoute, UserQuery>(url).unwrap();

    assert_eq!(route.id, 123);
    assert_eq!(query.include_fields, vec!["profile"]);
    assert_eq!(query.format, Some("json".to_string()));

    let reconstructed = build_full_url(&route, &query);
    assert!(reconstructed.contains("users/123"));
    assert!(reconstructed.contains("include=profile"));
    assert!(reconstructed.contains("format=json"));
  }

  #[test]
  fn test_post_route_with_query() {
    let url = "/blog/2024/01/test-post?include_comments=true&comment_page=2";
    let (route, query) = parse_full_url::<PostRoute, PostQuery>(url).unwrap();

    assert_eq!(route.year, 2024);
    assert_eq!(route.month, 1);
    assert_eq!(route.slug, "test-post");
    assert_eq!(query.include_comments, Some(true));
    assert_eq!(query.comment_page, Some(2));
  }

  #[test]
  fn test_nested_user_post_route() {
    let url = "/users/456/blog/2024/12/my-post?include_draft=true";
    let (route, query) = parse_full_url::<UserPostRoute, PostQuery>(url).unwrap();

    assert_eq!(route.user_id, 456);
    assert_eq!(route.year, 2024);
    assert_eq!(route.month, 12);
    assert_eq!(route.slug, "my-post");
    assert_eq!(query.include_draft, Some(true));
  }

  #[test]
  fn test_api_route_with_query() {
    let url = "/api/v2?debug=true&format=xml";
    let (route, query) = parse_full_url::<ApiRoute, ApiQuery>(url).unwrap();

    assert_eq!(route.version, "v2");
    assert_eq!(query.debug, Some(true));
    assert_eq!(query.format, Some("xml".to_string()));
  }

  #[test]
  fn test_api_user_route() {
    let url = "/api/v1/users/789?include=profile&include=settings";
    let (route, query) = parse_full_url::<ApiUserRoute, UserQuery>(url).unwrap();

    assert_eq!(route.version, "v1");
    assert_eq!(route.id, 789);
    assert_eq!(query.include_fields, vec!["profile", "settings"]);
  }

  #[test]
  fn test_complex_nested_route() {
    let url = "/api/v3/users/100/posts/200/comments/300?version=beta";
    let (route, query) = parse_full_url::<NestedCommentRoute, ApiQuery>(url).unwrap();

    assert_eq!(route.version, "v3");
    assert_eq!(route.user_id, 100);
    assert_eq!(route.post_id, 200);
    assert_eq!(route.comment_id, 300);
    assert_eq!(query.version, Some("beta".to_string()));
  }

  #[test]
  fn test_route_without_query() {
    let url = "/users/123";
    let (route, query) = parse_full_url::<UserRoute, UserQuery>(url).unwrap();

    assert_eq!(route.id, 123);
    assert_eq!(query, UserQuery::default());

    let reconstructed = build_full_url(&route, &query);
    assert_eq!(reconstructed, "/users/123");
  }

  #[test]
  fn test_manual_route_construction() {
    let route = ApiUserRoute {
      version: "v2".to_string(),
      id: 555,
    };

    let query = UserQuery {
      include_fields: vec!["profile".to_string()],
      include_sensitive: Some(false),
      format: Some("json".to_string()),
    };

    let url = build_full_url(&route, &query);
    assert!(url.contains("/api/v2/users/555"));
    assert!(url.contains("include=profile"));
    assert!(url.contains("format=json"));

    // 测试往返转换
    let (reparsed_route, reparsed_query) = parse_full_url::<ApiUserRoute, UserQuery>(&url).unwrap();
    assert_eq!(reparsed_route.version, route.version);
    assert_eq!(reparsed_route.id, route.id);
    assert_eq!(reparsed_query.include_fields, query.include_fields);
  }

  #[test]
  fn test_combined_structures() {
    let combined = UserRouteWithQuery {
      route: UserRoute { 
        id: 777,
        query: UserQuery {
          include_fields: vec!["profile".to_string(), "settings".to_string()],
          include_sensitive: Some(true),
          format: Some("xml".to_string()),
        },
      },
      query: UserQuery {
        include_fields: vec!["profile".to_string(), "settings".to_string()],
        include_sensitive: Some(true),
        format: Some("xml".to_string()),
      },
    };

    let url = build_full_url(&combined.route, &combined.query);
    assert!(url.contains("/users/777"));
    assert!(url.contains("include=profile"));
    assert!(url.contains("include=settings"));
    assert!(url.contains("format=xml"));

    // 验证可以重新解析
    let (route, query) = parse_full_url::<UserRoute, UserQuery>(&url).unwrap();
    assert_eq!(route.id, combined.route.id);
    assert_eq!(query.include_fields, combined.query.include_fields);
  }
}
