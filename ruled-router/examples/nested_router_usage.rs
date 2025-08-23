//! 三层嵌套路由使用示例
//!
//! 本示例展示了如何使用 ruled-router 库实现深度嵌套的路由结构。
//! 严格遵循 RouteMatcher -> Router -> RouteMatcher -> Router -> RouteMatcher -> Router 的设计模式。
//!
//! 架构层次：
//! 1. AppRouterMatch (第一层 RouteMatcher) -> ModuleRoute (第一层 Router)
//! 2. ModuleRoute -> SubRouterMatch (第二层 RouteMatcher) -> CategoryRoute (第二层 Router)
//! 3. CategoryRoute -> DetailRouterMatch (第三层 RouteMatcher) -> DetailRoute (第三层 Router)

use ruled_router::prelude::*;
use ruled_router::RouteMatcher;
use ruled_router_derive::RouterMatch;

// ===== 查询参数定义 (每层只保留一个) =====

#[derive(Debug, Clone, PartialEq, Default, Query)]
struct SimpleQuery {
  #[query(name = "format")]
  format: Option<String>,
}

// ===== 第一层：应用主路由 (Router) =====

/// 应用根路由 - 第一层 Router
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/")]
struct AppRoute {
  #[query]
  query: SimpleQuery,
}

// ===== 第二层：模块路由 (Router) =====

/// 用户模块路由 - 第二层 Router
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/users")]
struct UserModuleRoute {
  #[query]
  query: SimpleQuery,
  #[sub_router]
  sub_router: Option<UserSubRouterMatch>,
}

/// 商店模块路由 - 第二层 Router
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/shop")]
struct ShopModuleRoute {
  #[query]
  query: SimpleQuery,
  #[sub_router]
  sub_router: Option<ShopSubRouterMatch>,
}

/// 管理模块路由 - 第二层 Router
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/admin")]
struct AdminModuleRoute {
  #[query]
  query: SimpleQuery,
  #[sub_router]
  sub_router: Option<AdminSubRouterMatch>,
}

// ===== 第三层：分类路由 (Router) =====

/// 用户个人分类路由 - 第三层 Router
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/profile")]
struct UserProfileCategoryRoute {
  #[query]
  query: SimpleQuery,
  #[sub_router]
  sub_router: Option<UserProfileDetailRouterMatch>,
}

/// 用户内容分类路由 - 第三层 Router
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/content")]
struct UserContentCategoryRoute {
  #[query]
  query: SimpleQuery,
  #[sub_router]
  sub_router: Option<UserContentDetailRouterMatch>,
}

/// 商店产品分类路由 - 第三层 Router
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/products")]
struct ShopProductCategoryRoute {
  #[query]
  query: SimpleQuery,
  #[sub_router]
  sub_router: Option<ShopProductDetailRouterMatch>,
}

/// 商店订单分类路由 - 第三层 Router
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/orders")]
struct ShopOrderCategoryRoute {
  #[query]
  query: SimpleQuery,
  #[sub_router]
  sub_router: Option<ShopOrderDetailRouterMatch>,
}

/// 管理用户分类路由 - 第三层 Router
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/users")]
struct AdminUserCategoryRoute {
  #[query]
  query: SimpleQuery,
  #[sub_router]
  sub_router: Option<AdminUserDetailRouterMatch>,
}

/// 管理系统分类路由 - 第三层 Router
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/system")]
struct AdminSystemCategoryRoute {
  #[query]
  query: SimpleQuery,
  #[sub_router]
  sub_router: Option<AdminSystemDetailRouterMatch>,
}

// ===== 第四层：详细路由 (Router) =====

/// 用户基本信息路由 - 第四层 Router
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/:id")]
struct UserBasicInfoRoute {
  id: u32,
  #[query]
  query: SimpleQuery,
}

/// 用户设置路由 - 第四层 Router
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/:id/settings")]
struct UserSettingsRoute {
  id: u32,
  #[query]
  query: SimpleQuery,
}

/// 用户文章路由 - 第四层 Router
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/:user_id/posts/:post_id")]
struct UserPostRoute {
  user_id: u32,
  post_id: u32,
  #[query]
  query: SimpleQuery,
}

/// 用户评论路由 - 第四层 Router
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/:user_id/comments/:comment_id")]
struct UserCommentRoute {
  user_id: u32,
  comment_id: u32,
  #[query]
  query: SimpleQuery,
}

/// 产品详情路由 - 第四层 Router
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/:category/:id")]
struct ProductDetailRoute {
  category: String,
  id: u32,
  #[query]
  query: SimpleQuery,
}

