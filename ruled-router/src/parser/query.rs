//! 查询参数解析器
//!
//! 提供查询字符串的解析和格式化功能

use crate::error::{ParseError, ParseResult};
use crate::utils::{format_query_string, parse_query_string};
use std::collections::HashMap;

/// 查询参数解析器
///
/// 负责解析查询字符串并提供类型安全的参数访问
#[derive(Debug, Clone)]
pub struct QueryParser {
  /// 原始查询字符串
  raw_query: String,
  /// 解析后的参数映射
  params: HashMap<String, Vec<String>>,
}

impl QueryParser {
  /// 创建新的查询参数解析器
  ///
  /// # 参数
  ///
  /// * `query` - 查询字符串，不包含前导的 '?'
  ///
  /// # 返回值
  ///
  /// 解析器实例，如果解析失败则返回错误
  ///
  /// # 示例
  ///
  /// ```rust
  /// use ruled_router::parser::QueryParser;
  ///
  /// let parser = QueryParser::new("q=rust&page=2&tags=web&tags=backend").unwrap();
  /// ```
  pub fn new(query: &str) -> ParseResult<Self> {
    let params = parse_query_string(query)?;
    Ok(Self {
      raw_query: query.to_string(),
      params,
    })
  }

  /// 从参数映射创建查询解析器
  ///
  /// # 参数
  ///
  /// * `params` - 参数映射
  ///
  /// # 返回值
  ///
  /// 解析器实例
  pub fn from_params(params: HashMap<String, Vec<String>>) -> Self {
    let raw_query = format_query_string(&params);
    Self { raw_query, params }
  }

  /// 获取单个参数值
  ///
  /// # 参数
  ///
  /// * `key` - 参数名
  ///
  /// # 返回值
  ///
  /// 参数的第一个值，如果参数不存在则返回 None
  ///
  /// # 示例
  ///
  /// ```rust
  /// use ruled_router::parser::QueryParser;
  ///
  /// let parser = QueryParser::new("q=rust&page=2").unwrap();
  /// assert_eq!(parser.get("q"), Some("rust"));
  /// assert_eq!(parser.get("page"), Some("2"));
  /// assert_eq!(parser.get("missing"), None);
  /// ```
  pub fn get(&self, key: &str) -> Option<&str> {
    self.params.get(key)?.first().map(|s| s.as_str())
  }

  /// 获取多个参数值
  ///
  /// # 参数
  ///
  /// * `key` - 参数名
  ///
  /// # 返回值
  ///
  /// 参数的所有值，如果参数不存在则返回空切片
  ///
  /// # 示例
  ///
  /// ```rust
  /// use ruled_router::parser::QueryParser;
  ///
  /// let parser = QueryParser::new("tags=web&tags=backend&tags=rust").unwrap();
  /// let tags: Vec<&str> = parser.get_all("tags").iter().map(|s| s.as_str()).collect();
  /// assert_eq!(tags, vec!["web", "backend", "rust"]);
  /// ```
  pub fn get_all(&self, key: &str) -> &[String] {
    self.params.get(key).map(|v| v.as_slice()).unwrap_or(&[])
  }

  /// 检查参数是否存在
  ///
  /// # 参数
  ///
  /// * `key` - 参数名
  ///
  /// # 返回值
  ///
  /// 如果参数存在则返回 true，否则返回 false
  pub fn contains(&self, key: &str) -> bool {
    self.params.contains_key(key)
  }

  /// 获取参数的类型安全值
  ///
  /// # 参数
  ///
  /// * `key` - 参数名
  ///
  /// # 返回值
  ///
  /// 解析后的值，如果参数不存在或解析失败则返回错误
  ///
  /// # 示例
  ///
  /// ```rust
  /// use ruled_router::parser::QueryParser;
  ///
  /// let parser = QueryParser::new("page=2&limit=10").unwrap();
  /// let page: u32 = parser.get_parsed("page").unwrap();
  /// let limit: u32 = parser.get_parsed("limit").unwrap();
  /// assert_eq!(page, 2);
  /// assert_eq!(limit, 10);
  /// ```
  pub fn get_parsed<T>(&self, key: &str) -> ParseResult<T>
  where
    T: crate::traits::FromParam,
  {
    let value = self.get(key).ok_or_else(|| ParseError::missing_parameter(key.to_string()))?;
    T::from_param(value)
  }

