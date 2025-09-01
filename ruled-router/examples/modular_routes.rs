use ruled_router::error::RouteState;
use ruled_router::prelude::*;
use ruled_router::RouteMatcher;
use ruled_router_derive::{QueryDerive, RouterData, RouterMatch};

#[derive(Debug, Clone, PartialEq, Default, QueryDerive)]
struct SimpleQuery {
  #[query(name = "format")]
  format: Option<String>,
}

// User module routes
mod user {
  use super::*;

  #[derive(Debug, Clone, PartialEq, RouterData)]
  #[router(pattern = "/profile/:id")]
  pub struct UserProfileRoute {
    pub id: u32,
    #[query]
    pub query: SimpleQuery,
  }

  #[derive(Debug, Clone, PartialEq, RouterData)]
  #[router(pattern = "/settings")]
  pub struct UserSettingsRoute {
    #[query]
    pub query: SimpleQuery,
  }

  #[derive(Debug, Clone, PartialEq, RouterMatch)]
  pub enum UserSubRouterMatch {
    Profile(UserProfileRoute),
    Settings(UserSettingsRoute),
  }

  #[derive(Debug, Clone, PartialEq, RouterData)]
  #[router(pattern = "/users")]
  pub struct UserModuleRoute {
    #[query]
    pub query: SimpleQuery,
    #[sub_router]
    pub sub_router: RouteState<UserSubRouterMatch>,
  }
}

// Blog module routes
mod blog {
  use super::*;

  #[derive(Debug, Clone, PartialEq, RouterData)]
  #[router(pattern = "/post/:slug")]
  pub struct BlogPostRoute {
    pub slug: String,
    #[query]
    pub query: SimpleQuery,
  }

  #[derive(Debug, Clone, PartialEq, RouterMatch)]
  pub enum BlogSubRouterMatch {
    Post(BlogPostRoute),
  }

  #[derive(Debug, Clone, PartialEq, RouterData)]
  #[router(pattern = "/blog")]
  pub struct BlogModuleRoute {
    #[query]
    pub query: SimpleQuery,
    #[sub_router]
    pub sub_router: RouteState<BlogSubRouterMatch>,
  }
}

// Top-level route aggregation
#[derive(Debug, Clone, PartialEq, RouterMatch)]
enum AppRouterMatch {
  User(user::UserModuleRoute),
  Blog(blog::BlogModuleRoute),
}

fn main() {
  let test_paths = vec!["/users/profile/123", "/users/settings", "/blog/post/hello-world"];

  for path in test_paths {
    println!("\n解析路径: {path}");
    match AppRouterMatch::try_parse(path) {
      Ok(route_match) => {
        println!("  匹配成功: {route_match:?}");
        match route_match {
          AppRouterMatch::User(user_route) => {
            println!("  用户模块路由");
            if let RouteState::SubRoute(sub_route) = &user_route.sub_router {
              match sub_route {
                user::UserSubRouterMatch::Profile(profile) => {
                  println!("    用户资料: ID={}", profile.id);
                }
                user::UserSubRouterMatch::Settings(_) => {
                  println!("    用户设置页面");
                }
              }
            }
          }
          AppRouterMatch::Blog(blog_route) => {
            println!("  博客模块路由");
            if let RouteState::SubRoute(sub_route) = &blog_route.sub_router {
              match sub_route {
                blog::BlogSubRouterMatch::Post(post) => {
                  println!("    博客文章: Slug={}", post.slug);
                }
              }
            }
          }
        }
      }
      Err(e) => println!("  解析失败: {e:?}"),
    }
  }
}
