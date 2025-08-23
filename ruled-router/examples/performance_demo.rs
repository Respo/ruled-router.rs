//! 性能演示示例
//!
//! 演示 ruled-router 库的性能特性，包括：
//! - 批量路由解析
//! - 缓存和重用
//! - 内存使用优化
//! - 错误处理性能

use ruled_router::{
  error::ParseError,
  formatter::{PathFormatter, QueryFormatter},
  parser::{PathParser, QueryParser},
  traits::{Query, Router},
};
use std::collections::HashMap;
use std::time::Instant;

/// 简单的产品路由
#[derive(Debug, Clone, PartialEq)]
struct ProductRoute {
  category: String,
  id: u32,
}

impl Router for ProductRoute {
  type SubRouterMatch = ::ruled_router::NoSubRouter;

  fn parse(path: &str) -> Result<Self, ParseError> {
    let (path_part, _) = ruled_router::utils::split_path_query(path);
    let parser = PathParser::new("/products/:category/:id")?;
    let params = parser.match_path(path_part)?;

    let category = params
      .get("category")
      .ok_or_else(|| ParseError::missing_parameter("category"))?
      .clone();
    let id = params
      .get("id")
      .ok_or_else(|| ParseError::missing_parameter("id"))?
      .parse::<u32>()
      .map_err(|_| ParseError::type_conversion("Cannot convert id to u32".to_string()))?;

    Ok(Self { category, id })
  }

  fn format(&self) -> String {
    let mut params = HashMap::new();
    params.insert("category".to_string(), self.category.clone());
    params.insert("id".to_string(), self.id.to_string());

    let formatter = PathFormatter::new("/products/:category/:id").unwrap();
    formatter.format(&params).unwrap()
  }

  fn pattern() -> &'static str {
    "/products/:category/:id"
  }
}

/// 分页查询
#[derive(Debug, Clone, PartialEq, Default)]
struct PaginationQuery {
  page: Option<u32>,
  limit: Option<u32>,
  sort: Option<String>,
  filter: Vec<String>,
}

impl Query for PaginationQuery {
  fn parse(query: &str) -> Result<Self, ParseError> {
    let parser = QueryParser::new(query)?;

    Ok(Self {
      page: parser.get_optional("page")?,
      limit: parser.get_optional("limit")?,
      sort: parser.get_optional("sort")?,
      filter: parser.get_all("filter").iter().map(|s| s.to_string()).collect(),
    })
  }

  fn format(&self) -> String {
    let mut formatter = QueryFormatter::new();

    if let Some(page) = self.page {
      formatter.set("page", page.to_string());
    }

    if let Some(limit) = self.limit {
      formatter.set("limit", limit.to_string());
    }

    if let Some(ref sort) = self.sort {
      formatter.set("sort", sort.clone());
    }

    for filter in &self.filter {
      formatter.add("filter", filter.clone());
    }

    formatter.format()
  }

  fn from_query_map(query_map: &HashMap<String, Vec<String>>) -> Result<Self, ParseError> {
    Ok(Self {
      page: query_map.get("page").and_then(|values| values.first()).and_then(|s| s.parse().ok()),
      limit: query_map
        .get("limit")
        .and_then(|values| values.first())
        .and_then(|s| s.parse().ok()),
      sort: query_map.get("sort").and_then(|values| values.first()).map(|s| s.to_string()),
      filter: query_map
        .get("filter")
        .map(|values| values.iter().map(|s| s.to_string()).collect())
        .unwrap_or_default(),
    })
  }

  fn to_query_string(&self) -> String {
    self.format()
  }
}

/// 性能测试工具
struct PerformanceTester {
  test_routes: Vec<String>,
  test_queries: Vec<String>,
}

impl PerformanceTester {
  fn new() -> Self {
    let test_routes = (1..=1000).map(|i| format!("/products/category{}/{}", i % 10, i)).collect();

    let test_queries = (1..=1000)
      .map(|i| {
        format!(
          "page={}&limit={}&sort=name&filter=active&filter=featured",
          i % 10 + 1,
          (i % 5 + 1) * 10
        )
      })
      .collect();

    Self { test_routes, test_queries }
  }

