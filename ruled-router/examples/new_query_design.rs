//! 新的 Query 设计示例
//!
//! 演示新的查询设计：
//! 1. 在路由结构体中使用 #[route(query)] 定义查询字段
//! 2. 使用 #[querystring] 修饰查询结构体自动实现解析
//! 3. 支持多层查询合并和统一格式化

use ruled_router::{QueryString, Router};

/// 用户查询参数
#[derive(Debug, Clone, PartialEq, Default, QueryString)]
struct UserQuery {
  tab: Option<String>,
  active: Option<bool>,
}

/// 用户路由 - 在路由中定义查询字段
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/users/:id")] // query = "UserQuery" 是新设计，待实现
struct UserRoute {
  id: u32,
  // 查询参数字段，由 #[router(query)] 指定类型（新设计中会自动添加）
  // query: UserQuery, // 暂时注释，等待实现
}

/// 搜索查询参数
#[derive(Debug, Clone, PartialEq, Default, QueryString)]
struct SearchQuery {
  q: Option<String>,
  page: Option<u32>,
  limit: Option<u32>,
  tags: Vec<String>,
}

/// 博客路由 - 支持多个路径参数和查询
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/blog/:category/:slug")] // query = "SearchQuery" 是新设计，待实现
struct BlogRoute {
  category: String,
  slug: String,
  // query: SearchQuery, // 暂时注释，等待实现
}

/// 过滤查询参数
#[derive(Debug, Clone, PartialEq, Default, QueryString)]
struct FilterQuery {
  sort_by: Option<String>,
  order: Option<String>,
  active: Option<bool>,
}

/// API 路由 - 复杂的嵌套路由，每层可以有自己的查询
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/api/:version/users/:user_id")] // query = "FilterQuery" 是新设计，待实现
struct ApiUserRoute {
  version: String,
  user_id: u32,
  // query: FilterQuery, // 暂时注释，等待实现
}

/// 文章查询参数
#[derive(Debug, Clone, PartialEq, Default, QueryString)]
struct PostQuery {
  include_draft: Option<bool>,
  format: Option<String>,
}

/// API 文章路由 - 继承上层路由并添加自己的查询
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/api/:version/users/:user_id/posts/:post_id")] // query = "PostQuery" 是新设计，待实现
struct ApiPostRoute {
  version: String,
  user_id: u32,
  post_id: u32,
  // query: PostQuery, // 暂时注释，等待实现
}

/// 组合查询 - 演示多个查询参数的合并
#[derive(Debug, Clone, PartialEq, Default, QueryString)]
struct CombinedQuery {
  // 从 FilterQuery 继承
  sort_by: Option<String>,
  order: Option<String>,
  active: Option<bool>,
  // 从 SearchQuery 继承
  q: Option<String>,
  page: Option<u32>,
  limit: Option<u32>,
  tags: Vec<String>,
  // 自己的参数
  format: Option<String>,
}

/// 复杂路由 - 演示查询合并
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/search/:category")] // query = "CombinedQuery" 是新设计，待实现
struct SearchRoute {
  category: String,
  // query: CombinedQuery, // 暂时注释，等待实现
}

