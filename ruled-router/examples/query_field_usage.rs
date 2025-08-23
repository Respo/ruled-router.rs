use ruled_router::prelude::*;

#[derive(Debug, PartialEq, Router)]
#[router(pattern = "/users/{id}")]
struct UserRoute {
  id: u32,
  #[query]
  query: UserQuery,
}

#[derive(Debug, PartialEq, Query)]
struct UserQuery {
  #[query(rename = "include")]
  include_fields: Option<String>,
  page: Option<u32>,
  limit: Option<u32>,
}

#[derive(Debug, PartialEq, Router)]
#[router(pattern = "/api/posts/{post_id}/comments")]
struct PostCommentsRoute {
  post_id: String,
  #[query]
  query: CommentQuery,
}

#[derive(Debug, PartialEq, Query)]
struct CommentQuery {
  author: Option<String>,
  #[query(rename = "sort_by")]
  sort_field: Option<String>,
  order: Option<String>,
}

fn main() {
  println!("=== Query Field Usage Examples ===");

  // 示例 1: 用户路由与查询参数
  println!("\n1. User Route with Query Parameters:");

  let user_url = "/users/123?include=profile&page=2&limit=10";
  match UserRoute::parse(user_url) {
    Ok(route) => {
      println!("  Parsed: {route:?}");
      println!("  User ID: {}", route.id);
      println!("  Include: {:?}", route.query.include_fields);
      println!("  Page: {:?}", route.query.page);
      println!("  Limit: {:?}", route.query.limit);

      // 测试往返转换
      let reconstructed = route.format();
      println!("  Reconstructed: {reconstructed}");
    }
    Err(e) => println!("  Parse error: {e:?}"),
  }

  // 示例 2: 博客评论路由
  println!("\n2. Post Comments Route:");

  let comments_url = "/api/posts/hello-world/comments?author=john&sort_by=date&order=desc";
  match PostCommentsRoute::parse(comments_url) {
    Ok(route) => {
      println!("  Parsed: {route:?}");
      println!("  Post ID: {}", route.post_id);
      println!("  Author filter: {:?}", route.query.author);
      println!("  Sort by: {:?}", route.query.sort_field);
      println!("  Order: {:?}", route.query.order);

      let reconstructed = route.format();
      println!("  Reconstructed: {reconstructed}");
    }
    Err(e) => println!("  Parse error: {e:?}"),
  }

  // 示例 3: 测试部分查询参数
  println!("\n3. Partial Query Parameters:");

  let partial_url = "/users/456?page=1";
  match UserRoute::parse(partial_url) {
    Ok(route) => {
      println!("  Parsed: {route:?}");
      println!("  Only page specified: {:?}", route.query.page);

      let reconstructed = route.format();
      println!("  Reconstructed: {reconstructed}");
    }
    Err(e) => println!("  Parse error: {e:?}"),
  }

  // 示例 4: 手动构建路由
  println!("\n4. Manual Route Construction:");

  let manual_route = UserRoute {
    id: 789,
    query: UserQuery {
      include_fields: Some("avatar,email".to_string()),
      page: Some(3),
      limit: Some(20),
    },
  };

  let manual_url = manual_route.format();
  println!("  Manual route: {manual_route:?}");
  println!("  Generated URL: {manual_url}");

  // 验证往返转换
  match UserRoute::parse(&manual_url) {
    Ok(parsed) => {
      println!("  Round-trip successful: {}", parsed == manual_route);
    }
    Err(e) => println!("  Round-trip failed: {e:?}"),
  }

  println!("\n=== Tests ===");

  // 运行一些基本测试
  test_user_route();
  test_post_comments_route();
  test_empty_query();

  println!("\nAll tests completed!");
}

fn test_user_route() {
  println!("\nTesting UserRoute...");

  let url = "/users/42?include=profile,settings&page=1&limit=50";
  let route = UserRoute::parse(url).expect("Should parse successfully");

  assert_eq!(route.id, 42);
  assert_eq!(route.query.include_fields, Some("profile,settings".to_string()));
  assert_eq!(route.query.page, Some(1));
  assert_eq!(route.query.limit, Some(50));

  let reconstructed = route.format();
  let reparsed = UserRoute::parse(&reconstructed).expect("Should reparse successfully");
  assert_eq!(route, reparsed);

  println!("  ✓ UserRoute tests passed");
}

fn test_post_comments_route() {
  println!("\nTesting PostCommentsRoute...");

  let url = "/api/posts/my-first-post/comments?author=alice&sort_by=created_at&order=asc";
  let route = PostCommentsRoute::parse(url).expect("Should parse successfully");

  assert_eq!(route.post_id, "my-first-post");
  assert_eq!(route.query.author, Some("alice".to_string()));
  assert_eq!(route.query.sort_field, Some("created_at".to_string()));
  assert_eq!(route.query.order, Some("asc".to_string()));

  let reconstructed = route.format();
  let reparsed = PostCommentsRoute::parse(&reconstructed).expect("Should reparse successfully");
  assert_eq!(route, reparsed);

  println!("  ✓ PostCommentsRoute tests passed");
}

fn test_empty_query() {
  println!("\nTesting empty query parameters...");

  let url = "/users/100";
  let route = UserRoute::parse(url).expect("Should parse successfully");

  assert_eq!(route.id, 100);
  assert_eq!(route.query.include_fields, None);
  assert_eq!(route.query.page, None);
  assert_eq!(route.query.limit, None);

  println!("  ✓ Empty query tests passed");
}
