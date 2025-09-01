//! 性能测试
//!
//! 测试 ruled-router 库的性能特性

use ruled_router::prelude::*;
use ruled_router_derive::QueryDerive;
use std::time::Instant;

/// 简单的产品路由用于性能测试
#[derive(Debug, Clone, PartialEq, RouterData)]
#[router(pattern = "/products/:category/:id")]
struct ProductRoute {
  category: String,
  id: u32,
}

/// 分页查询用于性能测试
#[derive(Debug, Clone, PartialEq, Default, QueryDerive)]
struct PaginationQuery {
  page: Option<u32>,
  limit: Option<u32>,
  sort: Option<String>,
  filter: Vec<String>,
}

/// 性能测试器
struct PerformanceTester {
  test_routes: Vec<String>,
  test_queries: Vec<String>,
}

impl PerformanceTester {
  fn new() -> Self {
    let test_routes = (1..=1000).map(|i| format!("/products/category{}/{}", i % 10, i)).collect();

    let test_queries = (1..=1000)
      .map(|i| format!("page={}&limit=10&sort=name&filter=active&filter=featured", i % 100))
      .collect();

    Self { test_routes, test_queries }
  }

  fn benchmark_route_parsing(&self) -> (u128, usize) {
    let start = Instant::now();
    let mut success_count = 0;

    for route_str in &self.test_routes {
      if ProductRoute::parse(route_str).is_ok() {
        success_count += 1;
      }
    }

    (start.elapsed().as_nanos(), success_count)
  }

  fn benchmark_route_formatting(&self) -> (u128, usize) {
    let routes: Vec<ProductRoute> = (1..=1000)
      .map(|i| ProductRoute {
        category: format!("category{}", i % 10),
        id: i,
      })
      .collect();

    let start = Instant::now();
    let mut success_count = 0;

    for route in &routes {
      let _formatted = route.format();
      success_count += 1;
    }

    (start.elapsed().as_nanos(), success_count)
  }

  fn benchmark_query_parsing(&self) -> (u128, usize) {
    let start = Instant::now();
    let mut success_count = 0;

    for query_str in &self.test_queries {
      if PaginationQuery::parse(query_str).is_ok() {
        success_count += 1;
      }
    }

    (start.elapsed().as_nanos(), success_count)
  }

  fn benchmark_query_formatting(&self) -> (u128, usize) {
    let queries: Vec<PaginationQuery> = (1..=1000)
      .map(|i| PaginationQuery {
        page: Some(i % 100),
        limit: Some(10),
        sort: Some("name".to_string()),
        filter: vec!["active".to_string(), "featured".to_string()],
      })
      .collect();

    let start = Instant::now();
    let mut success_count = 0;

    for query in &queries {
      let _formatted = query.format();
      success_count += 1;
    }

    (start.elapsed().as_nanos(), success_count)
  }

  fn benchmark_roundtrip(&self) -> (u128, usize) {
    let start = Instant::now();
    let mut success_count = 0;

    for route_str in &self.test_routes {
      if let Ok(route) = ProductRoute::parse(route_str) {
        let formatted = route.format();
        if ProductRoute::parse(&formatted).is_ok() {
          success_count += 1;
        }
      }
    }

    (start.elapsed().as_nanos(), success_count)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_performance_benchmarks() {
    let tester = PerformanceTester::new();

    // 测试路由解析性能
    let (route_parse_time, route_parse_count) = tester.benchmark_route_parsing();
    assert_eq!(route_parse_count, 1000);
    println!(
      "Route parsing: {} ns total, {} ns per operation",
      route_parse_time,
      route_parse_time / 1000
    );

    // 测试路由格式化性能
    let (route_format_time, route_format_count) = tester.benchmark_route_formatting();
    assert_eq!(route_format_count, 1000);
    println!(
      "Route formatting: {} ns total, {} ns per operation",
      route_format_time,
      route_format_time / 1000
    );

    // 测试查询解析性能
    let (query_parse_time, query_parse_count) = tester.benchmark_query_parsing();
    assert_eq!(query_parse_count, 1000);
    println!(
      "Query parsing: {} ns total, {} ns per operation",
      query_parse_time,
      query_parse_time / 1000
    );

    // 测试查询格式化性能
    let (query_format_time, query_format_count) = tester.benchmark_query_formatting();
    assert_eq!(query_format_count, 1000);
    println!(
      "Query formatting: {} ns total, {} ns per operation",
      query_format_time,
      query_format_time / 1000
    );

    // 测试往返性能
    let (roundtrip_time, roundtrip_count) = tester.benchmark_roundtrip();
    assert_eq!(roundtrip_count, 1000);
    println!("Roundtrip: {} ns total, {} ns per operation", roundtrip_time, roundtrip_time / 1000);
  }

  #[test]
  fn test_product_route_performance() {
    let routes = vec!["/products/electronics/123", "/products/books/456", "/products/clothing/789"];

    let start = Instant::now();
    for route_str in &routes {
      let route = ProductRoute::parse(route_str).unwrap();
      let _formatted = route.format();
    }
    let elapsed = start.elapsed();

    println!("Processed {} routes in {:?}", routes.len(), elapsed);
    assert!(elapsed.as_millis() < 100); // 应该很快完成
  }

  #[test]
  fn test_pagination_query_performance() {
    let queries = vec![
      "page=1&limit=10&sort=name",
      "page=2&limit=20&sort=date&filter=active",
      "page=3&limit=50&filter=featured&filter=popular",
    ];

    let start = Instant::now();
    for query_str in &queries {
      let query = PaginationQuery::parse(query_str).unwrap();
      let _formatted = query.format();
    }
    let elapsed = start.elapsed();

    println!("Processed {} queries in {:?}", queries.len(), elapsed);
    assert!(elapsed.as_millis() < 100); // 应该很快完成
  }

  #[test]
  fn test_memory_efficiency() {
    // 测试大量路由对象的内存使用
    let routes: Vec<ProductRoute> = (1..=10000)
      .map(|i| ProductRoute {
        category: format!("cat{}", i % 100),
        id: i,
      })
      .collect();

    assert_eq!(routes.len(), 10000);

    // 测试格式化不会导致内存泄漏
    for route in routes.iter().take(100) {
      let _formatted = route.format();
    }
  }
}
