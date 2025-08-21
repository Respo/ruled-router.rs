//! 路由格式化器
//!
//! 提供将结构化数据格式化为路径和查询字符串的功能

use crate::error::ParseError;
use crate::parser::{PathParser, QueryParser};
use crate::traits::{Query, Router, ToParam};
use crate::utils::{format_query_string, normalize_path};
use std::collections::HashMap;

/// 路径格式化器
///
/// 用于将路由结构体格式化为 URL 路径
#[derive(Debug, Clone)]
pub struct PathFormatter {
  parser: PathParser,
}

impl PathFormatter {
  /// 创建新的路径格式化器
  pub fn new(pattern: &str) -> Result<Self, ParseError> {
    Ok(Self {
      parser: PathParser::new(pattern)?,
    })
  }

  /// 格式化路径
  ///
  /// # 参数
  ///
  /// * `params` - 路径参数映射
  ///
  /// # 返回值
  ///
  /// 格式化后的路径字符串
  ///
  /// # 示例
  ///
  /// ```rust
  /// use ruled_router::formatter::PathFormatter;
  /// use std::collections::HashMap;
  ///
  /// let formatter = PathFormatter::new("/users/{id}/posts/{post_id}").unwrap();
  /// let mut params = HashMap::new();
  /// params.insert("id".to_string(), "123".to_string());
  /// params.insert("post_id".to_string(), "456".to_string());
  ///
  /// let path = formatter.format(&params).unwrap();
  /// assert_eq!(path, "/users/123/posts/456");
  /// ```
  pub fn format(&self, params: &HashMap<String, String>) -> Result<String, ParseError> {
    self.parser.format_path(params)
  }

  /// 格式化路径（使用类型安全的参数）
  ///
  /// # 参数
  ///
  /// * `typed_params` - 类型化的参数映射
  ///
  /// # 返回值
  ///
  /// 格式化后的路径字符串
  pub fn format_typed<T: ToParam>(&self, typed_params: &HashMap<String, T>) -> Result<String, ParseError> {
    let string_params: HashMap<String, String> = typed_params.iter().map(|(k, v)| (k.clone(), v.to_param())).collect();
    self.format(&string_params)
  }
}

/// 查询格式化器
///
/// 用于将查询结构体格式化为查询字符串
#[derive(Debug, Clone, Default)]
pub struct QueryFormatter {
  params: HashMap<String, Vec<String>>,
}

impl QueryFormatter {
  /// 创建新的查询格式化器
  pub fn new() -> Self {
    Self::default()
  }

  /// 从查询解析器创建格式化器
  pub fn from_parser(parser: &QueryParser) -> Self {
    Self {
      params: parser.params().clone(),
    }
  }

  /// 设置参数值
  ///
  /// # 参数
  ///
  /// * `key` - 参数名
  /// * `value` - 参数值
  pub fn set<T: ToParam>(&mut self, key: &str, value: T) -> &mut Self {
    self.params.insert(key.to_string(), vec![value.to_param()]);
    self
  }

  /// 添加参数值（支持多值）
  ///
  /// # 参数
  ///
  /// * `key` - 参数名
  /// * `value` - 参数值
  pub fn add<T: ToParam>(&mut self, key: &str, value: T) -> &mut Self {
    self.params.entry(key.to_string()).or_default().push(value.to_param());
    self
  }

  /// 设置多个值
  ///
  /// # 参数
  ///
  /// * `key` - 参数名
  /// * `values` - 参数值列表
  pub fn set_multiple<T: ToParam>(&mut self, key: &str, values: &[T]) -> &mut Self {
    let string_values: Vec<String> = values.iter().map(|v| v.to_param()).collect();
    self.params.insert(key.to_string(), string_values);
    self
  }

  /// 移除参数
  ///
  /// # 参数
  ///
  /// * `key` - 参数名
  pub fn remove(&mut self, key: &str) -> &mut Self {
    self.params.remove(key);
    self
  }

  /// 清空所有参数
  pub fn clear(&mut self) -> &mut Self {
    self.params.clear();
    self
  }

  /// 格式化为查询字符串
  ///
  /// # 返回值
  ///
  /// 格式化后的查询字符串（不包含 '?' 前缀）
  ///
  /// # 示例
  ///
  /// ```rust
  /// use ruled_router::formatter::QueryFormatter;
  ///
  /// let mut formatter = QueryFormatter::new();
  /// formatter.set("page", 1)
  ///          .set("size", 20)
  ///          .add("tags", "rust")
  ///          .add("tags", "web");
  ///
  /// let query = formatter.format();
  /// // 结果类似: "page=1&size=20&tags=rust&tags=web"
  /// ```
  pub fn format(&self) -> String {
    format_query_string(&self.params)
  }

  /// 格式化为完整的查询字符串（包含 '?' 前缀）
  ///
  /// # 返回值
  ///
  /// 格式化后的查询字符串，如果没有参数则返回空字符串
  pub fn format_with_prefix(&self) -> String {
    let query = self.format();
    if query.is_empty() {
      String::new()
    } else {
      format!("?{query}")
    }
  }

  /// 检查是否为空
  pub fn is_empty(&self) -> bool {
    self.params.is_empty()
  }

  /// 获取参数数量
  pub fn len(&self) -> usize {
    self.params.len()
  }

  /// 获取所有参数的引用
  pub fn params(&self) -> &HashMap<String, Vec<String>> {
    &self.params
  }
}

/// URL 格式化器
///
/// 组合路径和查询参数格式化功能
#[derive(Debug, Clone)]
pub struct UrlFormatter {
  path_formatter: PathFormatter,
  query_formatter: QueryFormatter,
}

