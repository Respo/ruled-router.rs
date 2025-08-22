//! 嵌套路由使用示例 - RouterMatch 设计
//!
//! 这个示例展示了新的嵌套路由设计：RouterMatch > Router > RouterMatch > Router
//! 主要特性：
//! - RouterMatch 枚举表示路由选择
//! - Router 结构体表示具体路由
//! - 支持任意深度的嵌套
//! - 类型安全的路由匹配

use ruled_router::prelude::*;
use ruled_router::RouterMatch;

// ===== 查询参数定义 =====

/// 用户查询参数
#[derive(Debug, Clone, PartialEq, Default, Query)]
struct UserQuery {
  /// 包含的字段列表
  include_fields: Vec<String>,
  /// 是否包含敏感信息
  include_sensitive: Option<bool>,
  /// 响应格式
  format: Option<String>,
}

/// 博客文章查询参数
#[derive(Debug, Clone, PartialEq, Default, Query)]
struct PostQuery {
  /// 是否包含评论
  include_comments: Option<bool>,
  /// 评论排序方式
  comment_sort: Option<String>,
  /// 评论页码
  comment_page: Option<u32>,
}

/// API 查询参数
#[derive(Debug, Clone, PartialEq, Default, Query)]
struct ApiQuery {
  /// API 版本
  version: Option<String>,
  /// 调试模式
  debug: Option<bool>,
  /// 响应格式
  format: Option<String>,
}

// ===== 基础路由定义 =====

/// 用户路由
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/users/:id")]
struct UserRoute {
  id: u32,
  #[query]
  query: UserQuery,
}

/// 用户资料路由
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/users/:user_id/profile")]
struct UserProfileRoute {
  user_id: u32,
  #[query]
  query: UserQuery,
}

/// 用户文章路由
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/users/:user_id/posts/:post_id")]
struct UserPostRoute {
  user_id: u32,
  post_id: u32,
  #[query]
  query: PostQuery,
}

/// 用户设置路由
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/users/:user_id/settings")]
struct UserSettingsRoute {
  user_id: u32,
}

/// 博客路由
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/blog/:year/:month/:slug")]
struct BlogRoute {
  year: u32,
  month: u32,
  slug: String,
  #[query]
  query: PostQuery,
}

/// API v1 用户路由
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/api/v1/users/:id")]
struct ApiV1UserRoute {
  id: u32,
  #[query]
  query: ApiQuery,
}

/// API v1 文章路由
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/api/v1/posts/:id")]
struct ApiV1PostRoute {
  id: u32,
  #[query]
  query: ApiQuery,
}

/// API v2 用户路由
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/api/v2/users/:id")]
struct ApiV2UserRoute {
  id: u32,
  #[query]
  query: ApiQuery,
}

// ===== RouterMatch 枚举定义（手动实现）=====

/// 用户子路由匹配
#[derive(Debug, Clone, PartialEq)]
enum UserSubRouterMatch {
  Profile(UserProfileRoute),
  Post(UserPostRoute),
  Settings(UserSettingsRoute),
}

/// API v1 子路由匹配
#[derive(Debug, Clone, PartialEq)]
enum ApiV1SubRouterMatch {
  User(ApiV1UserRoute),
  Post(ApiV1PostRoute),
}

/// API v2 子路由匹配
#[derive(Debug, Clone, PartialEq)]
enum ApiV2SubRouterMatch {
  User(ApiV2UserRoute),
}

/// API 版本路由匹配
#[derive(Debug, Clone, PartialEq)]
enum ApiVersionRouterMatch {
  V1(ApiV1SubRouterMatch),
  V2(ApiV2SubRouterMatch),
}

/// 顶级应用路由匹配
#[derive(Debug, Clone, PartialEq)]
enum AppRouterMatch {
  User(UserRoute),
  UserSub(UserSubRouterMatch),
  Blog(BlogRoute),
  Api(ApiVersionRouterMatch),
}

// ===== 手动实现 RouterMatch trait =====

impl RouterMatch for UserSubRouterMatch {
  fn try_parse(path: &str) -> Result<Self, ParseError> {
    if let Ok(profile) = UserProfileRoute::parse(path) {
      return Ok(UserSubRouterMatch::Profile(profile));
    }
    if let Ok(post) = UserPostRoute::parse(path) {
      return Ok(UserSubRouterMatch::Post(post));
    }
    if let Ok(settings) = UserSettingsRoute::parse(path) {
      return Ok(UserSubRouterMatch::Settings(settings));
    }
    Err(ParseError::invalid_path("No matching user sub route"))
  }