  fn benchmark_route_parsing(&self) -> (u128, usize) {
    let start = Instant::now();
    let mut success_count = 0;

    for route in &self.test_routes {
      if ProductRoute::parse(route).is_ok() {
        success_count += 1;
      }
    }

    let duration = start.elapsed().as_micros();
    (duration, success_count)
  }

  fn benchmark_route_formatting(&self) -> (u128, usize) {
    // 先解析所有路由
    let routes: Vec<ProductRoute> = self.test_routes.iter().filter_map(|r| ProductRoute::parse(r).ok()).collect();

    let start = Instant::now();
    let mut success_count = 0;

    for route in &routes {
      let formatted = route.format();
      if !formatted.is_empty() {
        success_count += 1;
      }
    }

    let duration = start.elapsed().as_micros();
    (duration, success_count)
  }

  fn benchmark_query_parsing(&self) -> (u128, usize) {
    let start = Instant::now();
    let mut success_count = 0;

    for query in &self.test_queries {
      if PaginationQuery::parse(query).is_ok() {
        success_count += 1;
      }
    }

    let duration = start.elapsed().as_micros();
    (duration, success_count)
  }

  fn benchmark_query_formatting(&self) -> (u128, usize) {
    // 先解析所有查询
    let queries: Vec<PaginationQuery> = self.test_queries.iter().filter_map(|q| PaginationQuery::parse(q).ok()).collect();

    let start = Instant::now();
    let mut success_count = 0;

    for query in &queries {
      let formatted = query.format();
      if !formatted.is_empty() {
        success_count += 1;
      }
    }

    let duration = start.elapsed().as_micros();
    (duration, success_count)
  }

  fn benchmark_roundtrip(&self) -> (u128, usize) {
    let start = Instant::now();
    let mut success_count = 0;

    for (route_str, query_str) in self.test_routes.iter().zip(self.test_queries.iter()) {
      // 解析
      if let (Ok(route), Ok(query)) = (ProductRoute::parse(route_str), PaginationQuery::parse(query_str)) {
        // 格式化
        let formatted_route = route.format();
        let formatted_query = query.format();

        // 重新解析
        if ProductRoute::parse(&formatted_route).is_ok() && PaginationQuery::parse(&formatted_query).is_ok() {
          success_count += 1;
        }
      }
    }

    let duration = start.elapsed().as_micros();
    (duration, success_count)
  }

  fn benchmark_parser_reuse(&self) -> (u128, usize) {
    // 创建可重用的解析器
    let path_parser = PathParser::new("/products/:category/:id").unwrap();

    let start = Instant::now();
    let mut success_count = 0;

    for route in &self.test_routes {
      let (path_part, _) = ruled_router::utils::split_path_query(route);
      if path_parser.match_path(path_part).is_ok() {
        success_count += 1;
      }
    }

    let duration = start.elapsed().as_micros();
    (duration, success_count)
  }
}

