//! Query derive 宏测试
//!
//! 测试 #[derive(Query)] 宏的各种功能，包括自动字段映射和类型转换

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

  // 数值类型
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

  // 其他类型
  sort: Option<String>,
  format: Option<String>,
  status: Option<String>,
}

/// 带自定义字段名的查询参数测试（使用标准字段名）
#[derive(Debug, Clone, PartialEq, Default, Query)]
struct CustomFieldQuery {
  q: Option<String>,
  p: Option<u32>,
  size: Option<u32>,
  sort_by: Option<String>,
}

/// 嵌套查询参数测试
#[derive(Debug, Clone, PartialEq, Default, Query)]
struct NestedQuery {
  // 基础字段
  query: Option<String>,

  // 分页相关
  page: Option<u32>,
  per_page: Option<u32>,
  offset: Option<u32>,

  // 排序相关
  sort: Option<String>,
  order: Option<String>,

  // 过滤相关
  status: Vec<String>,
  category: Vec<String>,
  tag: Vec<String>,

  // 数值范围
  min_price: Option<f64>,
  max_price: Option<f64>,
  min_date: Option<String>,
  max_date: Option<String>,

  // 布尔标志
  active: Option<bool>,
  featured: Option<bool>,
  public: Option<bool>,
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

  #[test]
  fn test_custom_field_names() {
    // 测试自定义字段名映射
    let query_str = "q=rust&p=2&size=50&sort_by=date";
    let query = CustomFieldQuery::parse(query_str).unwrap();

    assert_eq!(query.q, Some("rust".to_string()));
    assert_eq!(query.p, Some(2));
    assert_eq!(query.size, Some(50));
    assert_eq!(query.sort_by, Some("date".to_string()));

    // 测试格式化
    let formatted = query.format();
    assert!(formatted.contains("q=rust"));
    assert!(formatted.contains("p=2"));
    assert!(formatted.contains("size=50"));
    assert!(formatted.contains("sort_by=date"));
  }

  #[test]
  fn test_nested_query_comprehensive() {
    // 测试复杂的嵌套查询参数
    let query_str = "query=search&page=1&per_page=20&offset=0&sort=name&order=asc&status=active&status=pending&category=tech&category=web&tag=rust&tag=programming&min_price=10.5&max_price=99.99&min_date=2023-01-01&max_date=2023-12-31&active=true&featured=false&public=1";
    let query = NestedQuery::parse(query_str).unwrap();

    // 基础字段
    assert_eq!(query.query, Some("search".to_string()));

    // 分页相关
    assert_eq!(query.page, Some(1));
    assert_eq!(query.per_page, Some(20));
    assert_eq!(query.offset, Some(0));

    // 排序相关
    assert_eq!(query.sort, Some("name".to_string()));
    assert_eq!(query.order, Some("asc".to_string()));

    // 过滤相关（数组）
    assert_eq!(query.status, vec!["active".to_string(), "pending".to_string()]);
    assert_eq!(query.category, vec!["tech".to_string(), "web".to_string()]);
    assert_eq!(query.tag, vec!["rust".to_string(), "programming".to_string()]);

    // 数值范围
    assert_eq!(query.min_price, Some(10.5));
    assert_eq!(query.max_price, Some(99.99));
    assert_eq!(query.min_date, Some("2023-01-01".to_string()));
    assert_eq!(query.max_date, Some("2023-12-31".to_string()));

    // 布尔标志
    assert_eq!(query.active, Some(true));
    assert_eq!(query.featured, Some(false));
    assert_eq!(query.public, Some(true)); // "1" 应该解析为 true
  }

  #[test]
  fn test_partial_query_parameters() {
    // 测试部分查询参数
    let query_str = "query=test&page=5&active=true";
    let query = NestedQuery::parse(query_str).unwrap();

    assert_eq!(query.query, Some("test".to_string()));
    assert_eq!(query.page, Some(5));
    assert_eq!(query.active, Some(true));

    // 其他字段应该是默认值
    assert_eq!(query.per_page, None);
    assert_eq!(query.offset, None);
    assert_eq!(query.sort, None);
    assert!(query.status.is_empty());
    assert!(query.category.is_empty());
    assert_eq!(query.min_price, None);
    assert_eq!(query.featured, None);
  }

  #[test]
  fn test_boolean_value_variations() {
    // 测试布尔值的各种表示方式
    let test_cases = vec![
      ("active=true", Some(true)),
      ("active=false", Some(false)),
      ("active=1", Some(true)),
      ("active=0", Some(false)),
      ("active=yes", Some(true)),
      ("active=no", Some(false)),
      ("active=on", Some(true)),
      ("active=off", Some(false)),
    ];

    for (query_str, expected) in test_cases {
      let query = NestedQuery::parse(query_str).unwrap();
      assert_eq!(query.active, expected, "Failed for query: {query_str}");
    }
  }

  #[test]
  fn test_numeric_type_conversions() {
    // 测试数值类型转换
    let query_str = "page=42&per_page=100&offset=200&min_price=19.99&max_price=199.99";
    let query = NestedQuery::parse(query_str).unwrap();

    assert_eq!(query.page, Some(42));
    assert_eq!(query.per_page, Some(100));
    assert_eq!(query.offset, Some(200));
    assert_eq!(query.min_price, Some(19.99));
    assert_eq!(query.max_price, Some(199.99));
  }

  #[test]
  fn test_array_parameter_handling() {
    // 测试数组参数的处理
    let query_str = "tag=rust&tag=web&tag=backend&category=tech&status=active";
    let query = NestedQuery::parse(query_str).unwrap();

    assert_eq!(query.tag, vec!["rust".to_string(), "web".to_string(), "backend".to_string()]);
    assert_eq!(query.category, vec!["tech".to_string()]);
    assert_eq!(query.status, vec!["active".to_string()]);
  }

  #[test]
  fn test_url_encoded_values_in_nested_query() {
    // 测试 URL 编码的值
    let query_str = "query=hello%20world&tag=rust%20programming&min_date=2023-01-01%2010%3A00%3A00";
    let query = NestedQuery::parse(query_str).unwrap();

    assert_eq!(query.query, Some("hello world".to_string()));
    assert_eq!(query.tag, vec!["rust programming".to_string()]);
    assert_eq!(query.min_date, Some("2023-01-01 10:00:00".to_string()));
  }

  #[test]
  fn test_empty_and_malformed_values() {
    // 测试空值和格式错误的值
    let query_str = "query=&page=abc&active=maybe&min_price=not_a_number";

    // 这应该返回错误，因为包含无法解析的值
    let result = NestedQuery::parse(query_str);
    assert!(result.is_err(), "Should fail to parse malformed query parameters");
  }

  #[test]
  fn test_query_formatting_with_arrays() {
    // 测试包含数组的查询格式化
    let query = NestedQuery {
      query: Some("test".to_string()),
      page: Some(1),
      status: vec!["active".to_string(), "pending".to_string()],
      tag: vec!["rust".to_string(), "web".to_string()],
      active: Some(true),
      min_price: Some(10.5),
      ..Default::default()
    };

    let formatted = query.format();

    // 验证格式化结果包含所有字段
    assert!(formatted.contains("query=test"));
    assert!(formatted.contains("page=1"));
    assert!(formatted.contains("status=active"));
    assert!(formatted.contains("status=pending"));
    assert!(formatted.contains("tag=rust"));
    assert!(formatted.contains("tag=web"));
    assert!(formatted.contains("active=true"));
    assert!(formatted.contains("min_price=10.5"));
  }
}
