//! Router derive 宏测试
//!
//! 测试 #[derive(Router)] 宏的各种功能，包括自动前缀提取

use ruled_router::prelude::*;

/// 基础路由测试
#[derive(Debug, Clone, PartialEq, RouterData)]
#[router(pattern = "/users/:id")]
struct UserRoute {
  id: u32,
}

/// 多参数路由测试
#[derive(Debug, Clone, PartialEq, RouterData)]
#[router(pattern = "/blog/:category/:slug")]
struct BlogRoute {
  category: String,
  slug: String,
}

/// 复杂 API 路由测试
#[derive(Debug, Clone, PartialEq, RouterData)]
#[router(pattern = "/api/:version/users/:user_id/posts/:post_id")]
struct ApiRoute {
  version: String,
  user_id: u32,
  post_id: u64,
}

/// 带查询参数的路由测试
#[derive(Debug, Clone, PartialEq, RouterData)]
#[router(pattern = "/search/:category")]
struct SearchRoute {
  category: String,
  #[query]
  params: SearchParams,
}

/// 搜索参数
#[derive(Debug, Clone, PartialEq, Default, QueryDerive)]
struct SearchParams {
  q: Option<String>,
  page: Option<u32>,
  limit: Option<u32>,
  tags: Vec<String>,
}

/// 嵌套路由测试（带子路由）
#[derive(Debug, Clone, PartialEq, RouterData)]
#[router(pattern = "/modules/:name")]
struct ModuleRoute {
  name: String,
  #[query]
  options: ModuleOptions,
  #[sub_router]
  sub_router: Option<SubRouteType>,
}

/// 模块选项
#[derive(Debug, Clone, PartialEq, Default, QueryDerive)]
struct ModuleOptions {
  version: Option<String>,
  debug: Option<bool>,
}

/// 子路由类型（占位符）
type SubRouteType = ruled_router::NoSubRouter;

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

  #[test]
  fn test_search_route_with_query_params() {
    // 测试带查询参数的路由解析
    let route = SearchRoute::parse("/search/tech?q=rust&page=2&limit=10&tags=web&tags=backend").unwrap();
    assert_eq!(route.category, "tech");
    assert_eq!(route.params.q, Some("rust".to_string()));
    assert_eq!(route.params.page, Some(2));
    assert_eq!(route.params.limit, Some(10));
    assert_eq!(route.params.tags, vec!["web".to_string(), "backend".to_string()]);

    // 测试格式化
    let formatted = route.format();
    assert!(formatted.contains("/search/tech"));
    assert!(formatted.contains("q=rust"));
    assert!(formatted.contains("page=2"));
    assert!(formatted.contains("limit=10"));
    assert!(formatted.contains("tags=web"));
    assert!(formatted.contains("tags=backend"));

    // 测试模式
    assert_eq!(SearchRoute::pattern(), "/search/:category");
  }

  #[test]
  fn test_module_route_with_sub_router() {
    // 测试带子路由的模块路由
    let route = ModuleRoute::parse("/modules/auth?version=v1&debug=true").unwrap();
    assert_eq!(route.name, "auth");
    assert_eq!(route.options.version, Some("v1".to_string()));
    assert_eq!(route.options.debug, Some(true));
    assert_eq!(route.sub_router, None); // NoSubRouter 应该是 None

    // 测试格式化
    let formatted = route.format();
    assert!(formatted.contains("/modules/auth"));
    assert!(formatted.contains("version=v1"));
    assert!(formatted.contains("debug=true"));

    // 测试模式
    assert_eq!(ModuleRoute::pattern(), "/modules/:name");
  }

  #[test]
  fn test_empty_query_params() {
    // 测试没有查询参数的情况
    let route = SearchRoute::parse("/search/books").unwrap();
    assert_eq!(route.category, "books");
    assert_eq!(route.params.q, None);
    assert_eq!(route.params.page, None);
    assert_eq!(route.params.limit, None);
    assert!(route.params.tags.is_empty());

    // 测试格式化（应该只包含路径部分）
    let formatted = route.format();
    assert_eq!(formatted, "/search/books");
  }

  #[test]
  fn test_automatic_prefix_extraction() {
    // 测试自动前缀提取功能
    // Router 派生宏应该能够自动从 #[router(pattern = "...")] 提取前缀

    // 验证各种路由的模式提取
    assert_eq!(UserRoute::pattern(), "/users/:id");
    assert_eq!(BlogRoute::pattern(), "/blog/:category/:slug");
    assert_eq!(ApiRoute::pattern(), "/api/:version/users/:user_id/posts/:post_id");
    assert_eq!(SearchRoute::pattern(), "/search/:category");
    assert_eq!(ModuleRoute::pattern(), "/modules/:name");
  }

  #[test]
  fn test_complex_query_parameter_types() {
    // 测试复杂查询参数类型的处理
    let route = ModuleRoute::parse("/modules/payment?version=v2.1&debug=false").unwrap();
    assert_eq!(route.name, "payment");
    assert_eq!(route.options.version, Some("v2.1".to_string()));
    assert_eq!(route.options.debug, Some(false));

    // 测试布尔值的不同表示
    let route_true = ModuleRoute::parse("/modules/test?debug=1").unwrap();
    assert_eq!(route_true.options.debug, Some(true));

    let route_false = ModuleRoute::parse("/modules/test?debug=0").unwrap();
    assert_eq!(route_false.options.debug, Some(false));
  }

  #[test]
  fn test_url_encoding_in_params() {
    // 测试 URL 编码的参数
    let route = SearchRoute::parse("/search/tech?q=rust%20programming&tags=web%20dev").unwrap();
    assert_eq!(route.category, "tech");
    assert_eq!(route.params.q, Some("rust programming".to_string()));
    assert_eq!(route.params.tags, vec!["web dev".to_string()]);
  }
}