  /// 获取可选的类型安全值
  ///
  /// # 参数
  ///
  /// * `key` - 参数名
  ///
  /// # 返回值
  ///
  /// 解析后的可选值，如果参数不存在则返回 None，如果解析失败则返回错误
  ///
  /// # 示例
  ///
  /// ```rust
  /// use ruled_router::parser::QueryParser;
  ///
  /// let parser = QueryParser::new("page=2").unwrap();
  /// let page: Option<u32> = parser.get_optional("page").unwrap();
  /// let limit: Option<u32> = parser.get_optional("limit").unwrap();
  /// assert_eq!(page, Some(2));
  /// assert_eq!(limit, None);
  /// ```
  pub fn get_optional<T>(&self, key: &str) -> ParseResult<Option<T>>
  where
    T: crate::traits::FromParam,
  {
    match self.get(key) {
      Some(value) => T::from_param(value).map(Some),
      None => Ok(None),
    }
  }

  /// 获取带默认值的类型安全值
  ///
  /// # 参数
  ///
  /// * `key` - 参数名
  /// * `default` - 默认值
  ///
  /// # 返回值
  ///
  /// 解析后的值，如果参数不存在则返回默认值，如果解析失败则返回错误
  ///
  /// # 示例
  ///
  /// ```rust
  /// use ruled_router::parser::QueryParser;
  ///
  /// let parser = QueryParser::new("page=2").unwrap();
  /// let page: u32 = parser.get_with_default("page", 1).unwrap();
  /// let limit: u32 = parser.get_with_default("limit", 10).unwrap();
  /// assert_eq!(page, 2);
  /// assert_eq!(limit, 10);
  /// ```
  pub fn get_with_default<T>(&self, key: &str, default: T) -> ParseResult<T>
  where
    T: crate::traits::FromParam,
  {
    match self.get(key) {
      Some(value) => T::from_param(value),
      None => Ok(default),
    }
  }

  /// 获取多值参数的类型安全值
  ///
  /// # 参数
  ///
  /// * `key` - 参数名
  ///
  /// # 返回值
  ///
  /// 解析后的值向量，如果任何值解析失败则返回错误
  ///
  /// # 示例
  ///
  /// ```rust
  /// use ruled_router::parser::QueryParser;
  ///
  /// let parser = QueryParser::new("ids=1&ids=2&ids=3").unwrap();
  /// let ids: Vec<u32> = parser.get_all_parsed("ids").unwrap();
  /// assert_eq!(ids, vec![1, 2, 3]);
  /// ```
  pub fn get_all_parsed<T>(&self, key: &str) -> ParseResult<Vec<T>>
  where
    T: crate::traits::FromParam,
  {
    let values = self.get_all(key);
    values.iter().map(|s| T::from_param(s)).collect::<ParseResult<Vec<_>>>()
  }

  /// 设置参数值
  ///
  /// # 参数
  ///
  /// * `key` - 参数名
  /// * `value` - 参数值
  pub fn set<T>(&mut self, key: &str, value: T)
  where
    T: crate::traits::ToParam,
  {
    let value_str = value.to_param();
    self.params.insert(key.to_string(), vec![value_str]);
    self.update_raw_query();
  }

  /// 添加参数值（支持多值）
  ///
  /// # 参数
  ///
  /// * `key` - 参数名
  /// * `value` - 参数值
  pub fn add<T>(&mut self, key: &str, value: T)
  where
    T: crate::traits::ToParam,
  {
    let value_str = value.to_param();
    self.params.entry(key.to_string()).or_default().push(value_str);
    self.update_raw_query();
  }

  /// 移除参数
  ///
  /// # 参数
  ///
  /// * `key` - 参数名
  ///
  /// # 返回值
  ///
  /// 被移除的参数值，如果参数不存在则返回 None
  pub fn remove(&mut self, key: &str) -> Option<Vec<String>> {
    let result = self.params.remove(key);
    self.update_raw_query();
    result
  }

  /// 清空所有参数
  pub fn clear(&mut self) {
    self.params.clear();
    self.raw_query.clear();
  }

  /// 获取所有参数名
  ///
  /// # 返回值
  ///
  /// 参数名的向量
  pub fn keys(&self) -> Vec<&str> {
    self.params.keys().map(|s| s.as_str()).collect()
  }

  /// 检查是否为空
  ///
  /// # 返回值
  ///
  /// 如果没有任何参数则返回 true
  pub fn is_empty(&self) -> bool {
    self.params.is_empty()
  }

  /// 获取参数数量
  ///
  /// # 返回值
  ///
  /// 参数的数量（不是值的数量）
  pub fn len(&self) -> usize {
    self.params.len()
  }

