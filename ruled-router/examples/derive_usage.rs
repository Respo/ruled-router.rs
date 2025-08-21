//! 派生宏使用示例
//!
//! 演示如何使用 #[derive(Router)] 和 #[derive(Query)] 宏
//! 来自动生成 Router 和 Query trait 的实现。

use ruled_router::{Query, Router};

/// 用户路由 - 使用派生宏自动实现 Router trait
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/users/:id")]
struct UserRoute {
  id: u32,
}

/// 博客文章路由 - 支持多个路径参数
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/blog/:category/:slug")]
struct BlogRoute {
  category: String,
  slug: String,
}

/// API 路由 - 支持更复杂的路径
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/api/:version/users/:user_id/posts/:post_id")]
struct ApiRoute {
  version: String,
  user_id: u32,
  post_id: u32,
}

/// 搜索查询 - 使用派生宏自动实现 Query trait
#[derive(Debug, Clone, PartialEq, Default, Query)]
struct SearchQuery {
  q: Option<String>,
  page: Option<u32>,
  limit: Option<u32>,
  tags: Vec<String>,
}

/// 过滤查询 - 支持布尔值和枚举
#[derive(Debug, Clone, PartialEq, Default, Query)]
struct FilterQuery {
  active: Option<bool>,
  sort_by: Option<String>,
  order: Option<String>,
  categories: Vec<String>,
}

/// 分页查询 - 简单的分页参数
#[derive(Debug, Clone, PartialEq, Default, Query)]
struct PaginationQuery {
  page: Option<u32>,
  per_page: Option<u32>,
}

