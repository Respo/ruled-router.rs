use ruled_router::error::RouteState;
use ruled_router::prelude::*;
use ruled_router::RouteMatcher;
use ruled_router_derive::{QueryDerive, RouterData, RouterMatch};

// Define top-level route matcher (enum for matching different routes)
#[derive(RouterMatch, Debug, Clone)]
enum AppRouterMatch {
  User(UserModuleRoute),
}

// Define module route with fixed prefix pattern
#[derive(RouterData, Debug, Clone)]
#[router(pattern = "/users")]
struct UserModuleRoute {
  #[query]
  query: UserQuery,
  #[sub_router]
  sub_router: RouteState<UserSubRouterMatch>,
}

// Define sub-router matcher for user routes
#[derive(RouterMatch, Debug, Clone)]
enum UserSubRouterMatch {
  Detail(UserDetailRoute),
}

// Define detail route with dynamic parameter
#[derive(RouterData, Debug, Clone)]
#[router(pattern = "/:id")]
struct UserDetailRoute {
  id: u32,
  #[query]
  query: UserQuery,
}

// Define query parameters
#[derive(QueryDerive, Debug, Default, Clone)]
struct UserQuery {
  #[query(name = "tab")]
  tab: Option<String>,
  #[query(name = "page", default = "1")]
  page: u32,
}

fn main() {
  // Parse route
  let path = "/users/123?tab=profile&page=2";
  let route_match = AppRouterMatch::try_parse(path).unwrap();

  match route_match {
    AppRouterMatch::User(user_module) => {
      println!("匹配到用户模块");
      match &user_module.sub_router {
        RouteState::SubRoute(UserSubRouterMatch::Detail(detail)) => {
          println!("用户ID: {}", detail.id);
          println!("标签页: {:?}", detail.query.tab);
          println!("页码: {}", detail.query.page);

          // Format route
          let formatted = detail.format();
          println!("格式化结果: {formatted}");
        }
        RouteState::NoSubRoute => {
          println!("子路由状态: NoSubRoute");
          println!("模块查询参数: {:?}", user_module.query);
        }
        RouteState::ParseFailed {
          remaining_path,
          attempted_patterns,
          closest_match,
        } => {
          println!("子路由状态: ParseFailed");
          println!("剩余路径: {remaining_path}");
          println!("尝试匹配的模式: {attempted_patterns:?}");
          println!("最接近的匹配: {closest_match:?}");
          println!("模块查询参数: {:?}", user_module.query);
        }
      }
    }
  }
}