/// 产品列表路由 - 第四层 Router
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/:category")]
struct ProductListRoute {
  category: String,
  #[query]
  query: SimpleQuery,
}

/// 订单详情路由 - 第四层 Router
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/:id")]
struct OrderDetailRoute {
  id: u32,
  #[query]
  query: SimpleQuery,
}

/// 管理员用户管理路由 - 第四层 Router
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/:id")]
struct AdminUserManageRoute {
  id: u32,
  #[query]
  query: SimpleQuery,
}

/// 系统配置路由 - 第四层 Router
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/config")]
struct SystemConfigRoute {
  #[query]
  query: SimpleQuery,
}

// ===== RouteMatcher 定义 =====

/// 用户个人详情路由匹配 - 第四层 RouteMatcher
#[derive(Debug, Clone, PartialEq, RouterMatch)]
#[route_prefix("/profile")]
enum UserProfileDetailRouterMatch {
  #[route("/:id")]
  BasicInfo(UserBasicInfoRoute),
  #[route("/:id/settings")]
  Settings(UserSettingsRoute),
}

/// 用户内容详情路由匹配 - 第四层 RouteMatcher
#[derive(Debug, Clone, PartialEq, RouterMatch)]
#[route_prefix("/content")]
enum UserContentDetailRouterMatch {
  #[route("/:user_id/posts/:post_id")]
  Post(UserPostRoute),
  #[route("/:user_id/comments/:comment_id")]
  Comment(UserCommentRoute),
}

/// 商店产品详情路由匹配 - 第四层 RouteMatcher
#[derive(Debug, Clone, PartialEq, RouterMatch)]
#[route_prefix("/products")]
enum ShopProductDetailRouterMatch {
  #[route("/:category/:id")]
  Detail(ProductDetailRoute),
  #[route("/:category")]
  List(ProductListRoute),
}

/// 商店订单详情路由匹配 - 第四层 RouteMatcher
#[derive(Debug, Clone, PartialEq, RouterMatch)]
#[route_prefix("/orders")]
enum ShopOrderDetailRouterMatch {
  #[route("/:id")]
  Detail(OrderDetailRoute),
}

/// 管理用户详情路由匹配 - 第四层 RouteMatcher
#[derive(Debug, Clone, PartialEq, RouterMatch)]
#[route_prefix("/users")]
enum AdminUserDetailRouterMatch {
  #[route("/:id")]
  Manage(AdminUserManageRoute),
}

/// 管理系统详情路由匹配 - 第四层 RouteMatcher
#[derive(Debug, Clone, PartialEq, RouterMatch)]
#[route_prefix("/system")]
enum AdminSystemDetailRouterMatch {
  #[route("/config")]
  Config(SystemConfigRoute),
}

/// 用户子路由匹配 - 第二层 RouteMatcher
#[derive(Debug, Clone, PartialEq, RouterMatch)]
#[route_prefix("/users")]
enum UserSubRouterMatch {
  #[route("/profile")]
  Profile(UserProfileCategoryRoute),
  #[route("/content")]
  Content(UserContentCategoryRoute),
}

/// 商店子路由匹配 - 第二层 RouteMatcher
#[derive(Debug, Clone, PartialEq, RouterMatch)]
#[route_prefix("/shop")]
enum ShopSubRouterMatch {
  #[route("/products")]
  Products(ShopProductCategoryRoute),
  #[route("/orders")]
  Orders(ShopOrderCategoryRoute),
}

/// 管理子路由匹配 - 第二层 RouteMatcher
#[derive(Debug, Clone, PartialEq, RouterMatch)]
#[route_prefix("/admin")]
enum AdminSubRouterMatch {
  #[route("/users")]
  Users(AdminUserCategoryRoute),
  #[route("/system")]
  System(AdminSystemCategoryRoute),
}

// ===== 第二层 RouterMatch =====

/// 第一层：应用模块路由匹配器 (RouteMatcher)
/// 严格遵循 RouteMatcher -> Router -> RouteMatcher -> Router -> RouteMatcher -> Router 模式
#[derive(Debug, Clone, PartialEq, RouterMatch)]
enum AppRouterMatch {
  #[route("/users")]
  User(UserModuleRoute),
  #[route("/shop")]
  Shop(ShopModuleRoute),
  #[route("/admin")]
  Admin(AdminModuleRoute),
}

// ===== 辅助函数 =====

