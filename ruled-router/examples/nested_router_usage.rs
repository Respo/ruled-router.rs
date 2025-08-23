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

// ===== 查询参数定义 =====

#[derive(Debug, Clone, PartialEq, Default, Query)]
struct SimpleQuery {
  #[query(name = "format")]
  format: Option<String>,
}

// ===== 第一层：顶层路由匹配器和路由 =====

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

/// 应用根路由 - 第一层 Router
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/")]
struct AppRoute {
  #[query]
  query: SimpleQuery,
}

// ===== 第二层：模块路由 =====

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

// ===== 第二层：子路由匹配器 =====

/// 用户子路由匹配 - 第二层 RouteMatcher
#[derive(Debug, Clone, PartialEq, RouterMatch)]
enum UserSubRouterMatch {
  #[route("/profile")]
  Profile(UserProfileCategoryRoute),
  #[route("/content")]
  Content(UserContentCategoryRoute),
}

/// 商店子路由匹配器 - 第二层 RouterMatch
#[derive(Debug, Clone, PartialEq, RouterMatch)]
enum ShopSubRouterMatch {
  #[route("/products")]
  Products(ShopProductCategoryRoute),
  #[route("/orders")]
  Orders(ShopOrderCategoryRoute),
}

/// 管理员子路由匹配器 - 第二层 RouterMatch
#[derive(Debug, Clone, PartialEq, RouterMatch)]
enum AdminSubRouterMatch {
  #[route("/users")]
  Users(AdminUserCategoryRoute),
  #[route("/system")]
  System(AdminSystemCategoryRoute),
}

// ===== 第三层：分类路由 =====

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

// ===== 第三层：详情路由匹配器 =====

/// 用户个人详情路由匹配 - 第四层 RouteMatcher
#[derive(Debug, Clone, PartialEq, RouterMatch)]
enum UserProfileDetailRouterMatch {
  #[route("/basic")]
  BasicInfo(UserBasicInfoRoute),
  #[route("/settings")]
  Settings(UserSettingsRoute),
}

/// 用户内容详情路由匹配 - 第四层 RouteMatcher
#[derive(Debug, Clone, PartialEq, RouterMatch)]
enum UserContentDetailRouterMatch {
  #[route("/posts")]
  Post(UserPostRoute),
  #[route("/comments")]
  Comment(UserCommentRoute),
}

/// 商店产品详情路由匹配 - 第四层 RouteMatcher
#[derive(Debug, Clone, PartialEq, RouterMatch)]
enum ShopProductDetailRouterMatch {
  #[route("/detail")]
  Detail(ProductDetailRoute),
  #[route("/list")]
  List(ProductListRoute),
}

/// 商店订单详情路由匹配 - 第四层 RouteMatcher
#[derive(Debug, Clone, PartialEq, RouterMatch)]
enum ShopOrderDetailRouterMatch {
  #[route("/detail")]
  Detail(OrderDetailRoute),
}

/// 管理用户详情路由匹配 - 第四层 RouteMatcher
#[derive(Debug, Clone, PartialEq, RouterMatch)]
enum AdminUserDetailRouterMatch {
  #[route("/manage")]
  Manage(AdminUserManageRoute),
}

/// 管理系统详情路由匹配 - 第四层 RouteMatcher
#[derive(Debug, Clone, PartialEq, RouterMatch)]
enum AdminSystemDetailRouterMatch {
  #[route("/config")]
  Config(SystemConfigRoute),
}

// ===== 第四层：详细路由 =====

/// 用户基本信息路由 - 第四层 Router
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/basic/:id")]
struct UserBasicInfoRoute {
  id: u32,
  #[query]
  query: SimpleQuery,
}

/// 用户设置路由 - 第四层 Router
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/settings/:id")]
struct UserSettingsRoute {
  id: u32,
  #[query]
  query: SimpleQuery,
}

/// 用户文章路由 - 第四层 Router
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/posts/:user_id/:post_id")]
struct UserPostRoute {
  user_id: u32,
  post_id: u32,
  #[query]
  query: SimpleQuery,
}

/// 用户评论路由 - 第四层 Router
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/comments/:user_id/:comment_id")]
struct UserCommentRoute {
  user_id: u32,
  comment_id: u32,
  #[query]
  query: SimpleQuery,
}

/// 产品详情路由 - 第四层 Router
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/detail/:category/:id")]
struct ProductDetailRoute {
  category: String,
  id: u32,
  #[query]
  query: SimpleQuery,
}

/// 产品列表路由 - 第四层 Router
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/list/:category")]
struct ProductListRoute {
  category: String,
  #[query]
  query: SimpleQuery,
}

/// 订单详情路由 - 第四层 Router
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/detail/:id")]
struct OrderDetailRoute {
  id: u32,
  #[query]
  query: SimpleQuery,
}

/// 管理员用户管理路由 - 第四层 Router
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/manage/:id")]
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

// ===== 路由解析函数 =====

