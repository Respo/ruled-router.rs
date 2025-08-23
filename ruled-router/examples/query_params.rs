use ruled_router::prelude::*;

#[derive(Router)]
#[router(pattern = "/search")]
struct SearchRoute {
  #[query]
  query: ListQuery,
}

#[derive(Debug, Query)]
struct ListQuery {
  #[query(name = "page", default = "1")]
  page: u32,

  #[query(name = "limit", default = "20")]
  limit: u32,

  #[query(name = "sort")]
  sort: Option<String>,

  #[query(name = "filter", multiple)]
  filters: Vec<String>,

  #[query(name = "q")]
  search_term: Option<String>,
}

fn main() {
  let test_paths = [
    "/search", // Default values
    "/search?page=2&limit=50",
    "/search?q=rust&sort=Desc",
    "/search?filter=tech&filter=programming&filter=rust",
    "/search?page=3&limit=10&q=router&sort=Asc&filter=web&filter=framework",
  ];

  for path in test_paths {
    match SearchRoute::parse(path) {
      Ok(route) => {
        let query = &route.query;
        println!("\n路径: {path}");
        println!("  页码: {}", query.page);
        println!("  限制: {}", query.limit);
        println!("  排序: {:?}", query.sort);
        println!("  搜索词: {:?}", query.search_term);
        println!("  过滤器: {:?}", query.filters);

        // Format back to URL
        let formatted = route.format();
        println!("  格式化: {formatted}");
      }
      Err(e) => {
        println!("解析失败: {path} -> {e:?}");
      }
    }
  }
}