  fn format(&self) -> String {
    match self {
      UserSubRouterMatch::Profile(route) => route.format(),
      UserSubRouterMatch::Post(route) => route.format(),
      UserSubRouterMatch::Settings(route) => route.format(),
    }
  }

  fn patterns() -> Vec<&'static str> {
    vec![UserProfileRoute::pattern(), UserPostRoute::pattern(), UserSettingsRoute::pattern()]
  }

  fn try_parse_with_remaining(path: &str, _consumed: usize) -> Result<(Self, &str), ParseError> {
    // 简化实现，暂时不支持剩余路径
    Self::try_parse(path).map(|route| (route, ""))
  }
}

impl RouterMatch for ApiV1SubRouterMatch {
  fn try_parse(path: &str) -> Result<Self, ParseError> {
    if let Ok(user) = ApiV1UserRoute::parse(path) {
      return Ok(ApiV1SubRouterMatch::User(user));
    }
    if let Ok(post) = ApiV1PostRoute::parse(path) {
      return Ok(ApiV1SubRouterMatch::Post(post));
    }
    Err(ParseError::invalid_path("No matching API v1 sub route"))
  }

  fn format(&self) -> String {
    match self {
      ApiV1SubRouterMatch::User(route) => route.format(),
      ApiV1SubRouterMatch::Post(route) => route.format(),
    }
  }

  fn patterns() -> Vec<&'static str> {
    vec![ApiV1UserRoute::pattern(), ApiV1PostRoute::pattern()]
  }

  fn try_parse_with_remaining(path: &str, _consumed: usize) -> Result<(Self, &str), ParseError> {
    Self::try_parse(path).map(|route| (route, ""))
  }
}

impl RouterMatch for ApiV2SubRouterMatch {
  fn try_parse(path: &str) -> Result<Self, ParseError> {
    if let Ok(user) = ApiV2UserRoute::parse(path) {
      return Ok(ApiV2SubRouterMatch::User(user));
    }
    Err(ParseError::invalid_path("No matching API v2 sub route"))
  }

  fn format(&self) -> String {
    match self {
      ApiV2SubRouterMatch::User(route) => route.format(),
    }
  }

  fn patterns() -> Vec<&'static str> {
    vec![ApiV2UserRoute::pattern()]
  }

  fn try_parse_with_remaining(path: &str, _consumed: usize) -> Result<(Self, &str), ParseError> {
    Self::try_parse(path).map(|route| (route, ""))
  }
}

impl RouterMatch for ApiVersionRouterMatch {
  fn try_parse(path: &str) -> Result<Self, ParseError> {
    if let Ok(v1) = ApiV1SubRouterMatch::try_parse(path) {
      return Ok(ApiVersionRouterMatch::V1(v1));
    }
    if let Ok(v2) = ApiV2SubRouterMatch::try_parse(path) {
      return Ok(ApiVersionRouterMatch::V2(v2));
    }
    Err(ParseError::invalid_path("No matching API version route"))
  }

  fn format(&self) -> String {
    match self {
      ApiVersionRouterMatch::V1(route) => route.format(),
      ApiVersionRouterMatch::V2(route) => route.format(),
    }
  }

  fn patterns() -> Vec<&'static str> {
    let mut patterns = Vec::new();
    patterns.extend(ApiV1SubRouterMatch::patterns());
    patterns.extend(ApiV2SubRouterMatch::patterns());
    patterns
  }

  fn try_parse_with_remaining(path: &str, _consumed: usize) -> Result<(Self, &str), ParseError> {
    Self::try_parse(path).map(|route| (route, ""))
  }
}

impl RouterMatch for AppRouterMatch {
  fn try_parse(path: &str) -> Result<Self, ParseError> {
    if let Ok(user) = UserRoute::parse(path) {
      return Ok(AppRouterMatch::User(user));
    }
    if let Ok(user_sub) = UserSubRouterMatch::try_parse(path) {
      return Ok(AppRouterMatch::UserSub(user_sub));
    }
    if let Ok(blog) = BlogRoute::parse(path) {
      return Ok(AppRouterMatch::Blog(blog));
    }
    if let Ok(api) = ApiVersionRouterMatch::try_parse(path) {
      return Ok(AppRouterMatch::Api(api));
    }
    Err(ParseError::invalid_path("No matching route"))
  }

  fn format(&self) -> String {
    match self {
      AppRouterMatch::User(route) => route.format(),
      AppRouterMatch::UserSub(route) => route.format(),
      AppRouterMatch::Blog(route) => route.format(),
      AppRouterMatch::Api(route) => route.format(),
    }
  }

