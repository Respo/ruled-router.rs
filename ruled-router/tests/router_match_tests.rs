//! RouterMatch 派生宏测试
//!
//! 测试 RouterMatch 枚举的各种功能，特别是 format 方法

use ruled_router::prelude::*;
use ruled_router::RouteMatcher;
use ruled_router_derive::{Query, RouterData, RouterMatch};

// ===== 测试用的查询参数 =====

#[derive(Debug, Clone, PartialEq, Default, Query)]
struct TestQuery {
  format: Option<String>,
  debug: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Default, Query)]
struct UserQuery {
  page: Option<u32>,
  limit: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Default, Query)]
struct ProductQuery {
  category: Option<String>,
  sort: Option<String>,
  min_price: Option<f64>,
}

// ===== 测试用的路由结构 =====

/// 简单的用户路由
#[derive(Debug, Clone, PartialEq, RouterData)]
#[router(pattern = "/users/:id")]
struct UserRoute {
  id: u32,
  #[query]
  query: UserQuery,
}

/// 产品路由
#[derive(Debug, Clone, PartialEq, RouterData)]
#[router(pattern = "/products/:category/:id")]
struct ProductRoute {
  category: String,
  id: u32,
  #[query]
  query: ProductQuery,
}

/// 设置路由
#[derive(Debug, Clone, PartialEq, RouterData)]
#[router(pattern = "/settings")]
struct SettingsRoute {
  #[query]
  query: TestQuery,
}

/// API 路由
#[derive(Debug, Clone, PartialEq, RouterData)]
#[router(pattern = "/api/:version/:endpoint")]
struct ApiRoute {
  version: String,
  endpoint: String,
  #[query]
  query: TestQuery,
}

// ===== RouterMatch 枚举定义 =====

/// 简单的应用路由匹配器
#[derive(Debug, Clone, PartialEq, RouterMatch)]
enum AppRouterMatch {
  User(UserRoute),
  Product(ProductRoute),
  Settings(SettingsRoute),
  Api(ApiRoute),
}

/// 嵌套的子路由匹配器
#[derive(Debug, Clone, PartialEq, RouterMatch)]
enum SubRouterMatch {
  User(UserRoute),
  Settings(SettingsRoute),
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_router_match_format_user_route() {
    // 测试用户路由的格式化
    let user_route = UserRoute {
      id: 123,
      query: UserQuery {
        page: Some(1),
        limit: Some(20),
      },
    };

    let app_match = AppRouterMatch::User(user_route);
    let formatted = app_match.format();

    assert!(formatted.contains("/users/123"));
    assert!(formatted.contains("page=1"));
    assert!(formatted.contains("limit=20"));
  }

  #[test]
  fn test_router_match_format_product_route() {
    // 测试产品路由的格式化
    let product_route = ProductRoute {
      category: "electronics".to_string(),
      id: 456,
      query: ProductQuery {
        category: Some("laptop".to_string()),
        sort: Some("price".to_string()),
        min_price: Some(100.0),
      },
    };

    let app_match = AppRouterMatch::Product(product_route);
    let formatted = app_match.format();

    assert!(formatted.contains("/products/electronics/456"));
    assert!(formatted.contains("category=laptop"));
    assert!(formatted.contains("sort=price"));
    assert!(formatted.contains("min_price=100"));
  }

  #[test]
  fn test_router_match_format_settings_route() {
    // 测试设置路由的格式化
    let settings_route = SettingsRoute {
      query: TestQuery {
        format: Some("json".to_string()),
        debug: Some(true),
      },
    };

    let app_match = AppRouterMatch::Settings(settings_route);
    let formatted = app_match.format();

    assert!(formatted.contains("/settings"));
    assert!(formatted.contains("format=json"));
    assert!(formatted.contains("debug=true"));
  }

  #[test]
  fn test_router_match_format_api_route() {
    // 测试 API 路由的格式化
    let api_route = ApiRoute {
      version: "v1".to_string(),
      endpoint: "users".to_string(),
      query: TestQuery {
        format: Some("xml".to_string()),
        debug: Some(false),
      },
    };

    let app_match = AppRouterMatch::Api(api_route);
    let formatted = app_match.format();

    assert!(formatted.contains("/api/v1/users"));
    assert!(formatted.contains("format=xml"));
    assert!(formatted.contains("debug=false"));
  }