impl UrlFormatter {
  /// 创建新的 URL 格式化器
  ///
  /// # 参数
  ///
  /// * `path_pattern` - 路径模式
  pub fn new(path_pattern: &str) -> Result<Self, ParseError> {
    Ok(Self {
      path_formatter: PathFormatter::new(path_pattern)?,
      query_formatter: QueryFormatter::new(),
    })
  }

  /// 获取路径格式化器的可变引用
  pub fn path_formatter(&self) -> &PathFormatter {
    &self.path_formatter
  }

  /// 获取查询格式化器的可变引用
  pub fn query_formatter_mut(&mut self) -> &mut QueryFormatter {
    &mut self.query_formatter
  }

  /// 获取查询格式化器的引用
  pub fn query_formatter(&self) -> &QueryFormatter {
    &self.query_formatter
  }

  /// 格式化完整的 URL
  ///
  /// # 参数
  ///
  /// * `path_params` - 路径参数
  ///
  /// # 返回值
  ///
  /// 完整的 URL 字符串
  ///
  /// # 示例
  ///
  /// ```rust
  /// use ruled_router::formatter::UrlFormatter;
  /// use std::collections::HashMap;
  ///
  /// let mut formatter = UrlFormatter::new("/users/{id}").unwrap();
  ///
  /// // 设置查询参数
  /// formatter.query_formatter_mut()
  ///          .set("page", 1)
  ///          .set("size", 20);
  ///
  /// // 设置路径参数
  /// let mut path_params = HashMap::new();
  /// path_params.insert("id".to_string(), "123".to_string());
  ///
  /// let url = formatter.format(&path_params).unwrap();
  /// // 结果: "/users/123?page=1&size=20"
  /// ```
  pub fn format(&self, path_params: &HashMap<String, String>) -> Result<String, ParseError> {
    let path = self.path_formatter.format(path_params)?;
    let query = self.query_formatter.format_with_prefix();
    Ok(format!("{}{}", normalize_path(&path), query))
  }

  /// 格式化完整的 URL（使用类型安全的路径参数）
  pub fn format_typed<T: ToParam>(&self, path_params: &HashMap<String, T>) -> Result<String, ParseError> {
    let path = self.path_formatter.format_typed(path_params)?;
    let query = self.query_formatter.format_with_prefix();
    Ok(format!("{}{}", normalize_path(&path), query))
  }
}

// 注意：RouterFormatter 和 QueryFormatter_ trait 将在宏实现时提供
// 这些 trait 需要访问结构体的内部字段，所以会通过派生宏自动生成

#[cfg(test)]
mod tests {
  use super::*;
  use std::collections::HashMap;

  #[test]
  fn test_path_formatter() {
    let formatter = PathFormatter::new("/users/{id}/posts/{post_id}").unwrap();

    let mut params = HashMap::new();
    params.insert("id".to_string(), "123".to_string());
    params.insert("post_id".to_string(), "456".to_string());

    let path = formatter.format(&params).unwrap();
    assert_eq!(path, "/users/123/posts/456");
  }

  #[test]
  fn test_path_formatter_typed() {
    let formatter = PathFormatter::new("/users/{id}").unwrap();

    let mut params = HashMap::new();
    params.insert("id".to_string(), 123u32);

    let path = formatter.format_typed(&params).unwrap();
    assert_eq!(path, "/users/123");
  }

  #[test]
  fn test_query_formatter() {
    let mut formatter = QueryFormatter::new();

    formatter.set("page", 1).set("size", 20).add("tags", "rust").add("tags", "web");

    let query = formatter.format();

    // 查询参数的顺序可能不同，所以我们检查包含关系
    assert!(query.contains("page=1"));
    assert!(query.contains("size=20"));
    assert!(query.contains("tags=rust"));
    assert!(query.contains("tags=web"));
  }

  #[test]
  fn test_query_formatter_with_prefix() {
    let mut formatter = QueryFormatter::new();
    formatter.set("test", "value");

    let query = formatter.format_with_prefix();
    assert_eq!(query, "?test=value");

    // 测试空查询
    let empty_formatter = QueryFormatter::new();
    let empty_query = empty_formatter.format_with_prefix();
    assert_eq!(empty_query, "");
  }

  #[test]
  fn test_query_formatter_multiple_values() {
    let mut formatter = QueryFormatter::new();

    formatter.set_multiple("colors", &["red", "green", "blue"]);

    let query = formatter.format();
    assert!(query.contains("colors=red"));
    assert!(query.contains("colors=green"));
    assert!(query.contains("colors=blue"));
  }

  #[test]
  fn test_url_formatter() {
    let mut formatter = UrlFormatter::new("/users/{id}").unwrap();

    formatter.query_formatter_mut().set("page", 1).set("size", 20);

    let mut path_params = HashMap::new();
    path_params.insert("id".to_string(), "123".to_string());

    let url = formatter.format(&path_params).unwrap();

    assert!(url.starts_with("/users/123?"));
    assert!(url.contains("page=1"));
    assert!(url.contains("size=20"));
  }

  #[test]
  fn test_query_formatter_operations() {
    let mut formatter = QueryFormatter::new();

    // 测试设置和添加
    formatter.set("key1", "value1").add("key2", "value2a").add("key2", "value2b");

    assert_eq!(formatter.len(), 2);
    assert!(!formatter.is_empty());

    // 测试移除
    formatter.remove("key1");
    assert_eq!(formatter.len(), 1);

    // 测试清空
    formatter.clear();
    assert_eq!(formatter.len(), 0);
    assert!(formatter.is_empty());
  }
}