fn main() {
  println!("=== ruled-router 性能演示 ===");

  let tester = PerformanceTester::new();

  println!(
    "\n测试数据集: {} 个路由, {} 个查询",
    tester.test_routes.len(),
    tester.test_queries.len()
  );

  // 1. 路由解析性能
  println!("\n1. 路由解析性能:");
  let (duration, success) = tester.benchmark_route_parsing();
  println!("  时间: {duration} 微秒");
  println!("  成功: {}/{}", success, tester.test_routes.len());
  println!("  平均: {:.2} 微秒/次", duration as f64 / tester.test_routes.len() as f64);

  // 2. 路由格式化性能
  println!("\n2. 路由格式化性能:");
  let (duration, success) = tester.benchmark_route_formatting();
  println!("  时间: {duration} 微秒");
  println!("  成功: {success}");
  println!("  平均: {:.2} 微秒/次", duration as f64 / success as f64);

  // 3. 查询解析性能
  println!("\n3. 查询解析性能:");
  let (duration, success) = tester.benchmark_query_parsing();
  println!("  时间: {duration} 微秒");
  println!("  成功: {}/{}", success, tester.test_queries.len());
  println!("  平均: {:.2} 微秒/次", duration as f64 / tester.test_queries.len() as f64);

  // 4. 查询格式化性能
  println!("\n4. 查询格式化性能:");
  let (duration, success) = tester.benchmark_query_formatting();
  println!("  时间: {duration} 微秒");
  println!("  成功: {success}");
  println!("  平均: {:.2} 微秒/次", duration as f64 / success as f64);

  // 5. 往返性能（解析 -> 格式化 -> 重新解析）
  println!("\n5. 往返性能:");
  let (duration, success) = tester.benchmark_roundtrip();
  println!("  时间: {duration} 微秒");
  println!("  成功: {success}");
  println!("  平均: {:.2} 微秒/次", duration as f64 / success as f64);

  // 6. 解析器重用性能
  println!("\n6. 解析器重用性能:");
  let (duration, success) = tester.benchmark_parser_reuse();
  println!("  时间: {duration} 微秒");
  println!("  成功: {}/{}", success, tester.test_routes.len());
  println!("  平均: {:.2} 微秒/次", duration as f64 / tester.test_routes.len() as f64);

  // 7. 内存使用演示
  println!("\n7. 内存使用演示:");

  // 创建大量路由对象
  let routes: Vec<ProductRoute> = (1..=10000)
    .map(|i| ProductRoute {
      category: format!("category{}", i % 100),
      id: i,
    })
    .collect();

  println!("  创建了 {} 个路由对象", routes.len());

  // 批量格式化
  let start = Instant::now();
  let formatted: Vec<String> = routes.iter().map(|r| r.format()).collect();
  let duration = start.elapsed().as_micros();

  println!("  批量格式化时间: {duration} 微秒");
  println!("  平均: {:.2} 微秒/次", duration as f64 / routes.len() as f64);
  println!("  格式化结果数量: {}", formatted.len());

  // 8. 错误处理性能
  println!("\n8. 错误处理性能:");

  let invalid_routes = vec![
    "/invalid/path",
    "/products/category/abc", // 无效 ID
    "/products",              // 路径太短
    "/products/category",     // 缺少 ID
    "/other/path/123",        // 完全不匹配
  ];

  let start = Instant::now();
  let mut error_count = 0;

  for _ in 0..1000 {
    for invalid_route in &invalid_routes {
      if ProductRoute::parse(invalid_route).is_err() {
        error_count += 1;
      }
    }
  }

  let duration = start.elapsed().as_micros();
  println!("  错误处理时间: {duration} 微秒");
  println!("  错误数量: {error_count}");
  println!("  平均: {:.2} 微秒/次", duration as f64 / error_count as f64);

  println!("\n=== 性能演示完成 ===");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_performance_tester() {
    let tester = PerformanceTester::new();
    assert_eq!(tester.test_routes.len(), 1000);
    assert_eq!(tester.test_queries.len(), 1000);
  }

  #[test]
  fn test_product_route_performance() {
    let route = ProductRoute {
      category: "electronics".to_string(),
      id: 123,
    };

    // 测试多次格式化的一致性
    for _ in 0..100 {
      let formatted = route.format();
      assert_eq!(formatted, "/products/electronics/123");

      let parsed = ProductRoute::parse(&formatted).unwrap();
      assert_eq!(parsed, route);
    }
  }

  #[test]
  fn test_pagination_query_performance() {
    let query = PaginationQuery {
      page: Some(1),
      limit: Some(20),
      sort: Some("name".to_string()),
      filter: vec!["active".to_string(), "featured".to_string()],
    };

    // 测试多次格式化的一致性
    for _ in 0..100 {
      let formatted = query.format();
      let parsed = PaginationQuery::parse(&formatted).unwrap();
      assert_eq!(parsed.page, query.page);
      assert_eq!(parsed.limit, query.limit);
      assert_eq!(parsed.sort, query.sort);
      assert_eq!(parsed.filter, query.filter);
    }
  }
}
