use ruled_router::prelude::*;

// Define route structure
#[derive(Router)]
#[router(pattern = "/users/:id")] // Define path pattern only once
struct UserRoute {
  id: u32,
  #[query]
  query: UserQuery,
}

// Define query parameters
#[derive(Query)]
struct UserQuery {
  #[query(name = "tab")]
  tab: Option<String>,
  #[query(name = "page", default = "1")]
  page: u32,
}

fn main() {
  // Parse route
  let path = "/users/123?tab=profile&page=2";
  let route = UserRoute::parse(path).unwrap();

  println!("用户ID: {}", route.id);
  println!("标签页: {:?}", route.query.tab);
  println!("页码: {}", route.query.page);

  // Format route
  let formatted = route.format();
  println!("格式化结果: {formatted}");
  // Output: /users/123?tab=profile&page=2
}
