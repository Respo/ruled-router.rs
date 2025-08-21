//! 基础使用示例
//!
//! 这个示例展示了 ruled-router 库的基本功能：
//! - 使用派生宏定义路由和查询结构体
//! - 路径参数解析
//! - 查询参数解析
//! - URL 格式化
//! - 错误处理

use ruled_router::prelude::*;

// 定义用户路由结构体
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "users/:id")]
struct UserRoute {
  id: u32,
}

// 用户查询参数
#[derive(Debug, Clone, PartialEq, Default, Query)]
struct UserQuery {
  tab: Option<String>,
  active: Option<bool>,
}

// 定义博客路由结构体
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "blog/:year/:month/:slug")]
struct BlogPostRoute {
  year: u32,
  month: u32,
  slug: String,
}

// 博客查询参数
#[derive(Debug, Clone, PartialEq, Default, Query)]
struct BlogQuery {
  preview: Option<bool>,
  tags: Vec<String>,
}

// 定义搜索路由结构体
#[derive(Debug, Clone, PartialEq)]
struct SearchRoute;

// 手动实现 Router trait（因为没有路径参数）
impl Router for SearchRoute {
  fn parse(path: &str) -> Result<Self, ParseError> {
    if path == "/search" {
      Ok(SearchRoute)
    } else {
      Err(ParseError::invalid_path(format!("Expected /search, got {path}")))
    }
  }

  fn format(&self) -> String {
    "/search".to_string()
  }

  fn pattern() -> &'static str {
    "/search"
  }
}

// 搜索查询参数
#[derive(Debug, Clone, PartialEq, Default, Query)]
struct SearchQuery {
  q: String,
  page: Option<u32>,
  size: Option<u32>,
  sort: Option<String>,
  filters: Vec<String>,
}