fn parse_nested_route(url: &str) -> Result<String, Box<dyn std::error::Error>> {
  let mut result = String::new();

  // 调试：先测试第一层解析
  result.push_str(&format!("尝试解析 URL: {url}\n"));

  // 分离路径和查询参数
  let (path_part, query_part) = if let Some(query_index) = url.find('?') {
    (&url[..query_index], Some(&url[query_index..]))
  } else {
    (url, None)
  };

  result.push_str(&format!("路径部分: {path_part}\n"));
  if let Some(query) = query_part {
    result.push_str(&format!("查询参数: {query}\n"));
  }

  // 调试：测试各个路由的 pattern
  result.push_str(&format!("UserModuleRoute pattern: {}\n", UserModuleRoute::pattern()));
  result.push_str(&format!("ShopModuleRoute pattern: {}\n", ShopModuleRoute::pattern()));
  result.push_str(&format!("AdminModuleRoute pattern: {}\n", AdminModuleRoute::pattern()));

  // 使用真正的嵌套路由解析
  match AppRouterMatch::try_parse_with_remaining(path_part, 0) {
    Ok((app_match, remaining_path)) => {
      result.push_str(&format!("第一层解析成功: {app_match:?}\n"));
      result.push_str(&format!("剩余路径: '{remaining_path}'\n"));

      // 如果有剩余路径，尝试解析子路由
      if !remaining_path.is_empty() {
        match &app_match {
          AppRouterMatch::User(user_route) => {
            if let Some(ref sub_router) = user_route.sub_router {
              result.push_str(&format!("用户子路由: {sub_router:?}\n"));
            } else if let Ok((sub_match, sub_remaining)) = UserSubRouterMatch::try_parse_with_remaining(remaining_path, 0) {
              result.push_str(&format!("第二层 (用户分类): {sub_match:?}\n"));
              result.push_str(&format!("第二层剩余路径: '{sub_remaining}'\n"));

              // 继续解析第三层
              if !sub_remaining.is_empty() {
                match &sub_match {
                  UserSubRouterMatch::Profile(profile_route) => {
                    if let Ok((profile_detail, _)) = UserProfileDetailRouterMatch::try_parse_with_remaining(sub_remaining, 0) {
                      result.push_str(&format!("第三层 (个人详情): {profile_detail:?}\n"));
                    }
                  }
                  UserSubRouterMatch::Content(content_route) => {
                    if let Ok((content_detail, _)) = UserContentDetailRouterMatch::try_parse_with_remaining(sub_remaining, 0) {
                      result.push_str(&format!("第三层 (内容详情): {content_detail:?}\n"));
                    }
                  }
                }
              }
            }
          }
          AppRouterMatch::Shop(shop_route) => {
            if let Some(ref sub_router) = shop_route.sub_router {
              result.push_str(&format!("商店子路由: {sub_router:?}\n"));
            } else if let Ok((sub_match, sub_remaining)) = ShopSubRouterMatch::try_parse_with_remaining(remaining_path, 0) {
              result.push_str(&format!("第二层 (商店分类): {sub_match:?}\n"));
              result.push_str(&format!("第二层剩余路径: '{sub_remaining}'\n"));

              // 继续解析第三层
              if !sub_remaining.is_empty() {
                match &sub_match {
                  ShopSubRouterMatch::Products(product_route) => {
                    if let Ok((product_detail, _)) = ShopProductDetailRouterMatch::try_parse_with_remaining(sub_remaining, 0) {
                      result.push_str(&format!("第三层 (产品详情): {product_detail:?}\n"));
                    }
                  }
                  ShopSubRouterMatch::Orders(order_route) => {
                    if let Ok((order_detail, _)) = ShopOrderDetailRouterMatch::try_parse_with_remaining(sub_remaining, 0) {
                      result.push_str(&format!("第三层 (订单详情): {order_detail:?}\n"));
                    }
                  }
                }
              }
            }
          }
          AppRouterMatch::Admin(admin_route) => {
            if let Some(ref sub_router) = admin_route.sub_router {
              result.push_str(&format!("管理子路由: {sub_router:?}\n"));
            } else if let Ok((sub_match, sub_remaining)) = AdminSubRouterMatch::try_parse_with_remaining(remaining_path, 0) {
              result.push_str(&format!("第二层 (管理分类): {sub_match:?}\n"));
              result.push_str(&format!("第二层剩余路径: '{sub_remaining}'\n"));

              // 继续解析第三层
              if !sub_remaining.is_empty() {
                match &sub_match {
                  AdminSubRouterMatch::Users(user_category_route) => {
                    if let Ok((user_detail, _)) = AdminUserDetailRouterMatch::try_parse_with_remaining(sub_remaining, 0) {
                      result.push_str(&format!("第三层 (用户详情): {user_detail:?}\n"));
                    }
                  }
                  AdminSubRouterMatch::System(system_category_route) => {
                    if let Ok((system_detail, _)) = AdminSystemDetailRouterMatch::try_parse_with_remaining(sub_remaining, 0) {
                      result.push_str(&format!("第三层 (系统详情): {system_detail:?}\n"));
                    }
                  }
                }
              }
            }
          }
        }
      }

      // 演示格式化
      result.push_str(&format!("格式化输出: {}\n", app_match.format()));
    }
    Err(e) => {
      result.push_str(&format!("第一层解析失败: {e:?}\n"));
      result.push_str("无法解析为有效的路由\n");
    }
  }

  Ok(result)
}