  #[test]
  fn test_router_match_format_empty_query() {
    // 测试空查询参数的格式化
    let user_route = UserRoute {
      id: 789,
      query: UserQuery::default(), // 空查询参数
    };

    let app_match = AppRouterMatch::User(user_route);
    let formatted = app_match.format();

    // 应该只包含路径部分，没有查询参数
    assert_eq!(formatted, "/users/789");
  }

  #[test]
  fn test_router_match_format_partial_query() {
    // 测试部分查询参数的格式化
    let user_route = UserRoute {
      id: 999,
      query: UserQuery {
        page: Some(5),
        limit: None, // 只有部分查询参数
      },
    };

    let app_match = AppRouterMatch::User(user_route);
    let formatted = app_match.format();

    assert!(formatted.contains("/users/999"));
    assert!(formatted.contains("page=5"));
    assert!(!formatted.contains("limit=")); // 不应该包含空的 limit 参数
  }

  #[test]
  fn test_router_match_format_special_characters() {
    // 测试特殊字符的格式化
    let product_route = ProductRoute {
      category: "home & garden".to_string(), // 包含特殊字符
      id: 123,
      query: ProductQuery {
        category: Some("outdoor furniture".to_string()), // 包含空格
        sort: Some("price-desc".to_string()),
        min_price: None,
      },
    };

    let app_match = AppRouterMatch::Product(product_route);
    let formatted = app_match.format();

    // 验证 URL 编码
    assert!(formatted.contains("/products/home%20%26%20garden/123") || formatted.contains("/products/home%20&%20garden/123"));
    assert!(formatted.contains("category=outdoor%20furniture") || formatted.contains("category=outdoor+furniture"));
    assert!(formatted.contains("sort=price-desc"));
  }

  #[test]
  fn test_sub_router_match_format() {
    // 测试子路由匹配器的格式化
    let user_route = UserRoute {
      id: 456,
      query: UserQuery {
        page: Some(2),
        limit: Some(50),
      },
    };

    let sub_match = SubRouterMatch::User(user_route);
    let formatted = sub_match.format();

    assert!(formatted.contains("/users/456"));
    assert!(formatted.contains("page=2"));
    assert!(formatted.contains("limit=50"));
  }

  #[test]
  fn test_router_match_format_roundtrip() {
    // 测试格式化和解析的往返一致性
    let original_route = UserRoute {
      id: 777,
      query: UserQuery {
        page: Some(3),
        limit: Some(25),
      },
    };

    let app_match = AppRouterMatch::User(original_route.clone());
    let formatted = app_match.format();

    // 尝试解析格式化后的字符串
    let parsed_route = UserRoute::parse(&formatted).unwrap();

    assert_eq!(parsed_route.id, original_route.id);
    assert_eq!(parsed_route.query.page, original_route.query.page);
    assert_eq!(parsed_route.query.limit, original_route.query.limit);
  }

  #[test]
  fn test_router_match_patterns() {
    // 测试 RouterMatch 的 patterns 方法
    let patterns = AppRouterMatch::patterns();

    assert!(patterns.contains(&"/users/:id"));
    assert!(patterns.contains(&"/products/:category/:id"));
    assert!(patterns.contains(&"/settings"));
    assert!(patterns.contains(&"/api/:version/:endpoint"));
    assert_eq!(patterns.len(), 4);
  }

  #[test]
  fn test_router_match_format_consistency() {
    // 测试不同路由类型的格式化一致性
    let routes = vec![
      AppRouterMatch::User(UserRoute {
        id: 1,
        query: UserQuery::default(),
      }),
      AppRouterMatch::Settings(SettingsRoute {
        query: TestQuery::default(),
      }),
      AppRouterMatch::Api(ApiRoute {
        version: "v2".to_string(),
        endpoint: "posts".to_string(),
        query: TestQuery::default(),
      }),
    ];

    for route in routes {
      let formatted = route.format();

      // 所有格式化的路由都应该以 '/' 开头
      assert!(formatted.starts_with('/'), "Route should start with '/': {formatted}");

      // 不应该包含连续的斜杠
      assert!(!formatted.contains("//"), "Route should not contain '//': {formatted}");

      // 如果有查询参数，应该只有一个 '?'
      let question_marks = formatted.matches('?').count();
      assert!(question_marks <= 1, "Route should have at most one '?': {formatted}");
    }
  }
}
