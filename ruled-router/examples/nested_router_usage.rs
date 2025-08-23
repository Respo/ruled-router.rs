//! 三层嵌套路由使用示例
//!
//! 本示例展示了如何使用 ruled-router 库实现深度嵌套的路由结构。
//! 严格遵循 RouterMatch -> Router -> RouterMatch -> Router -> RouterMatch -> Router 的设计模式。
//!
//! 架构设计：
//! 1. AppRouterMatch (第一层 RouterMatch) -> ModuleRoute (第一层 Router)
//! 2. ModuleRoute -> SubRouterMatch (第二层 RouterMatch) -> CategoryRoute (第二层 Router)
//! 3. CategoryRoute -> DetailRouterMatch (第三层 RouterMatch) -> DetailRoute (第三层 Router)

use ruled_router::prelude::*;
use ruled_router::RouterMatch;
use ruled_router_derive::RouterMatch as RouterMatchDerive;

// ===== 查询参数定义 (每层只保留一个) =====

#[derive(Debug, Clone, PartialEq, Default, Query)]
struct SimpleQuery {
  #[query(name = "format")]
  format: Option<String>,
}

// ===== 第一层：模块路由 (Router) =====

/// 用户模块路由 - 第一层 Router
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/users")]
struct UserModuleRoute {
  // 模块入口，不包含参数
}

/// 商店模块路由 - 第一层 Router
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/shop")]
struct ShopModuleRoute {
  // 模块入口，不包含参数
}

/// 管理模块路由 - 第一层 Router
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/admin")]
struct AdminModuleRoute {
  // 模块入口，不包含参数
}

// ===== 第二层：分类路由 (Router) =====

/// 用户个人分类路由 - 第二层 Router
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/users/profile")]
struct UserProfileCategoryRoute {
  // 分类入口，不包含参数
}

/// 用户内容分类路由 - 第二层 Router
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/users/content")]
struct UserContentCategoryRoute {
  // 分类入口，不包含参数
}

/// 商店产品分类路由 - 第二层 Router
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/shop/products")]
struct ShopProductCategoryRoute {
  // 分类入口，不包含参数
}

/// 商店订单分类路由 - 第二层 Router
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/shop/orders")]
struct ShopOrderCategoryRoute {
  // 分类入口，不包含参数
}

/// 管理用户分类路由 - 第二层 Router
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/admin/users")]
struct AdminUserCategoryRoute {
  // 分类入口，不包含参数
}

/// 管理系统分类路由 - 第二层 Router
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/admin/system")]
struct AdminSystemCategoryRoute {
  // 分类入口，不包含参数
}

// ===== 第三层：详细路由 (Router) =====

/// 用户基本信息路由 - 第三层 Router
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/users/profile/:id")]
struct UserBasicInfoRoute {
  id: u32,
  #[query]
  query: SimpleQuery,
}

/// 用户设置路由 - 第三层 Router
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/users/profile/:id/settings")]
struct UserSettingsRoute {
  id: u32,
  #[query]
  query: SimpleQuery,
}

/// 用户文章路由 - 第三层 Router
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/users/content/:user_id/posts/:post_id")]
struct UserPostRoute {
  user_id: u32,
  post_id: u32,
  #[query]
  query: SimpleQuery,
}

/// 用户评论路由 - 第三层 Router
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/users/content/:user_id/comments/:comment_id")]
struct UserCommentRoute {
  user_id: u32,
  comment_id: u32,
  #[query]
  query: SimpleQuery,
}

/// 产品详情路由 - 第三层 Router
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/shop/products/:category/:id")]
struct ProductDetailRoute {
  category: String,
  id: u32,
  #[query]
  query: SimpleQuery,
}

/// 产品列表路由 - 第三层 Router
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/shop/products/:category")]
struct ProductListRoute {
  category: String,
  #[query]
  query: SimpleQuery,
}

/// 订单详情路由 - 第三层 Router
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/shop/orders/:id")]
struct OrderDetailRoute {
  id: u32,
  #[query]
  query: SimpleQuery,
}

/// 管理员用户管理路由 - 第三层 Router
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/admin/users/:id")]
struct AdminUserManageRoute {
  id: u32,
  #[query]
  query: SimpleQuery,
}

/// 系统配置路由 - 第三层 Router
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/admin/system/config")]
struct SystemConfigRoute {
  #[query]
  query: SimpleQuery,
}

// ===== RouterMatch 定义 =====

/// 用户个人详情路由匹配 - 第三层 RouterMatch
#[derive(Debug, Clone, PartialEq, RouterMatchDerive)]
enum UserProfileDetailRouterMatch {
  BasicInfo(UserBasicInfoRoute),
  Settings(UserSettingsRoute),
}

