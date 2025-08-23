//! 简化的路由使用示例
//! 展示如何使用 RouterMatch derive 宏来减少模板代码

use ruled_router::prelude::*;
use ruled_router::RouteMatcher;
use ruled_router_derive::{Query, Router, RouterMatch};

// ===== 查询参数定义 =====

#[derive(Debug, Clone, PartialEq, Default, Query)]
struct UserQuery {
  #[query(name = "include")]
  include_fields: Vec<String>,
  #[query(name = "format")]
  format: Option<String>,
}

// ===== 路由定义 =====

#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/users/:id")]
struct UserDetailRoute {
  id: u32,
  #[query]
  query: UserQuery,
}

#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/users/:user_id/profile")]
struct UserProfileRoute {
  user_id: u32,
  #[query]
  query: UserQuery,
}

#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/users/:user_id/settings")]
struct UserSettingsRoute {
  user_id: u32,
}

// ===== RouteMatcher 定义 (使用 derive 宏) =====

/// 用户路由匹配 - 使用 derive 宏自动生成 RouteMatcher trait 实现
#[derive(Debug, Clone, PartialEq, RouterMatch)]
enum UserRouterMatch {
  Detail(UserDetailRoute),
  Profile(UserProfileRoute),
  Settings(UserSettingsRoute),
}

fn main() {
  println!("=== 简化的路由使用示例 ===");
  println!("使用 RouterMatch derive 宏减少模板代码\n");

  // 测试 URL 列表
  let test_urls = vec![
    "/users/123?include=profile&format=json",
    "/users/456/profile?include=email",
    "/users/789/settings",
  ];

  for url in test_urls {
    println!("解析 URL: {url}");

    match UserRouterMatch::try_parse(url) {
      Ok(route_match) => {
        println!("  匹配成功: {route_match:?}");

        // 测试格式化
        let formatted = route_match.format();
        println!("  格式化结果: {formatted}");
      }
      Err(e) => {
        println!("  解析失败: {e}");
      }
    }
    println!();
  }

  // 展示路由模式
  println!("=== 支持的路由模式 ===");
  println!("用户路由模式: {:?}", UserRouterMatch::patterns());
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_derive_macro_functionality() {
    // 测试 derive 宏生成的代码是否正确工作

    // 测试用户路由解析
    let user_detail = UserRouterMatch::try_parse("/users/123");
    assert!(user_detail.is_ok());

    // 测试格式化
    if let Ok(route) = user_detail {
      let formatted = route.format();
      assert!(formatted.contains("/users/123"));
    }

    // 测试模式获取
    let patterns = UserRouterMatch::patterns();
    assert!(!patterns.is_empty());
    assert!(patterns.contains(&"/users/:id"));
    assert!(patterns.contains(&"/users/:user_id/profile"));
    assert!(patterns.contains(&"/users/:user_id/settings"));
  }

  #[test]
  fn test_query_parsing() {
    // 测试查询参数解析
    let route = UserRouterMatch::try_parse("/users/123?include=profile,email&format=json");
    assert!(route.is_ok());

    if let Ok(UserRouterMatch::Detail(detail)) = route {
      assert_eq!(detail.id, 123);
      assert_eq!(detail.query.include_fields, vec!["profile", "email"]);
      assert_eq!(detail.query.format, Some("json".to_string()));
    }
  }
}