  /// 格式化为查询字符串
  ///
  /// # 返回值
  ///
  /// 格式化后的查询字符串，不包含前导的 '?'
  ///
  /// # 示例
  ///
  /// ```rust
  /// use ruled_router::parser::QueryParser;
  ///
  /// let parser = QueryParser::new("q=rust&page=2").unwrap();
  /// let formatted = parser.format();
  /// // 注意：HashMap 的迭代顺序不确定
  /// assert!(formatted.contains("q=rust"));
  /// assert!(formatted.contains("page=2"));
  /// ```
  pub fn format(&self) -> String {
    self.raw_query.clone()
  }

  /// 获取原始查询字符串
  pub fn raw(&self) -> &str {
    &self.raw_query
  }

  /// 获取参数映射的引用
  pub fn params(&self) -> &HashMap<String, Vec<String>> {
    &self.params
  }

  /// 更新原始查询字符串
  fn update_raw_query(&mut self) {
    self.raw_query = format_query_string(&self.params);
  }
}

/// 查询参数构建器
///
/// 提供链式调用的方式构建查询参数
#[derive(Debug, Default)]
pub struct QueryBuilder {
  params: HashMap<String, Vec<String>>,
}

impl QueryBuilder {
  /// 创建新的查询构建器
  pub fn new() -> Self {
    Self::default()
  }

  /// 设置参数值
  pub fn set<T>(mut self, key: &str, value: T) -> Self
  where
    T: crate::traits::ToParam,
  {
    let value_str = value.to_param();
    self.params.insert(key.to_string(), vec![value_str]);
    self
  }

  /// 添加参数值（支持多值）
  pub fn add<T>(mut self, key: &str, value: T) -> Self
  where
    T: crate::traits::ToParam,
  {
    let value_str = value.to_param();
    self.params.entry(key.to_string()).or_default().push(value_str);
    self
  }

  /// 构建查询解析器
  pub fn build(self) -> QueryParser {
    QueryParser::from_params(self.params)
  }

  /// 构建查询字符串
  pub fn build_string(self) -> String {
    format_query_string(&self.params)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_simple_query() {
    let parser = QueryParser::new("q=rust&page=2").unwrap();

    assert_eq!(parser.get("q"), Some("rust"));
    assert_eq!(parser.get("page"), Some("2"));
    assert_eq!(parser.get("missing"), None);
  }

  #[test]
  fn test_parse_multi_value_query() {
    let parser = QueryParser::new("tags=web&tags=backend&tags=rust").unwrap();

    let tags = parser.get_all("tags");
    assert_eq!(tags, &["web", "backend", "rust"]);

    assert_eq!(parser.get("tags"), Some("web")); // 第一个值
  }

  #[test]
  fn test_empty_query() {
    let parser = QueryParser::new("").unwrap();

    assert!(parser.is_empty());
    assert_eq!(parser.len(), 0);
    assert_eq!(parser.keys(), Vec::<&str>::new());
  }

  #[test]
  fn test_query_with_empty_values() {
    let parser = QueryParser::new("flag&empty=").unwrap();

    assert_eq!(parser.get("flag"), Some(""));
    assert_eq!(parser.get("empty"), Some(""));
  }

  #[test]
  fn test_url_encoded_query() {
    let parser = QueryParser::new("q=hello%20world&name=John%2BDoe").unwrap();

    assert_eq!(parser.get("q"), Some("hello world"));
    assert_eq!(parser.get("name"), Some("John Doe"));
  }

  #[test]
  fn test_query_modification() {
    let mut parser = QueryParser::new("q=rust").unwrap();

    parser.set("page", 2u32);
    parser.add("tags", "web");
    parser.add("tags", "backend");

    assert_eq!(parser.get("page"), Some("2"));
    assert_eq!(parser.get_all("tags"), &["web", "backend"]);

    parser.remove("q");
    assert_eq!(parser.get("q"), None);
  }

  #[test]
  fn test_query_builder() {
    let query = QueryBuilder::new()
      .set("q", "rust")
      .set("page", 2u32)
      .add("tags", "web")
      .add("tags", "backend")
      .build_string();

    // 验证查询字符串包含所有参数
    assert!(query.contains("q=rust"));
    assert!(query.contains("page=2"));
    assert!(query.contains("tags=web"));
    assert!(query.contains("tags=backend"));
  }

  #[test]
  fn test_contains_and_keys() {
    let parser = QueryParser::new("q=rust&page=2&tags=web").unwrap();

    assert!(parser.contains("q"));
    assert!(parser.contains("page"));
    assert!(parser.contains("tags"));
    assert!(!parser.contains("missing"));

    let mut keys = parser.keys();
    keys.sort(); // HashMap 顺序不确定
    assert_eq!(keys, vec!["page", "q", "tags"]);
  }
}