fn parse_nested_route(url: &str) -> Result<String, Box<dyn std::error::Error>> {
  // 第一层：解析应用路由
  let (app_match, remaining) = AppRouterMatch::try_parse_with_remaining(url, 0)?;

  match app_match {
    AppRouterMatch::User(mut user_route) => {
      // 手动解析子路由
      if !remaining.is_empty() {
        if let Ok((mut user_sub_match, sub_remaining)) = UserSubRouterMatch::try_parse_with_remaining(remaining, 0) {
          // 进一步解析第三层路由
          match &mut user_sub_match {
            UserSubRouterMatch::Profile(ref mut profile_route) => {
              if !sub_remaining.is_empty() {
                if let Ok((profile_detail_match, _detail_remaining)) =
                  UserProfileDetailRouterMatch::try_parse_with_remaining(sub_remaining, 0)
                {
                  profile_route.sub_router = Some(profile_detail_match);
                }
              }
            }
            UserSubRouterMatch::Content(ref mut content_route) => {
              if !sub_remaining.is_empty() {
                if let Ok((content_detail_match, _detail_remaining)) =
                  UserContentDetailRouterMatch::try_parse_with_remaining(sub_remaining, 0)
                {
                  content_route.sub_router = Some(content_detail_match);
                }
              }
            }
          }
          user_route.sub_router = Some(user_sub_match);
        }
      }

      if let Some(user_sub_match) = user_route.sub_router {
        match user_sub_match {
          UserSubRouterMatch::Profile(profile_route) => {
            if let Some(profile_detail_match) = profile_route.sub_router {
              match profile_detail_match {
                UserProfileDetailRouterMatch::BasicInfo(basic_info_route) => Ok(format!(
                  "用户基本信息: ID={}, 格式={:?}",
                  basic_info_route.id, basic_info_route.query.format
                )),
                UserProfileDetailRouterMatch::Settings(settings_route) => Ok(format!(
                  "用户设置: ID={}, 格式={:?}",
                  settings_route.id, settings_route.query.format
                )),
              }
            } else {
              Ok("用户个人资料分类页面".to_string())
            }
          }
          UserSubRouterMatch::Content(content_route) => {
            if let Some(content_detail_match) = content_route.sub_router {
              match content_detail_match {
                UserContentDetailRouterMatch::Post(post_route) => Ok(format!(
                  "用户文章: 用户ID={}, 文章ID={}, 格式={:?}",
                  post_route.user_id, post_route.post_id, post_route.query.format
                )),
                UserContentDetailRouterMatch::Comment(comment_route) => Ok(format!(
                  "用户评论: 用户ID={}, 评论ID={}, 格式={:?}",
                  comment_route.user_id, comment_route.comment_id, comment_route.query.format
                )),
              }
            } else {
              Ok("用户内容分类页面".to_string())
            }
          }
        }
      } else {
        Ok("用户模块主页".to_string())
      }
    }
    AppRouterMatch::Shop(mut shop_route) => {
      // 手动解析子路由
      if !remaining.is_empty() {
        if let Ok((mut shop_sub_match, sub_remaining)) = ShopSubRouterMatch::try_parse_with_remaining(remaining, 0) {
          // 进一步解析第三层路由
          match &mut shop_sub_match {
            ShopSubRouterMatch::Products(ref mut product_route) => {
              if !sub_remaining.is_empty() {
                if let Ok((product_detail_match, _detail_remaining)) =
                  ShopProductDetailRouterMatch::try_parse_with_remaining(sub_remaining, 0)
                {
                  product_route.sub_router = Some(product_detail_match);
                }
              }
            }
            ShopSubRouterMatch::Orders(ref mut order_route) => {
              if !sub_remaining.is_empty() {
                if let Ok((order_detail_match, _detail_remaining)) =
                  ShopOrderDetailRouterMatch::try_parse_with_remaining(sub_remaining, 0)
                {
                  order_route.sub_router = Some(order_detail_match);
                }
              }
            }
          }
          shop_route.sub_router = Some(shop_sub_match);
        }
      }

      if let Some(shop_sub_match) = shop_route.sub_router {
        match shop_sub_match {
          ShopSubRouterMatch::Products(product_route) => {
            if let Some(product_detail_match) = product_route.sub_router {
              match product_detail_match {
                ShopProductDetailRouterMatch::Detail(detail_route) => Ok(format!(
                  "产品详情: 分类={}, ID={}, 格式={:?}",
                  detail_route.category, detail_route.id, detail_route.query.format
                )),
                ShopProductDetailRouterMatch::List(list_route) => Ok(format!(
                  "产品列表: 分类={}, 格式={:?}",
                  list_route.category, list_route.query.format
                )),
              }
            } else {
              Ok("商店产品分类页面".to_string())
            }
          }
          ShopSubRouterMatch::Orders(order_route) => {
            if let Some(order_detail_match) = order_route.sub_router {
              match order_detail_match {
                ShopOrderDetailRouterMatch::Detail(detail_route) => {
                  Ok(format!("订单详情: ID={}, 格式={:?}", detail_route.id, detail_route.query.format))
                }
              }
            } else {
              Ok("商店订单分类页面".to_string())
            }
          }
        }
      } else {
        Ok("商店模块主页".to_string())
      }
    }
    AppRouterMatch::Admin(mut admin_route) => {
      // 手动解析子路由
      if !remaining.is_empty() {
        if let Ok((mut admin_sub_match, sub_remaining)) = AdminSubRouterMatch::try_parse_with_remaining(remaining, 0) {
          // 进一步解析第三层路由
          match &mut admin_sub_match {
            AdminSubRouterMatch::Users(ref mut user_category_route) => {
              if !sub_remaining.is_empty() {
                if let Ok((user_detail_match, _detail_remaining)) =
                  AdminUserDetailRouterMatch::try_parse_with_remaining(sub_remaining, 0)
                {
                  user_category_route.sub_router = Some(user_detail_match);
                }
              }
            }
            AdminSubRouterMatch::System(ref mut system_category_route) => {
              if !sub_remaining.is_empty() {
                if let Ok((system_detail_match, _detail_remaining)) =
                  AdminSystemDetailRouterMatch::try_parse_with_remaining(sub_remaining, 0)
                {
                  system_category_route.sub_router = Some(system_detail_match);
                }
              }
            }
          }
          admin_route.sub_router = Some(admin_sub_match);
        }
      }

      if let Some(admin_sub_match) = admin_route.sub_router {
        match admin_sub_match {
          AdminSubRouterMatch::Users(user_category_route) => {
            if let Some(user_detail_match) = user_category_route.sub_router {
              match user_detail_match {
                AdminUserDetailRouterMatch::Manage(manage_route) => Ok(format!(
                  "管理员用户管理: ID={}, 格式={:?}",
                  manage_route.id, manage_route.query.format
                )),
              }
            } else {
              Ok("管理员用户分类页面".to_string())
            }
          }
          AdminSubRouterMatch::System(system_category_route) => {
            if let Some(system_detail_match) = system_category_route.sub_router {
              match system_detail_match {
                AdminSystemDetailRouterMatch::Config(config_route) => Ok(format!("系统配置: 格式={:?}", config_route.query.format)),
              }
            } else {
              Ok("管理员系统分类页面".to_string())
            }
          }
        }
      } else {
        Ok("管理模块主页".to_string())
      }
    }
  }
}

