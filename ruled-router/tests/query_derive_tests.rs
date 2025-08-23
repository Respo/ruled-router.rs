//! Query derive 宏测试
//!
//! 测试 #[derive(Query)] 宏的各种功能

use ruled_router::prelude::*;

/// 基础查询参数测试
#[derive(Debug, Clone, PartialEq, Default, Query)]
struct SearchQuery {
  q: Option<String>,
  page: Option<u32>,
  limit: Option<u32>,
  tags: Vec<String>,
}

/// 过滤查询参数测试
#[derive(Debug, Clone, PartialEq, Default, Query)]
struct FilterQuery {
  active: Option<bool>,
  sort_by: Option<String>,
  order: Option<String>,
  categories: Vec<String>,
  min_price: Option<f64>,
  max_price: Option<f64>,
}

/// 用户偏好查询测试
#[derive(Debug, Clone, PartialEq, Default, Query)]
struct UserPreferencesQuery {
  theme: Option<String>,
  lang: Option<String>,
  timezone: Option<String>,
  notifications: Option<bool>,
}

/// 分页查询测试
#[derive(Debug, Clone, PartialEq, Default, Query)]
struct PaginationQuery {
  page: Option<u32>,
  per_page: Option<u32>,
  offset: Option<u32>,
}

/// 复杂查询参数测试
#[derive(Debug, Clone, PartialEq, Default, Query)]
struct ComplexQuery {
  // 字符串类型
  query: Option<String>,
  title: Option<String>,
  author: Option<String>,

  // 数字类型
  page: Option<u32>,
  limit: Option<u32>,
  min_rating: Option<f32>,
  max_rating: Option<f32>,
  year: Option<i32>,

  // 布尔类型
  published: Option<bool>,
  featured: Option<bool>,
  free: Option<bool>,

  // 数组类型
  tags: Vec<String>,
  categories: Vec<String>,
  authors: Vec<String>,

  // 其他
  sort: Option<String>,
  format: Option<String>,
  status: Option<String>,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_search_query_parse_and_format() {
    let query_str = "q=rust&page=2&limit=10&tags=programming&tags=tutorial";
    let query = SearchQuery::parse(query_str).unwrap();

    assert_eq!(query.q, Some("rust".to_string()));
    assert_eq!(query.page, Some(2));
    assert_eq!(query.limit, Some(10));
    assert_eq!(query.tags, vec!["programming", "tutorial"]);

    let formatted = query.format();
    let reparsed = SearchQuery::parse(&formatted).unwrap();
    assert_eq!(query, reparsed);
  }

  #[test]
  fn test_filter_query_with_floats() {
    let query_str = "active=true&sort_by=price&min_price=10.5&max_price=99.99&categories=electronics&categories=books";
    let query = FilterQuery::parse(query_str).unwrap();

    assert_eq!(query.active, Some(true));
    assert_eq!(query.sort_by, Some("price".to_string()));
    assert_eq!(query.min_price, Some(10.5));
    assert_eq!(query.max_price, Some(99.99));
    assert_eq!(query.categories, vec!["electronics", "books"]);
  }

  #[test]
  fn test_empty_query() {
    let query = SearchQuery::parse("").unwrap();
    assert_eq!(query.q, None);
    assert_eq!(query.page, None);
    assert_eq!(query.limit, None);
    assert!(query.tags.is_empty());

    let query = SearchQuery::default();
    let formatted = query.format();
    assert!(formatted.is_empty() || formatted == "?");
  }

  #[test]
  fn test_complex_query_all_types() {
    let query_str = "query=test&page=1&min_rating=4.5&published=true&tags=rust&tags=web&sort=date";
    let query = ComplexQuery::parse(query_str).unwrap();

    assert_eq!(query.query, Some("test".to_string()));
    assert_eq!(query.page, Some(1));
    assert_eq!(query.min_rating, Some(4.5));
    assert_eq!(query.published, Some(true));
    assert_eq!(query.tags, vec!["rust", "web"]);
    assert_eq!(query.sort, Some("date".to_string()));
  }

  #[test]
  fn test_url_encoded_values() {
    let query_str = "q=hello%20world&title=Rust%20Programming";
    let query = SearchQuery::parse(query_str).unwrap();
    assert_eq!(query.q, Some("hello world".to_string()));
  }

  #[test]
  fn test_boolean_parsing() {
    let test_cases = vec![
      ("active=true", Some(true)),
      ("active=false", Some(false)),
      ("active=1", Some(true)),
      ("active=0", Some(false)),
      ("active=yes", Some(true)),
      ("active=no", Some(false)),
      ("", None),
    ];

    for (query_str, expected) in test_cases {
      let query = FilterQuery::parse(query_str).unwrap();
      assert_eq!(query.active, expected, "Failed for: {query_str}");
    }
  }

  #[test]
  fn test_array_parameters() {
    let query_str = "tags=rust&tags=programming&tags=tutorial";
    let query = SearchQuery::parse(query_str).unwrap();
    assert_eq!(query.tags, vec!["rust", "programming", "tutorial"]);
  }

  #[test]
  fn test_numeric_types() {
    let query_str = "page=42&min_rating=4.5&year=-2023";
    let query = ComplexQuery::parse(query_str).unwrap();
    assert_eq!(query.page, Some(42));
    assert_eq!(query.min_rating, Some(4.5));
    assert_eq!(query.year, Some(-2023));
  }

  #[test]
  fn test_roundtrip_consistency() {
    let original = ComplexQuery {
      query: Some("test search".to_string()),
      page: Some(5),
      min_rating: Some(4.2),
      published: Some(true),
      tags: vec!["rust".to_string(), "web".to_string()],
      sort: Some("relevance".to_string()),
      ..Default::default()
    };

    let formatted = original.format();
    let parsed = ComplexQuery::parse(&formatted).unwrap();
    assert_eq!(original, parsed);
  }
}