fn main() {
  println!("=== 新的 Query 设计演示 ===");

  // 1. 基本路由和查询解析
  println!("\n1. 基本路由和查询解析:");

  // 用户路由示例
  println!("\n  用户路由:");
  let user_url = "/users/123?tab=profile&active=true";
  println!("    URL: {user_url}");

  // 模拟解析过程（实际实现中会自动处理）
  let user_route = UserRoute {
    id: 123,
    // query 字段在新设计中会自动添加
  };
  let user_query = UserQuery {
    tab: Some("profile".to_string()),
    active: Some(true),
  };
  println!("    解析结果: {user_route:?}");
  println!("    查询结果: {user_query:?}");

  // 模拟格式化过程
  let formatted_url = format!(
    "/users/{}?tab={}&active={}",
    user_route.id,
    user_query.tab.as_ref().unwrap(),
    user_query.active.unwrap()
  );
  println!("    格式化: {formatted_url}");

  // 2. 博客路由示例
  println!("\n  博客路由:");
  let blog_url = "/blog/technology/rust-tutorial?q=advanced&page=1&tags=async&tags=performance";
  println!("    URL: {blog_url}");

  let blog_route = BlogRoute {
    category: "technology".to_string(),
    slug: "rust-tutorial".to_string(),
    // query 字段在新设计中会自动添加
  };
  let blog_query = SearchQuery {
    q: Some("advanced".to_string()),
    page: Some(1),
    limit: None,
    tags: vec!["async".to_string(), "performance".to_string()],
  };
  println!("    解析结果: {blog_route:?}");
  println!("    查询结果: {blog_query:?}");

  // 3. 多层查询合并演示
  println!("\n2. 多层查询合并演示:");

  // 模拟多层路由的查询合并
  println!("\n  场景: API 用户路由 + API 文章路由");

  // 第一层：API 用户路由
  let api_user_route = ApiUserRoute {
    version: "v1".to_string(),
    user_id: 456,
    // query 字段在新设计中会自动添加
  };
  let api_user_query = FilterQuery {
    sort_by: Some("date".to_string()),
    order: Some("desc".to_string()),
    active: Some(true),
  };
  println!("    第一层 (API 用户): {api_user_route:?}");
  println!("    第一层查询: {api_user_query:?}");

  // 第二层：API 文章路由
  let api_post_route = ApiPostRoute {
    version: "v1".to_string(),
    user_id: 456,
    post_id: 789,
    // query 字段在新设计中会自动添加
  };
  let api_post_query = PostQuery {
    include_draft: Some(false),
    format: Some("json".to_string()),
  };
  println!("    第二层 (API 文章): {api_post_route:?}");
  println!("    第二层查询: {api_post_query:?}");

  // 模拟合并后的查询字符串
  let merged_query = "sort_by=date&order=desc&active=true&include_draft=false&format=json";
  println!("    合并后查询: {merged_query}");

  let final_url = format!("/api/v1/users/456/posts/789?{merged_query}");
  println!("    最终 URL: {final_url}");

  // 4. 复杂查询合并
  println!("\n3. 复杂查询合并演示:");

  let search_route = SearchRoute {
    category: "tutorials".to_string(),
    // query 字段在新设计中会自动添加
  };
  let combined_query = CombinedQuery {
    // 过滤参数
    sort_by: Some("popularity".to_string()),
    order: Some("desc".to_string()),
    active: Some(true),
    // 搜索参数
    q: Some("rust async".to_string()),
    page: Some(2),
    limit: Some(20),
    tags: vec!["rust".to_string(), "async".to_string(), "tutorial".to_string()],
    // 自定义参数
    format: Some("detailed".to_string()),
  };

  println!("    复杂路由: {search_route:?}");
  println!("    复杂查询: {combined_query:?}");

  // 模拟格式化的查询字符串
  let complex_query =
    "sort_by=popularity&order=desc&active=true&q=rust%20async&page=2&limit=20&tags=rust&tags=async&tags=tutorial&format=detailed";
  let complex_url = format!("/search/tutorials?{complex_query}");
  println!("    格式化 URL: {complex_url}");

  // 5. 查询冲突处理演示
  println!("\n4. 查询冲突处理演示:");

  println!("    场景: 上层定义 active=true，下层定义 active=false");
  println!("    策略: 使用上层的值 (active=true)");

  // 6. 中间结构演示
  println!("\n5. 中间结构处理演示:");

  println!("    原始查询字符串: sort_by=date&active=true&tags=rust&tags=web&page=1");
  println!("    中间结构 (HashMap): {{");
  println!("        \"sort_by\": [\"date\"],");
  println!("        \"active\": [\"true\"],");
  println!("        \"tags\": [\"rust\", \"web\"],");
  println!("        \"page\": [\"1\"]");
  println!("    }}");

  println!("    提取到具体结构:");
  println!("        FilterQuery {{ sort_by: Some(\"date\"), active: Some(true), .. }}");
  println!("        SearchQuery {{ page: Some(1), tags: [\"rust\", \"web\"], .. }}");

  println!("\n=== 新设计演示完成 ===");

  println!("\n=== 设计要点总结 ===");
  println!("1. 路由结构体使用 #[router(pattern = \"...\", query = \"QueryType\")] 定义查询类型");
  println!("2. 查询结构体使用 #[querystring] 自动实现解析和格式化");
  println!("3. 支持多层路由的查询参数合并，上层优先");
  println!("4. 查询解析通过中间结构 (HashMap<String, Vec<String>>) 进行");
  println!("5. 最终格式化时将所有层的查询参数合并成统一的查询字符串");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_user_route_structure() {
    let route = UserRoute {
      id: 123,
      // query 字段在新设计中会自动添加
    };
    let query = UserQuery {
      tab: Some("profile".to_string()),
      active: Some(true),
    };

    assert_eq!(route.id, 123);
    assert_eq!(query.tab, Some("profile".to_string()));
    assert_eq!(query.active, Some(true));
  }

  #[test]
  fn test_blog_route_structure() {
    let route = BlogRoute {
      category: "tech".to_string(),
      slug: "rust-guide".to_string(),
      // query 字段在新设计中会自动添加
    };
    let query = SearchQuery {
      q: Some("advanced".to_string()),
      page: Some(1),
      limit: Some(10),
      tags: vec!["rust".to_string()],
    };

    assert_eq!(route.category, "tech");
    assert_eq!(route.slug, "rust-guide");
    assert_eq!(query.q, Some("advanced".to_string()));
    assert_eq!(query.tags, vec!["rust".to_string()]);
  }

  #[test]
  fn test_combined_query_structure() {
    let query = CombinedQuery {
      sort_by: Some("date".to_string()),
      order: Some("desc".to_string()),
      active: Some(true),
      q: Some("rust".to_string()),
      page: Some(1),
      limit: Some(20),
      tags: vec!["tutorial".to_string(), "beginner".to_string()],
      format: Some("json".to_string()),
    };

    // 验证所有字段都正确设置
    assert_eq!(query.sort_by, Some("date".to_string()));
    assert_eq!(query.q, Some("rust".to_string()));
    assert_eq!(query.tags.len(), 2);
    assert_eq!(query.format, Some("json".to_string()));
  }
}