  fn patterns() -> Vec<&'static str> {
    let mut patterns = Vec::new();
    patterns.push(UserRoute::pattern());
    patterns.extend(UserSubRouterMatch::patterns());
    patterns.push(BlogRoute::pattern());
    patterns.extend(ApiVersionRouterMatch::patterns());
    patterns
  }

  fn try_parse_with_remaining(path: &str, _consumed: usize) -> Result<(Self, &str), ParseError> {
    Self::try_parse(path).map(|route| (route, ""))
  }
}

// ===== 辅助函数 =====

/// 解析完整的嵌套路由
fn parse_nested_route(url: &str) -> Result<AppRouterMatch, Box<dyn std::error::Error>> {
  AppRouterMatch::try_parse(url).map_err(|e| e.into())
}

/// 格式化嵌套路由为 URL
fn format_nested_route(route_match: &AppRouterMatch) -> String {
  route_match.format()
}

/// 显示路由匹配信息
fn display_route_match_info(route_match: &AppRouterMatch, description: &str) {
  println!("\n=== {description} ===");
  println!("Route Match: {route_match:?}");
  println!("Formatted URL: {}", route_match.format());

  match route_match {
    AppRouterMatch::User(user_route) => {
      println!("User ID: {}", user_route.id);
      println!("Query: {:?}", user_route.query);
    }
    AppRouterMatch::UserSub(user_sub) => match user_sub {
      UserSubRouterMatch::Profile(profile) => {
        println!("User Profile - User ID: {}", profile.user_id);
        println!("Query: {:?}", profile.query);
      }
      UserSubRouterMatch::Post(post) => {
        println!("User Post - User ID: {}, Post ID: {}", post.user_id, post.post_id);
        println!("Query: {:?}", post.query);
      }
      UserSubRouterMatch::Settings(settings) => {
        println!("User Settings - User ID: {}", settings.user_id);
      }
    },
    AppRouterMatch::Blog(blog) => {
      println!("Blog - Year: {}, Month: {}, Slug: {}", blog.year, blog.month, blog.slug);
      println!("Query: {:?}", blog.query);
    }
    AppRouterMatch::Api(api_version) => match api_version {
      ApiVersionRouterMatch::V1(v1_sub) => match v1_sub {
        ApiV1SubRouterMatch::User(user) => {
          println!("API v1 User - ID: {}", user.id);
          println!("Query: {:?}", user.query);
        }
        ApiV1SubRouterMatch::Post(post) => {
          println!("API v1 Post - ID: {}", post.id);
          println!("Query: {:?}", post.query);
        }
      },
      ApiVersionRouterMatch::V2(v2_sub) => match v2_sub {
        ApiV2SubRouterMatch::User(user) => {
          println!("API v2 User - ID: {}", user.id);
          println!("Query: {:?}", user.query);
        }
      },
    },
  }
}

/// 演示路由构建
fn demonstrate_route_building() {
  println!("\n=== 路由构建演示 ===");

  // 构建用户路由
  let user_route = UserRoute {
    id: 123,
    query: UserQuery {
      include_fields: vec!["name".to_string(), "email".to_string()],
      include_sensitive: Some(false),
      format: Some("json".to_string()),
    },
  };
  let user_match = AppRouterMatch::User(user_route);
  println!("用户路由: {}", user_match.format());

  // 构建用户文章路由
  let user_post_route = UserPostRoute {
    user_id: 456,
    post_id: 789,
    query: PostQuery {
      include_comments: Some(true),
      comment_sort: Some("date".to_string()),
      comment_page: Some(1),
    },
  };
  let user_post_match = AppRouterMatch::UserSub(UserSubRouterMatch::Post(user_post_route));
  println!("用户文章路由: {}", user_post_match.format());

  // 构建 API 路由
  let api_user_route = ApiV1UserRoute {
    id: 999,
    query: ApiQuery {
      version: Some("1.0".to_string()),
      debug: Some(true),
      format: Some("xml".to_string()),
    },
  };
  let api_match = AppRouterMatch::Api(ApiVersionRouterMatch::V1(ApiV1SubRouterMatch::User(api_user_route)));
  println!("API 路由: {}", api_match.format());
}

