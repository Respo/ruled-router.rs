use ruled_router::prelude::*;
use ruled_router::RouteMatcher;
use ruled_router_derive::{QueryDerive, RouterData, RouterMatch};

// Simple two-level nested routing example
#[derive(Debug, RouterMatch)]
enum AppRouterMatch {
  User(UserModuleRoute),
  Blog(BlogModuleRoute),
}

#[derive(Debug, RouterData)]
#[router(pattern = "/user")]
struct UserModuleRoute {
  #[sub_router]
  sub_router: Option<UserSubRouterMatch>,
}

#[derive(Debug, RouterMatch)]
enum UserSubRouterMatch {
  Profile(UserProfileRoute),
  Settings(UserSettingsRoute),
}

#[derive(Debug, RouterData)]
#[router(pattern = "/profile/:id")]
struct UserProfileRoute {
  id: u32,
  #[query]
  query: UserQuery,
}

#[derive(Debug, RouterData)]
#[router(pattern = "/settings")]
struct UserSettingsRoute {
  #[query]
  query: UserQuery,
}

#[derive(Debug, RouterData)]
#[router(pattern = "/blog")]
struct BlogModuleRoute {
  #[sub_router]
  sub_router: Option<BlogSubRouterMatch>,
}

#[derive(Debug, RouterMatch)]
enum BlogSubRouterMatch {
  Post(BlogPostRoute),
}

#[derive(Debug, RouterData)]
#[router(pattern = "/post/:slug")]
struct BlogPostRoute {
  slug: String,
  #[query]
  query: BlogQuery,
}

#[derive(Debug, QueryDerive)]
struct UserQuery {
  #[query(name = "tab")]
  tab: Option<String>,
}

#[derive(Debug, QueryDerive)]
struct BlogQuery {
  #[query(name = "format")]
  format: Option<String>,
}

fn main() {
  let test_paths = [
    "/user/profile/123?tab=settings",
    "/user/settings",
    "/blog/post/hello-world?format=json",
  ];

  for path in test_paths {
    match AppRouterMatch::try_parse(path) {
      Ok(route) => match route {
        AppRouterMatch::User(user_route) => match &user_route.sub_router {
          Some(UserSubRouterMatch::Profile(profile)) => {
            println!("用户资料: ID={}, Tab={:?}", profile.id, profile.query.tab);
          }
          Some(UserSubRouterMatch::Settings(settings)) => {
            println!("用户设置: Tab={:?}", settings.query.tab);
          }
          None => println!("用户模块根路径"),
        },
        AppRouterMatch::Blog(blog_route) => match &blog_route.sub_router {
          Some(BlogSubRouterMatch::Post(post)) => {
            println!("博客文章: Slug={}, Format={:?}", post.slug, post.query.format);
          }
          None => println!("博客模块根路径"),
        },
      },
      Err(e) => println!("解析失败: {path} -> {e:?}"),
    }
  }
}
