use ruled_router::error::RouteState;
use ruled_router::prelude::*;
use ruled_router_derive::{QueryDerive, RouterData, RouterMatch};

#[derive(Debug, QueryDerive, PartialEq)]
struct TestQuery {
  #[query(name = "tab")]
  tab: Option<String>,
}

#[derive(Debug, RouterData)]
#[router(pattern = "/user/:id")]
struct UserRoute {
  id: u32,
  #[query]
  query: TestQuery,
  #[sub_router]
  sub_router: RouteState<UserSubRouterMatch>,
}

#[derive(Debug, RouterMatch)]
enum UserSubRouterMatch {
  Profile(ProfileRoute),
  Settings(SettingsRoute),
}

#[derive(Debug, RouterData)]
#[router(pattern = "/profile")]
struct ProfileRoute {
  #[query]
  query: TestQuery,
}

#[derive(Debug, RouterData)]
#[router(pattern = "/settings")]
struct SettingsRoute {
  #[query]
  query: TestQuery,
}

fn main() {
  println!("=== RouterData format_sub_router 演示 ===\n");

  // 测试 1: 没有子路由的情况
  println!("测试 1: 没有子路由的情况");
  let route_no_sub = UserRoute {
    id: 123,
    query: TestQuery {
      tab: Some("profile".to_string()),
    },
    sub_router: RouteState::no_sub_route(),
  };

  println!("  基础 format():           {}", route_no_sub.format());
  println!("  新方法 format_sub_router(): {}", route_no_sub.format_sub_router());
  println!();

  // 测试 2: 有 Profile 子路由的情况
  println!("测试 2: 有 Profile 子路由的情况");
  let profile_route = ProfileRoute {
    query: TestQuery {
      tab: Some("basic".to_string()),
    },
  };

  let route_with_profile = UserRoute {
    id: 456,
    query: TestQuery { tab: None },
    sub_router: RouteState::sub_route(UserSubRouterMatch::Profile(profile_route)),
  };

  println!("  基础 format():           {}", route_with_profile.format());
  println!("  新方法 format_sub_router(): {}", route_with_profile.format_sub_router());
  println!();

  // 测试 3: 有 Settings 子路由的情况
  println!("测试 3: 有 Settings 子路由的情况");
  let settings_route = SettingsRoute {
    query: TestQuery {
      tab: Some("general".to_string()),
    },
  };

  let route_with_settings = UserRoute {
    id: 789,
    query: TestQuery { tab: None },
    sub_router: RouteState::sub_route(UserSubRouterMatch::Settings(settings_route)),
  };

  println!("  基础 format():           {}", route_with_settings.format());
  println!("  新方法 format_sub_router(): {}", route_with_settings.format_sub_router());
  println!();

  // 测试 4: 往返测试
  println!("测试 4: 往返测试（解析和格式化）");
  let test_paths = [
    "/user/111/profile?tab=advanced",
    "/user/222/settings?tab=security",
    "/user/333?tab=main",
  ];

  for path in test_paths {
    println!("  原始路径: {path}");

    match UserRoute::parse_with_sub(path) {
      Ok((route, sub_match)) => {
        let route_with_sub = UserRoute {
          id: route.id,
          query: route.query,
          sub_router: sub_match,
        };

        let formatted = route_with_sub.format_sub_router();
        println!("  格式化后: {formatted}");
        println!("  是否匹配: {}", path == formatted);
      }
      Err(e) => {
        println!("  解析失败: {e:?}");
      }
    }
    println!();
  }

  println!("=== 演示完成 ===");
}