fn main() {
  println!("嵌套路由使用示例 - RouterMatch 设计");
  println!("=====================================\n");

  // 显示所有可能的路由模式
  println!("支持的路由模式:");
  for pattern in AppRouterMatch::patterns() {
    println!("  - {pattern}");
  }

  // 测试各种路由解析
  let test_urls = vec![
    "/users/123?include=name&include=email&format=json",
    "/users/456/profile?include_sensitive=false",
    "/users/789/posts/101?include_comments=true&comment_sort=date",
    "/users/999/settings",
    "/blog/2024/03/rust-tutorial?include_comments=true",
    "/api/v1/users/555?debug=true&format=xml",
    "/api/v1/posts/777?version=1.2",
    "/api/v2/users/888?format=json",
  ];

  for url in test_urls {
    match parse_nested_route(url) {
      Ok(route_match) => {
        display_route_match_info(&route_match, &format!("解析 URL: {url}"));
      }
      Err(e) => {
        println!("\n解析失败 - URL: {url}");
        println!("错误: {e}");
      }
    }
  }

  // 演示路由构建
  demonstrate_route_building();

  // 演示路由模式匹配
  println!("\n=== 路由模式匹配演示 ===");
  let test_url = "/api/v1/users/123?debug=true";
  match parse_nested_route(test_url) {
    Ok(route_match) => {
      println!("成功解析: {test_url}");

      // 演示嵌套匹配
      match &route_match {
        AppRouterMatch::Api(api_version) => {
          println!("这是一个 API 路由");
          match api_version {
            ApiVersionRouterMatch::V1(v1_sub) => {
              println!("API 版本: v1");
              match v1_sub {
                ApiV1SubRouterMatch::User(user) => {
                  println!("资源类型: 用户");
                  println!("用户 ID: {}", user.id);
                  if user.query.debug == Some(true) {
                    println!("调试模式已启用");
                  }
                }
                ApiV1SubRouterMatch::Post(_) => {
                  println!("资源类型: 文章");
                }
              }
            }
            ApiVersionRouterMatch::V2(_) => {
              println!("API 版本: v2");
            }
          }
        }
        _ => println!("非 API 路由"),
      }
    }
    Err(e) => {
      println!("解析失败: {e}");
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_user_route_parsing() {
    let url = "/users/123?include_fields=name&format=json";
    let route_match = parse_nested_route(url).unwrap();

    match route_match {
      AppRouterMatch::User(user_route) => {
        assert_eq!(user_route.id, 123);
        assert_eq!(user_route.query.include_fields, vec!["name"]);
        assert_eq!(user_route.query.format, Some("json".to_string()));
      }
      _ => panic!("Expected User route"),
    }
  }

  #[test]
  fn test_nested_user_post_route() {
    let url = "/users/456/posts/789?include_comments=true";
    let route_match = parse_nested_route(url).unwrap();

    match route_match {
      AppRouterMatch::UserSub(UserSubRouterMatch::Post(post_route)) => {
        assert_eq!(post_route.user_id, 456);
        assert_eq!(post_route.post_id, 789);
        assert_eq!(post_route.query.include_comments, Some(true));
      }
      _ => panic!("Expected UserSub::Post route"),
    }
  }

  #[test]
  fn test_api_v1_user_route() {
    let url = "/api/v1/users/999?debug=true";
    let route_match = parse_nested_route(url).unwrap();

    match route_match {
      AppRouterMatch::Api(ApiVersionRouterMatch::V1(ApiV1SubRouterMatch::User(user_route))) => {
        assert_eq!(user_route.id, 999);
        assert_eq!(user_route.query.debug, Some(true));
      }
      _ => panic!("Expected API v1 User route"),
    }
  }

  #[test]
  fn test_route_formatting() {
    let user_route = UserRoute {
      id: 123,
      query: UserQuery {
        include_fields: vec!["name".to_string()],
        include_sensitive: None,
        format: Some("json".to_string()),
      },
    };
    let route_match = AppRouterMatch::User(user_route);
    let formatted = route_match.format();

    assert!(formatted.contains("/users/123"));
    assert!(formatted.contains("include_fields=name"));
    assert!(formatted.contains("format=json"));
  }

  #[test]
  fn test_route_patterns() {
    let patterns = AppRouterMatch::patterns();

    assert!(patterns.contains(&"/users/:id"));
    assert!(patterns.contains(&"/users/:user_id/posts/:post_id"));
    assert!(patterns.contains(&"/api/v1/users/:id"));
    assert!(patterns.contains(&"/blog/:year/:month/:slug"));
  }

  #[test]
  fn test_invalid_route() {
    let url = "/invalid/route/path";
    let result = parse_nested_route(url);

    assert!(result.is_err());
  }
}
