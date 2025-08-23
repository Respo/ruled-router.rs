//! Router derive 宏测试
//!
//! 测试 #[derive(Router)] 宏的各种功能

use ruled_router::prelude::*;

/// 基础路由测试
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/users/:id")]
struct UserRoute {
  id: u32,
}

/// 多参数路由测试
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/blog/:category/:slug")]
struct BlogRoute {
  category: String,
  slug: String,
}

/// 复杂 API 路由测试
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/api/:version/users/:user_id/posts/:post_id")]
struct ApiRoute {
  version: String,
  user_id: u32,
  post_id: u64,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_user_route_derive() {
    let route = UserRoute::parse("/users/123").unwrap();
    assert_eq!(route.id, 123);
    assert_eq!(route.format(), "/users/123");
    assert_eq!(UserRoute::pattern(), "/users/:id");
  }

  #[test]
  fn test_blog_route_derive() {
    let route = BlogRoute::parse("/blog/tech/rust-tips").unwrap();
    assert_eq!(route.category, "tech");
    assert_eq!(route.slug, "rust-tips");
    assert_eq!(route.format(), "/blog/tech/rust-tips");
    assert_eq!(BlogRoute::pattern(), "/blog/:category/:slug");
  }

  #[test]
  fn test_api_route_derive() {
    let route = ApiRoute::parse("/api/v1/users/456/posts/789").unwrap();
    assert_eq!(route.version, "v1");
    assert_eq!(route.user_id, 456);
    assert_eq!(route.post_id, 789);
    assert_eq!(route.format(), "/api/v1/users/456/posts/789");
    assert_eq!(ApiRoute::pattern(), "/api/:version/users/:user_id/posts/:post_id");
  }

  #[test]
  fn test_pattern_methods() {
    assert_eq!(UserRoute::pattern(), "/users/:id");
    assert_eq!(BlogRoute::pattern(), "/blog/:category/:slug");
    assert_eq!(ApiRoute::pattern(), "/api/:version/users/:user_id/posts/:post_id");
  }

  #[test]
  fn test_error_handling() {
    // 测试无效路径
    assert!(UserRoute::parse("/invalid").is_err());
    assert!(BlogRoute::parse("/blog/only-one-param").is_err());
    assert!(ApiRoute::parse("/api/v1/incomplete").is_err());

    // 测试类型转换错误
    assert!(UserRoute::parse("/users/not-a-number").is_err());
    assert!(ApiRoute::parse("/api/v1/users/not-a-number/posts/123").is_err());
  }

  #[test]
  fn test_roundtrip_consistency() {
    let user_route = UserRoute { id: 123 };
    let parsed = UserRoute::parse(&user_route.format()).unwrap();
    assert_eq!(user_route, parsed);

    let blog_route = BlogRoute {
      category: "tech".to_string(),
      slug: "rust-guide".to_string(),
    };
    let parsed = BlogRoute::parse(&blog_route.format()).unwrap();
    assert_eq!(blog_route, parsed);

    let api_route = ApiRoute {
      version: "v2".to_string(),
      user_id: 999,
      post_id: 12345,
    };
    let parsed = ApiRoute::parse(&api_route.format()).unwrap();
    assert_eq!(api_route, parsed);
  }
}
