//! API 对比示例：展示新旧 API 的区别
//!
//! 这个示例展示了在支持 #[query] 字段后，如何简化 API 使用

use ruled_router::prelude::*;

#[derive(Debug, Clone, PartialEq, Default)]
struct UserQuery {
  include_fields: Vec<String>,
  include_sensitive: Option<bool>,
  format: Option<String>,
}

impl Query for UserQuery {
  fn parse(query: &str) -> Result<Self, ParseError> {
    let query_map = ruled_router::utils::parse_query_string(query)?;
    Self::from_query_map(&query_map)
  }

  fn format(&self) -> String {
    let mut formatter = ruled_router::formatter::QueryFormatter::new();
    for field in &self.include_fields {
      formatter.add("include", field.clone());
    }
    if let Some(sensitive) = self.include_sensitive {
      formatter.set("include_sensitive", sensitive.to_string());
    }
    if let Some(format) = &self.format {
      formatter.set("format", format.clone());
    }
    formatter.format()
  }

  fn from_query_map(query_map: &std::collections::HashMap<String, Vec<String>>) -> Result<Self, ParseError> {
    let mut query = Self::default();

    if let Some(includes) = query_map.get("include") {
      query.include_fields = includes.clone();
    }

    if let Some(sensitive) = query_map.get("include_sensitive").and_then(|v| v.first()) {
      query.include_sensitive = Some(sensitive.parse().unwrap_or(false));
    }

    if let Some(format) = query_map.get("format").and_then(|v| v.first()) {
      query.format = Some(format.clone());
    }

    Ok(query)
  }

  fn to_query_string(&self) -> String {
    self.format()
  }
}

// 新 API：带有 #[query] 字段的路由
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/users/:id")]
struct UserRouteNew {
  id: u32,
  #[query]
  query: UserQuery,
}

// 传统 API：不带 #[query] 字段的路由
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/users/:id")]
struct UserRouteOld {
  id: u32,
}

// 传统 API 的解析函数
fn parse_full_url<R, Q>(url: &str) -> Result<(R, Q), Box<dyn std::error::Error>>
where
  R: Router,
  Q: Query + Default,
{
  let (path, query_str) = if let Some(pos) = url.find('?') {
    (&url[..pos], &url[pos + 1..])
  } else {
    (url, "")
  };

  let route = R::parse(path)?;
  let query = if !query_str.is_empty() {
    Q::parse(query_str)?
  } else {
    Q::default()
  };

  Ok((route, query))
}

// 新 API 的解析函数
fn parse_url<R>(url: &str) -> Result<R, Box<dyn std::error::Error>>
where
  R: Router,
{
  R::parse(url).map_err(|e| e.into())
}

fn main() {
  println!("=== API 对比示例 ===");

  let url = "/users/123?include=profile&include=settings&include_sensitive=true&format=json";
  println!("\n测试 URL: {url}");

  // 传统 API 使用方式
  println!("\n=== 传统 API ===");
  match parse_full_url::<UserRouteOld, UserQuery>(url) {
    Ok((route, query)) => {
      println!("路由: {route:#?}");
      println!("查询: {query:#?}");
      println!("需要分别处理 route 和 query 两个对象");

      // 重新构建 URL 需要两个参数
      let reconstructed = format!("{}?{}", route.format(), query.format());
      println!("重构 URL: {reconstructed}");
    }
    Err(e) => println!("解析失败: {e}"),
  }

  // 新 API 使用方式
  println!("\n=== 新 API ===");
  match parse_url::<UserRouteNew>(url) {
    Ok(route) => {
      println!("路由: {route:#?}");
      println!("只需要处理一个 route 对象，查询信息已包含在内");
      println!("用户ID: {}", route.id);
      println!("查询参数: {:#?}", route.query);

      // 重新构建 URL 只需要一个参数
      let reconstructed = route.format();
      println!("重构 URL: {reconstructed}");
    }
    Err(e) => println!("解析失败: {e}"),
  }

  println!("\n=== 总结 ===");
  println!("新 API 的优势:");
  println!("1. 一体化设计：route 包含所有信息，无需分别处理");
  println!("2. 类型安全：查询参数与路由绑定，减少错误");
  println!("3. 简化代码：只需要一个对象，API 更简洁");
  println!("4. 自动格式化：route.format() 自动包含查询参数");
  println!("5. 更好的封装：相关数据聚合在一起");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_api_comparison() {
    let url = "/users/456?include=profile&format=xml";

    // 测试传统 API
    let (old_route, old_query) = parse_full_url::<UserRouteOld, UserQuery>(url).unwrap();
    assert_eq!(old_route.id, 456);
    assert_eq!(old_query.include_fields, vec!["profile"]);
    assert_eq!(old_query.format, Some("xml".to_string()));

    // 测试新 API
    let new_route = parse_url::<UserRouteNew>(url).unwrap();
    assert_eq!(new_route.id, 456);
    assert_eq!(new_route.query.include_fields, vec!["profile"]);
    assert_eq!(new_route.query.format, Some("xml".to_string()));

    // 验证新 API 的便利性
    assert_eq!(new_route.id, old_route.id);
    assert_eq!(new_route.query.include_fields, old_query.include_fields);
    assert_eq!(new_route.query.format, old_query.format);
  }

  #[test]
  fn test_new_api_simplicity() {
    let url = "/users/789?include=settings&include_sensitive=false";

    // 新 API：一步解析，一个对象
    let route = parse_url::<UserRouteNew>(url).unwrap();
    let reconstructed = route.format();

    // 验证往返转换
    let reparsed = parse_url::<UserRouteNew>(&reconstructed).unwrap();
    assert_eq!(route.id, reparsed.id);
    assert_eq!(route.query.include_fields, reparsed.query.include_fields);
    assert_eq!(route.query.include_sensitive, reparsed.query.include_sensitive);
  }
}