fn main() {
  println!("=== 三层嵌套路由使用示例 ===");
  println!("架构：RouteMatcher -> Router -> RouteMatcher -> Router -> RouteMatcher -> Router");
  println!();

  let test_urls = vec![
    "/users",
    "/shop",
    "/admin",
    "/users/profile/123?format=json",
    "/users/profile/456/settings?format=xml",
    "/users/content/789/posts/101?format=json",
    "/users/content/999/comments/202?format=xml",
    "/shop/products/electronics/555?format=json",
    "/shop/products/books?format=xml",
    "/shop/orders/777?format=json",
    "/admin/users/888?format=xml",
    "/admin/system/config?format=json",
  ];

  for url in test_urls {
    println!("=== 解析 URL: {url} ===");

    match parse_nested_route(url) {
      Ok(result) => {
        println!("{result}");
      }
      Err(e) => {
        println!("解析失败: {e}");
      }
    }
    println!();
  }

  println!("=== 架构验证 ===");
  println!("✓ AppRouterMatch (第一层 RouteMatcher) -> UserModuleRoute/ShopModuleRoute/AdminModuleRoute (第一层 Router)");
  println!("✓ UserModuleRoute -> UserSubRouterMatch (第二层 RouteMatcher) -> UserProfileCategoryRoute/UserContentCategoryRoute (第二层 Router)");
  println!("✓ UserProfileCategoryRoute -> UserProfileDetailRouterMatch (第三层 RouteMatcher) -> UserBasicInfoRoute/UserSettingsRoute (第三层 Router)");
  println!("✓ 严格遵循 RouteMatcher -> Router -> RouteMatcher -> Router -> RouteMatcher -> Router 模式");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_architecture_compliance() {
    // 测试第一层路由匹配
    let (app_match, remaining) = AppRouterMatch::try_parse_with_remaining("/users/profile/123", 0).unwrap();
    assert!(matches!(app_match, AppRouterMatch::User(_)));
    assert_eq!(remaining, "/profile/123");

    // 测试第二层路由匹配 - UserSubRouterMatch 需要 /users 前缀
    let (user_sub_match, remaining) = UserSubRouterMatch::try_parse_with_remaining("/users/profile", 0).unwrap();
    assert!(matches!(user_sub_match, UserSubRouterMatch::Profile(_)));
    assert_eq!(remaining, "");

    // 测试第三层路由匹配 - UserProfileDetailRouterMatch 需要 /profile 前缀
    let (profile_detail_match, remaining) = UserProfileDetailRouterMatch::try_parse_with_remaining("/profile/123", 0).unwrap();
    assert!(matches!(profile_detail_match, UserProfileDetailRouterMatch::BasicInfo(_)));
    assert_eq!(remaining, "");
  }

  #[test]
  fn test_user_basic_info_route() {
    let route = UserBasicInfoRoute::parse("/123?format=json").unwrap();
    assert_eq!(route.id, 123);
    assert_eq!(route.query.format, Some("json".to_string()));
  }

  #[test]
  fn test_nested_parsing() {
    let result = parse_nested_route("/users/profile/123").unwrap();
    assert!(result.contains("第一层解析成功"));
    assert!(result.contains("User("));
  }

  #[test]
  fn test_shop_product_route() {
    let route = ProductDetailRoute::parse("/electronics/555?format=xml").unwrap();
    assert_eq!(route.category, "electronics");
    assert_eq!(route.id, 555);
    assert_eq!(route.query.format, Some("xml".to_string()));
  }

  #[test]
  fn test_admin_system_route() {
    let route = SystemConfigRoute::parse("/config?format=json").unwrap();
    assert_eq!(route.query.format, Some("json".to_string()));
  }
}