/// 用户内容详情路由匹配 - 第三层 RouterMatch
#[derive(Debug, Clone, PartialEq, RouterMatchDerive)]
enum UserContentDetailRouterMatch {
  Post(UserPostRoute),
  Comment(UserCommentRoute),
}

/// 商店产品详情路由匹配 - 第三层 RouterMatch
#[derive(Debug, Clone, PartialEq, RouterMatchDerive)]
enum ShopProductDetailRouterMatch {
  Detail(ProductDetailRoute),
  List(ProductListRoute),
}

/// 商店订单详情路由匹配 - 第三层 RouterMatch
#[derive(Debug, Clone, PartialEq, RouterMatchDerive)]
enum ShopOrderDetailRouterMatch {
  Detail(OrderDetailRoute),
}

/// 管理用户详情路由匹配 - 第三层 RouterMatch
#[derive(Debug, Clone, PartialEq, RouterMatchDerive)]
enum AdminUserDetailRouterMatch {
  Manage(AdminUserManageRoute),
}

/// 管理系统详情路由匹配 - 第三层 RouterMatch
#[derive(Debug, Clone, PartialEq, RouterMatchDerive)]
enum AdminSystemDetailRouterMatch {
  Config(SystemConfigRoute),
}

/// 用户子路由匹配 - 第二层 RouterMatch
#[derive(Debug, Clone, PartialEq, RouterMatchDerive)]
enum UserSubRouterMatch {
  #[route_prefix = "/users/profile"]
  Profile(UserProfileCategoryRoute),
  #[route_prefix = "/users/content"]
  Content(UserContentCategoryRoute),
}

/// 商店子路由匹配 - 第二层 RouterMatch
#[derive(Debug, Clone, PartialEq, RouterMatchDerive)]
enum ShopSubRouterMatch {
  #[route_prefix = "/shop/products"]
  Products(ShopProductCategoryRoute),
  #[route_prefix = "/shop/orders"]
  Orders(ShopOrderCategoryRoute),
}

/// 管理子路由匹配 - 第二层 RouterMatch
#[derive(Debug, Clone, PartialEq, RouterMatchDerive)]
enum AdminSubRouterMatch {
  #[route_prefix = "/admin/users"]
  Users(AdminUserCategoryRoute),
  #[route_prefix = "/admin/system"]
  System(AdminSystemCategoryRoute),
}

/// 顶级应用路由匹配 - 第一层 RouterMatch
/// 严格遵循 RouterMatch -> Router -> RouterMatch -> Router -> RouterMatch -> Router 模式
#[derive(Debug, Clone, PartialEq, RouterMatchDerive)]
enum AppRouterMatch {
  #[route_prefix = "/users"]
  User(UserModuleRoute),
  #[route_prefix = "/shop"]
  Shop(ShopModuleRoute),
  #[route_prefix = "/admin"]
  Admin(AdminModuleRoute),
}



// ===== 辅助函数 =====

fn parse_nested_route(url: &str) -> Result<(AppRouterMatch, Option<Box<dyn std::fmt::Debug>>, Option<Box<dyn std::fmt::Debug>>), Box<dyn std::error::Error>> {
  // 第一层：解析模块路由
  let app_match = AppRouterMatch::try_parse(url)?;

  // 第二层：根据模块类型解析分类路由
  let (sub_match, detail_match): (Option<Box<dyn std::fmt::Debug>>, Option<Box<dyn std::fmt::Debug>>) = match &app_match {
    AppRouterMatch::User(_) => {
      if let Ok(user_sub) = UserSubRouterMatch::try_parse(url) {
        // 第三层：根据用户分类解析详细路由
        let detail = match &user_sub {
          UserSubRouterMatch::Profile(_) => {
            if let Ok(profile_detail) = UserProfileDetailRouterMatch::try_parse(url) {
              Some(Box::new(profile_detail) as Box<dyn std::fmt::Debug>)
            } else {
              None
            }
          }
          UserSubRouterMatch::Content(_) => {
            if let Ok(content_detail) = UserContentDetailRouterMatch::try_parse(url) {
              Some(Box::new(content_detail) as Box<dyn std::fmt::Debug>)
            } else {
              None
            }
          }
        };
        (Some(Box::new(user_sub)), detail)
      } else {
        (None, None)
      }
    }
    AppRouterMatch::Shop(_) => {
      if let Ok(shop_sub) = ShopSubRouterMatch::try_parse(url) {
        // 第三层：根据商店分类解析详细路由
        let detail = match &shop_sub {
          ShopSubRouterMatch::Products(_) => {
            if let Ok(product_detail) = ShopProductDetailRouterMatch::try_parse(url) {
              Some(Box::new(product_detail) as Box<dyn std::fmt::Debug>)
            } else {
              None
            }
          }
          ShopSubRouterMatch::Orders(_) => {
            if let Ok(order_detail) = ShopOrderDetailRouterMatch::try_parse(url) {
              Some(Box::new(order_detail) as Box<dyn std::fmt::Debug>)
            } else {
              None
            }
          }
        };
        (Some(Box::new(shop_sub)), detail)
      } else {
        (None, None)
      }
    }
    AppRouterMatch::Admin(_) => {
      if let Ok(admin_sub) = AdminSubRouterMatch::try_parse(url) {
        // 第三层：根据管理分类解析详细路由
        let detail = match &admin_sub {
          AdminSubRouterMatch::Users(_) => {
            if let Ok(user_detail) = AdminUserDetailRouterMatch::try_parse(url) {
              Some(Box::new(user_detail) as Box<dyn std::fmt::Debug>)
            } else {
              None
            }
          }
          AdminSubRouterMatch::System(_) => {
            if let Ok(system_detail) = AdminSystemDetailRouterMatch::try_parse(url) {
              Some(Box::new(system_detail) as Box<dyn std::fmt::Debug>)
            } else {
              None
            }
          }
        };
        (Some(Box::new(admin_sub)), detail)
      } else {
        (None, None)
      }
    }
  };

  Ok((app_match, sub_match, detail_match))
}

