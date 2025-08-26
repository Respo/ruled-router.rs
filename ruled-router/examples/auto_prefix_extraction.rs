use ruled_router::prelude::*;
use ruled_router::RouteMatcher;
use ruled_router_derive::RouterMatch;

// Route matcher - automatic prefix extraction, no need to repeat path definitions
#[derive(Debug, RouterMatch)]
enum AppRouterMatch {
  Users(UsersModuleRoute), // Auto-extracted prefix: "/users"
  Blog(BlogModuleRoute),   // Auto-extracted prefix: "/blog"
  Api(ApiModuleRoute),     // Auto-extracted prefix: "/api"
}

#[derive(Debug, RouterData)]
#[router(pattern = "/users")]
struct UsersModuleRoute {
  #[sub_router]
  sub_router: Option<UserSubRouterMatch>,
}

#[derive(Debug, RouterMatch)]
enum UserSubRouterMatch {
  Detail(UserDetailRoute),
}

#[derive(Debug, RouterData)]
#[router(pattern = "/:id")]
struct UserDetailRoute {
  id: u32,
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
#[router(pattern = "/:slug")]
struct BlogPostRoute {
  slug: String,
}

#[derive(Debug, RouterData)]
#[router(pattern = "/api")]
struct ApiModuleRoute {
  #[sub_router]
  sub_router: Option<ApiSubRouterMatch>,
}

#[derive(Debug, RouterMatch)]
enum ApiSubRouterMatch {
  V1(ApiV1Route),
}

#[derive(Debug, RouterData)]
#[router(pattern = "/v1")]
struct ApiV1Route {
  // Empty route with no parameters
}

fn main() {
  // Automatic route matching
  let paths = ["/users/123", "/blog/hello-world", "/api/v1"];

  for path in paths {
    match AppRouterMatch::try_parse(path) {
      Ok(route) => match route {
        AppRouterMatch::Users(users_route) => {
          if let Some(UserSubRouterMatch::Detail(detail)) = &users_route.sub_router {
            println!("匹配成功: {} -> 用户详情，ID: {}", path, detail.id);
          }
        }
        AppRouterMatch::Blog(blog_route) => {
          if let Some(BlogSubRouterMatch::Post(post)) = &blog_route.sub_router {
            println!("匹配成功: {} -> 博客文章，Slug: {}", path, post.slug);
          }
        }
        AppRouterMatch::Api(api_route) => {
          if let Some(ApiSubRouterMatch::V1(_)) = &api_route.sub_router {
            println!("匹配成功: {path} -> API v1");
          }
        }
      },
      Err(e) => println!("匹配失败: {path} -> {e:?}"),
    }
  }
}