fn main() {
  let test_urls = vec![
    "/users/profile/basic/123?format=json",
    "/users/profile/settings/456?format=xml",
    "/users/content/posts/789/101112?format=yaml",
    "/users/content/comments/131415/161718?format=toml",
    "/shop/products/detail/electronics/999?format=json",
    "/shop/products/list/books?format=xml",
    "/shop/orders/detail/202122?format=yaml",
    "/admin/users/manage/232425?format=json",
    "/admin/system/config?format=xml",
  ];

  println!("=== 三层嵌套路由解析示例 ===");
  for url in test_urls {
    match parse_nested_route(url) {
      Ok(result) => println!("✓ {url} -> {result}"),
      Err(e) => println!("✗ {url} -> 错误: {e}"),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_architecture_compliance() {
    // 测试第一层路由匹配
    let (app_match, remaining) = AppRouterMatch::try_parse_with_remaining("/users/profile/basic/123", 0).unwrap();
    assert!(matches!(app_match, AppRouterMatch::User(_)));
    assert_eq!(remaining, "/profile/basic/123");

    // 测试第二层路由匹配 - UserSubRouterMatch 只处理相对路径
    let (user_sub_match, remaining) = UserSubRouterMatch::try_parse_with_remaining("/profile/basic/123", 0).unwrap();
    assert!(matches!(user_sub_match, UserSubRouterMatch::Profile(_)));
    assert_eq!(remaining, "/basic/123");

    // 第三层：UserProfileDetailRouterMatch 解析 "/basic/123"，匹配 /basic 前缀
    let (user_profile_detail_match, remaining) = UserProfileDetailRouterMatch::try_parse_with_remaining("/basic/123", 0).unwrap();
    assert!(matches!(user_profile_detail_match, UserProfileDetailRouterMatch::BasicInfo(_)));
    assert_eq!(remaining, "");
  }

  #[test]
  fn test_user_basic_info_route() {
    let route = UserBasicInfoRoute::parse("/basic/123?format=json").unwrap();
    assert_eq!(route.id, 123);
    assert_eq!(route.query.format, Some("json".to_string()));
  }

  #[test]
  fn test_nested_parsing() {
    let result = parse_nested_route("/users/profile/basic/123").unwrap();
    assert!(result.contains("用户基本信息"));
    assert!(result.contains("123"));
  }

  #[test]
  fn test_shop_product_route() {
    let route = ProductDetailRoute::parse("/detail/electronics/555?format=xml").unwrap();
    assert_eq!(route.category, "electronics");
    assert_eq!(route.id, 555);
    assert_eq!(route.query.format, Some("xml".to_string()));
  }

  #[test]
  fn test_admin_system_route() {
    let route = SystemConfigRoute::parse("/config?format=yaml").unwrap();
    assert_eq!(route.query.format, Some("yaml".to_string()));
  }
}