fn main() {
  println!("=== ruled-router 基础使用示例 ===");
  println!();

  // 1. 用户路由示例
  println!("1. 用户路由示例:");

  // 解析用户路径
  let user_path = "/users/123";
  match UserRoute::parse(user_path) {
    Ok(route) => {
      println!("  解析: {user_path} -> {route:?}");

      // 格式化回路径
      let formatted = route.format();
      println!("  格式化: {route:?} -> {formatted}");

      // 显示路由模式
      println!("  模式: {}", UserRoute::pattern());
    }
    Err(e) => println!("  错误: {e:?}"),
  }

  // 解析用户查询参数
  let user_query_str = "tab=profile&active=true";
  match UserQuery::parse(user_query_str) {
    Ok(query) => {
      println!("  查询解析: {user_query_str} -> {query:?}");

      // 格式化回查询字符串
      let formatted = query.format();
      println!("  查询格式化: {query:?} -> {formatted}");
    }
    Err(e) => println!("  查询错误: {e:?}"),
  }
  println!();

  // 2. 博客路由示例
  println!("2. 博客路由示例:");

  // 解析博客路径
  let blog_path = "/blog/2024/01/rust-tutorial";
  match BlogPostRoute::parse(blog_path) {
    Ok(route) => {
      println!("  解析: {blog_path} -> {route:?}");

      // 格式化回路径
      let formatted = route.format();
      println!("  格式化: {route:?} -> {formatted}");

      // 显示路由模式
      println!("  模式: {}", BlogPostRoute::pattern());
    }
    Err(e) => println!("  错误: {e:?}"),
  }

  // 解析博客查询参数
  let blog_query_str = "preview=true&tags=rust&tags=tutorial";
  match BlogQuery::parse(blog_query_str) {
    Ok(query) => {
      println!("  查询解析: {blog_query_str} -> {query:?}");

      // 格式化回查询字符串
      let formatted = query.format();
      println!("  查询格式化: {query:?} -> {formatted}");
    }
    Err(e) => println!("  查询错误: {e:?}"),
  }
  println!();

  // 3. 搜索路由示例
  println!("3. 搜索路由示例:");

  // 解析搜索路径
  let search_path = "/search";
  match SearchRoute::parse(search_path) {
    Ok(route) => {
      println!("  解析: {search_path} -> {route:?}");

      // 格式化回路径
      let formatted = route.format();
      println!("  格式化: {route:?} -> {formatted}");

      // 显示路由模式
      println!("  模式: {}", SearchRoute::pattern());
    }
    Err(e) => println!("  错误: {e:?}"),
  }

  // 解析搜索查询参数
  let search_query_str = "q=rust&page=1&size=10&sort=date&filters=tutorial&filters=beginner";
  match SearchQuery::parse(search_query_str) {
    Ok(query) => {
      println!("  查询解析: {search_query_str} -> {query:?}");

      // 格式化回查询字符串
      let formatted = query.format();
      println!("  查询格式化: {query:?} -> {formatted}");
    }
    Err(e) => println!("  查询错误: {e:?}"),
  }
  println!();

  // 4. 错误处理示例
  println!("4. 错误处理示例:");

  // 无效的用户 ID
  match UserRoute::parse("/users/abc") {
    Ok(route) => println!("  意外成功: {route:?}"),
    Err(e) => println!("  预期错误: /users/abc -> {e:?}"),
  }

  // 缺少参数
  match UserRoute::parse("/users") {
    Ok(route) => println!("  意外成功: {route:?}"),
    Err(e) => println!("  预期错误: /users -> {e:?}"),
  }

  // 路径不匹配
  match UserRoute::parse("/posts/123") {
    Ok(route) => println!("  意外成功: {route:?}"),
    Err(e) => println!("  预期错误: /posts/123 -> {e:?}"),
  }
  println!();

  // 5. 格式化器使用示例
  println!("5. 格式化器使用示例:");

  // 创建用户路由并格式化
  let user_route = UserRoute { id: 456 };
  let user_query = UserQuery {
    tab: Some("settings".to_string()),
    active: Some(false),
  };

  println!("  用户路由: {user_route:?}");
  println!("  用户查询: {user_query:?}");
  println!("  完整 URL: {}?{}", user_route.format(), user_query.format());

  // 创建博客路由并格式化
  let blog_route = BlogPostRoute {
    year: 2024,
    month: 12,
    slug: "advanced-rust".to_string(),
  };
  let blog_query = BlogQuery {
    preview: Some(false),
    tags: vec!["rust".to_string(), "advanced".to_string()],
  };

  println!("  博客路由: {blog_route:?}");
  println!("  博客查询: {blog_query:?}");
  println!("  完整 URL: {}?{}", blog_route.format(), blog_query.format());

  println!();
  println!("=== 基础示例完成 ===");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_user_route() {
    let route = UserRoute { id: 123 };
    let path = route.format();
    let parsed = UserRoute::parse(&path).unwrap();
    assert_eq!(route, parsed);
  }

  #[test]
  fn test_blog_route() {
    let route = BlogPostRoute {
      year: 2024,
      month: 1,
      slug: "test".to_string(),
    };
    let path = route.format();
    let parsed = BlogPostRoute::parse(&path).unwrap();
    assert_eq!(route, parsed);
  }

  #[test]
  fn test_search_route() {
    let route = SearchRoute;
    let path = route.format();
    let parsed = SearchRoute::parse(&path).unwrap();
    assert_eq!(route, parsed);
  }

  #[test]
  fn test_user_query() {
    let query = UserQuery {
      tab: Some("profile".to_string()),
      active: Some(true),
    };
    let query_str = query.format();
    let parsed = UserQuery::parse(&query_str).unwrap();
    assert_eq!(query, parsed);
  }

  #[test]
  fn test_error_handling() {
    assert!(UserRoute::parse("/users/abc").is_err());
    assert!(UserRoute::parse("/users").is_err());
    assert!(UserRoute::parse("/posts/123").is_err());
  }
}