fn main() {
  println!("=== ruled-router 派生宏使用示例 ===");

  // 1. 路由派生宏演示
  println!("\n1. 路由派生宏演示:");

  // 用户路由
  println!("\n  用户路由:");
  let user_route = UserRoute::parse("/users/123").unwrap();
  println!("    解析: /users/123 -> {user_route:?}");
  println!("    格式化: {:?} -> {}", user_route, user_route.format());
  println!("    模式: {}", UserRoute::pattern());

  // 博客路由
  println!("\n  博客路由:");
  let blog_route = BlogRoute::parse("/blog/technology/rust-tutorial").unwrap();
  println!("    解析: /blog/technology/rust-tutorial -> {blog_route:?}");
  println!("    格式化: {:?} -> {}", blog_route, blog_route.format());
  println!("    模式: {}", BlogRoute::pattern());

  // API 路由
  println!("\n  API 路由:");
  let api_route = ApiRoute::parse("/api/v1/users/456/posts/789").unwrap();
  println!("    解析: /api/v1/users/456/posts/789 -> {api_route:?}");
  println!("    格式化: {:?} -> {}", api_route, api_route.format());
  println!("    模式: {}", ApiRoute::pattern());

  // 2. 查询派生宏演示
  println!("\n2. 查询派生宏演示:");

  // 搜索查询
  println!("\n  搜索查询:");
  let search_query = SearchQuery::parse("q=rust&page=1&limit=10&tags=tutorial&tags=beginner").unwrap();
  println!("    解析: q=rust&page=1&limit=10&tags=tutorial&tags=beginner");
  println!("    结果: {search_query:?}");
  println!("    格式化: {}", search_query.format());

  // 过滤查询
  println!("\n  过滤查询:");
  let filter_query = FilterQuery::parse("active=true&sort_by=date&order=desc&categories=tech&categories=programming").unwrap();
  println!("    解析: active=true&sort_by=date&order=desc&categories=tech&categories=programming");
  println!("    结果: {filter_query:?}");
  println!("    格式化: {}", filter_query.format());

  // 分页查询
  println!("\n  分页查询:");
  let pagination_query = PaginationQuery::parse("page=2&per_page=20").unwrap();
  println!("    解析: page=2&per_page=20");
  println!("    结果: {pagination_query:?}");
  println!("    格式化: {}", pagination_query.format());

  // 3. 组合使用演示
  println!("\n3. 组合使用演示:");

  let full_url = "/blog/technology/rust-tutorial?q=advanced&page=1&tags=async&tags=performance";
  let (path_part, query_part) = ruled_router::utils::split_path_query(full_url);

  println!("\n  完整 URL: {full_url}");
  println!("  路径部分: {path_part}");
  println!("  查询部分: {query_part:?}");

  // 解析路径
  let blog_route = BlogRoute::parse(path_part).unwrap();
  println!("  解析路径: {blog_route:?}");

  // 解析查询
  let search_query = SearchQuery::parse(query_part.unwrap_or("")).unwrap();
  println!("  解析查询: {search_query:?}");

  // 重新组合
  let reconstructed_path = blog_route.format();
  let reconstructed_query = search_query.format();
  let reconstructed_url = if reconstructed_query.is_empty() {
    reconstructed_path
  } else {
    format!("{reconstructed_path}?{reconstructed_query}")
  };

  println!("  重构 URL: {reconstructed_url}");

  // 4. 错误处理演示
  println!("\n4. 错误处理演示:");

  let error_cases = vec![
    "/users/abc",     // 无效的数字
    "/blog/category", // 缺少参数
    "/invalid/path",  // 不匹配的路径
  ];

  for case in error_cases {
    match UserRoute::parse(case) {
      Ok(route) => println!("  {case} -> 成功: {route:?}"),
      Err(e) => println!("  {case} -> 错误: {e:?}"),
    }
  }

  // 5. 性能对比演示
  println!("\n5. 性能对比演示:");

  use std::time::Instant;

  // 测试派生宏版本的性能
  let start = Instant::now();
  for i in 1..=1000 {
    let route = UserRoute { id: i };
    let formatted = route.format();
    let _parsed = UserRoute::parse(&formatted).unwrap();
  }
  let derive_duration = start.elapsed();

  println!("  派生宏版本 1000 次往返: {derive_duration:?}");
  println!("  平均每次: {:?}", derive_duration / 1000);

  println!("\n=== 派生宏示例完成 ===");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_user_route_derive() {
    let route = UserRoute { id: 123 };
    let formatted = route.format();
    assert_eq!(formatted, "/users/123");

    let parsed = UserRoute::parse(&formatted).unwrap();
    assert_eq!(parsed, route);
  }

  #[test]
  fn test_blog_route_derive() {
    let route = BlogRoute {
      category: "tech".to_string(),
      slug: "rust-guide".to_string(),
    };
    let formatted = route.format();
    assert_eq!(formatted, "/blog/tech/rust-guide");

    let parsed = BlogRoute::parse(&formatted).unwrap();
    assert_eq!(parsed, route);
  }

  #[test]
  fn test_search_query_derive() {
    let query = SearchQuery {
      q: Some("rust".to_string()),
      page: Some(1),
      limit: Some(10),
      tags: vec!["tutorial".to_string(), "beginner".to_string()],
    };

    let formatted = query.format();
    let parsed = SearchQuery::parse(&formatted).unwrap();

    assert_eq!(parsed.q, query.q);
    assert_eq!(parsed.page, query.page);
    assert_eq!(parsed.limit, query.limit);
    assert_eq!(parsed.tags, query.tags);
  }

  #[test]
  fn test_api_route_derive() {
    let route = ApiRoute {
      version: "v2".to_string(),
      user_id: 456,
      post_id: 789,
    };

    let formatted = route.format();
    assert_eq!(formatted, "/api/v2/users/456/posts/789");

    let parsed = ApiRoute::parse(&formatted).unwrap();
    assert_eq!(parsed, route);
  }

  #[test]
  fn test_filter_query_derive() {
    let query = FilterQuery {
      active: Some(true),
      sort_by: Some("date".to_string()),
      order: Some("desc".to_string()),
      categories: vec!["tech".to_string(), "programming".to_string()],
    };

    let formatted = query.format();
    let parsed = FilterQuery::parse(&formatted).unwrap();

    assert_eq!(parsed.active, query.active);
    assert_eq!(parsed.sort_by, query.sort_by);
    assert_eq!(parsed.order, query.order);
    assert_eq!(parsed.categories, query.categories);
  }

  #[test]
  fn test_empty_query_derive() {
    let query = PaginationQuery::default();
    let formatted = query.format();
    assert_eq!(formatted, "");

    let parsed = PaginationQuery::parse("").unwrap();
    assert_eq!(parsed, query);
  }

  #[test]
  fn test_pattern_methods() {
    assert_eq!(UserRoute::pattern(), "/users/:id");
    assert_eq!(BlogRoute::pattern(), "/blog/:category/:slug");
    assert_eq!(ApiRoute::pattern(), "/api/:version/users/:user_id/posts/:post_id");
  }
}