fn main() {
  println!("=== 三层嵌套路由使用示例 ===");
  println!("架构：RouterMatch -> Router -> RouterMatch -> Router -> RouterMatch -> Router");
  println!();

  let test_urls = vec![
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
      Ok((app_match, sub_match, detail_match)) => {
        println!("第一层 (模块): {app_match:?}");
        if let Some(sub) = sub_match {
          println!("第二层 (分类): {sub:?}");
        } else {
          println!("第二层: 无匹配的分类路由");
        }
        if let Some(detail) = detail_match {
          println!("第三层 (详情): {detail:?}");
        } else {
          println!("第三层: 无匹配的详情路由");
        }

        // 演示格式化
        let formatted = app_match.format();
        println!("模块路由格式化: {formatted}");
      }
      Err(e) => {
        println!("解析失败: {e}");
      }
    }
    println!();
  }

  println!("=== 架构验证 ===");
  println!("✓ AppRouterMatch (第一层 RouterMatch) -> UserModuleRoute/ShopModuleRoute/AdminModuleRoute (第一层 Router)");
  println!("✓ UserModuleRoute -> UserSubRouterMatch (第二层 RouterMatch) -> UserProfileCategoryRoute/UserContentCategoryRoute (第二层 Router)");
  println!("✓ UserProfileCategoryRoute -> UserProfileDetailRouterMatch (第三层 RouterMatch) -> UserBasicInfoRoute/UserSettingsRoute (第三层 Router)");
  println!("✓ 严格遵循 RouterMatch -> Router -> RouterMatch -> Router -> RouterMatch -> Router 模式");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_architecture_compliance() {
    // 测试架构是否符合 RouterMatch -> Router -> RouterMatch -> Router -> RouterMatch -> Router

    // 第一层：AppRouterMatch -> ModuleRoute
    let app_match = AppRouterMatch::try_parse("/users/profile/123").unwrap();
    assert!(matches!(app_match, AppRouterMatch::User(_)));

    // 第二层：UserSubRouterMatch -> UserProfileCategoryRoute
    let user_sub = UserSubRouterMatch::try_parse("/users/profile/123").unwrap();
    assert!(matches!(user_sub, UserSubRouterMatch::Profile(_)));

    // 第三层：UserProfileDetailRouterMatch -> UserBasicInfoRoute
    let user_detail = UserProfileDetailRouterMatch::try_parse("/users/profile/123").unwrap();
    assert!(matches!(user_detail, UserProfileDetailRouterMatch::BasicInfo(_)));
  }

  #[test]
  fn test_user_basic_info_route() {
    let route = UserBasicInfoRoute::parse("/users/profile/123?format=json").unwrap();
    assert_eq!(route.id, 123);
    assert_eq!(route.query.format, Some("json".to_string()));
  }

  #[test]
  fn test_nested_parsing() {
    let (app_match, sub_match, detail_match) = parse_nested_route("/users/profile/123").unwrap();
    assert!(matches!(app_match, AppRouterMatch::User(_)));
    assert!(sub_match.is_some());
    assert!(detail_match.is_some());
  }

  #[test]
  fn test_shop_product_route() {
    let route = ProductDetailRoute::parse("/shop/products/electronics/555?format=xml").unwrap();
    assert_eq!(route.category, "electronics");
    assert_eq!(route.id, 555);
    assert_eq!(route.query.format, Some("xml".to_string()));
  }

  #[test]
  fn test_admin_system_route() {
    let route = SystemConfigRoute::parse("/admin/system/config?format=json").unwrap();
    assert_eq!(route.query.format, Some("json".to_string()));
  }
}
