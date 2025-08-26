//! 高级功能测试
//!
//! 测试自定义类型转换、复杂路由模式等高级功能

use ruled_router::prelude::*;
use std::collections::HashMap;

/// 自定义枚举类型
#[derive(Debug, Clone, PartialEq)]
enum UserRole {
  Admin,
  User,
  Guest,
}

impl FromParam for UserRole {
  fn from_param(param: &str) -> Result<Self, ParseError> {
    match param.to_lowercase().as_str() {
      "admin" => Ok(UserRole::Admin),
      "user" => Ok(UserRole::User),
      "guest" => Ok(UserRole::Guest),
      _ => Err(ParseError::type_conversion(format!("Invalid role: {param}"))),
    }
  }
}

impl ToParam for UserRole {
  fn to_param(&self) -> String {
    match self {
      UserRole::Admin => "admin".to_string(),
      UserRole::User => "user".to_string(),
      UserRole::Guest => "guest".to_string(),
    }
  }
}

/// 复杂的 API 路由，手动实现以测试高级功能
#[derive(Debug, Clone, PartialEq)]
struct ApiRoute {
  version: String,
  resource: String,
  id: Option<u32>,
  action: Option<String>,
}

impl RouterData for ApiRoute {
  type SubRouterMatch = ::ruled_router::NoSubRouter;

  fn parse(path: &str) -> Result<Self, ParseError> {
    let (path_part, _) = ruled_router::utils::split_path_query(path);
    let segments: Vec<&str> = path_part.trim_start_matches('/').split('/').collect();

    if segments.len() < 3 || segments[0] != "api" {
      return Err(ParseError::invalid_path("Expected /api/... pattern"));
    }

    let version = segments[1].to_string();
    let resource = segments[2].to_string();

    let (id, action) = match segments.len() {
      3 => (None, None),
      4 => {
        if let Ok(id) = segments[3].parse::<u32>() {
          (Some(id), None)
        } else {
          (None, Some(segments[3].to_string()))
        }
      }
      5 => {
        let id = segments[3]
          .parse::<u32>()
          .map_err(|_| ParseError::type_conversion("Invalid ID".to_string()))?;
        (Some(id), Some(segments[4].to_string()))
      }
      _ => return Err(ParseError::invalid_path("Too many path segments")),
    };

    Ok(Self {
      version,
      resource,
      id,
      action,
    })
  }

  fn format(&self) -> String {
    let mut path = format!("/api/{}/{}", self.version, self.resource);

    if let Some(id) = self.id {
      path.push_str(&format!("/{id}"));

      if let Some(ref action) = self.action {
        path.push_str(&format!("/{action}"));
      }
    } else if let Some(ref action) = self.action {
      path.push_str(&format!("/{action}"));
    }

    path
  }

  fn pattern() -> &'static str {
    "/api/:version/:resource[/:id[/:action]]"
  }
}

/// 高级搜索查询，手动实现以测试复杂功能
#[derive(Debug, Clone, PartialEq, Default)]
struct AdvancedSearchQuery {
  q: Option<String>,
  tags: Vec<String>,
  role: Option<UserRole>,
  page: Option<u32>,
  limit: Option<u32>,
  sort_by: Option<String>,
  order: Option<String>,
  active: Option<bool>,
}

impl Query for AdvancedSearchQuery {
  fn parse(query: &str) -> Result<Self, ParseError> {
    let query_map = ruled_router::utils::parse_query_string(query)?;
    Self::from_query_map(&query_map)
  }

  fn format(&self) -> String {
    let mut formatter = ruled_router::formatter::QueryFormatter::new();

    if let Some(ref q) = self.q {
      formatter.set("q", q.clone());
    }

    for tag in &self.tags {
      formatter.add("tags", tag.clone());
    }

    if let Some(ref role) = self.role {
      formatter.set("role", role.to_param());
    }

    if let Some(page) = self.page {
      formatter.set("page", page.to_string());
    }

    if let Some(limit) = self.limit {
      formatter.set("limit", limit.to_string());
    }

    if let Some(ref sort_by) = self.sort_by {
      formatter.set("sort_by", sort_by.clone());
    }

    if let Some(ref order) = self.order {
      formatter.set("order", order.clone());
    }

    if let Some(active) = self.active {
      formatter.set("active", active.to_string());
    }

    formatter.format()
  }

  fn from_query_map(query_map: &HashMap<String, Vec<String>>) -> Result<Self, ParseError> {
    let mut query = Self::default();

    if let Some(q_values) = query_map.get("q") {
      query.q = q_values.first().cloned();
    }

    if let Some(tag_values) = query_map.get("tags") {
      query.tags = tag_values.clone();
    }

    if let Some(role_values) = query_map.get("role") {
      if let Some(role_str) = role_values.first() {
        query.role = Some(UserRole::from_param(role_str)?);
      }
    }

    if let Some(page_values) = query_map.get("page") {
      if let Some(page_str) = page_values.first() {
        query.page = Some(
          page_str
            .parse()
            .map_err(|_| ParseError::type_conversion("Invalid page number".to_string()))?,
        );
      }
    }

    if let Some(limit_values) = query_map.get("limit") {
      if let Some(limit_str) = limit_values.first() {
        query.limit = Some(
          limit_str
            .parse()
            .map_err(|_| ParseError::type_conversion("Invalid limit".to_string()))?,
        );
      }
    }

    if let Some(sort_values) = query_map.get("sort_by") {
      query.sort_by = sort_values.first().cloned();
    }

    if let Some(order_values) = query_map.get("order") {
      query.order = order_values.first().cloned();
    }

    if let Some(active_values) = query_map.get("active") {
      if let Some(active_str) = active_values.first() {
        query.active = Some(
          active_str
            .parse()
            .map_err(|_| ParseError::type_conversion("Invalid active flag".to_string()))?,
        );
      }
    }

    Ok(query)
  }

  fn to_query_string(&self) -> String {
    self.format()
  }
}

/// 完整路由，结合路径和查询
#[derive(Debug, Clone, PartialEq)]
struct FullRoute {
  api: ApiRoute,
  query: AdvancedSearchQuery,
}

impl FullRoute {
  fn parse(url: &str) -> Result<Self, ParseError> {
    let (path, query_str) = ruled_router::utils::split_path_query(url);
    let query_str = query_str.unwrap_or("");

    let api = ApiRoute::parse(path)?;
    let query = AdvancedSearchQuery::parse(query_str)?;

    Ok(Self { api, query })
  }

  fn format(&self) -> String {
    let path = self.api.format();
    let query = self.query.format();

    if query.is_empty() {
      path
    } else {
      format!("{path}?{query}")
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_user_role_conversion() {
    assert_eq!(UserRole::from_param("admin").unwrap(), UserRole::Admin);
    assert_eq!(UserRole::from_param("user").unwrap(), UserRole::User);
    assert_eq!(UserRole::from_param("guest").unwrap(), UserRole::Guest);
    assert!(UserRole::from_param("invalid").is_err());

    assert_eq!(UserRole::Admin.to_param(), "admin");
    assert_eq!(UserRole::User.to_param(), "user");
    assert_eq!(UserRole::Guest.to_param(), "guest");
  }

  #[test]
  fn test_api_route_parsing() {
    let route = ApiRoute::parse("/api/v1/users").unwrap();
    assert_eq!(route.version, "v1");
    assert_eq!(route.resource, "users");
    assert_eq!(route.id, None);
    assert_eq!(route.action, None);

    let route = ApiRoute::parse("/api/v2/posts/123").unwrap();
    assert_eq!(route.version, "v2");
    assert_eq!(route.resource, "posts");
    assert_eq!(route.id, Some(123));
    assert_eq!(route.action, None);

    let route = ApiRoute::parse("/api/v1/users/456/edit").unwrap();
    assert_eq!(route.version, "v1");
    assert_eq!(route.resource, "users");
    assert_eq!(route.id, Some(456));
    assert_eq!(route.action, Some("edit".to_string()));
  }

  #[test]
  fn test_advanced_search_query_parsing() {
    let query_str = "q=rust&tags=programming&tags=tutorial&role=admin&page=2&active=true";
    let query = AdvancedSearchQuery::parse(query_str).unwrap();

    assert_eq!(query.q, Some("rust".to_string()));
    assert_eq!(query.tags, vec!["programming", "tutorial"]);
    assert_eq!(query.role, Some(UserRole::Admin));
    assert_eq!(query.page, Some(2));
    assert_eq!(query.active, Some(true));
  }

  #[test]
  fn test_full_route_roundtrip() {
    let url = "/api/v1/users/123/edit?q=test&role=admin&page=1";
    let route = FullRoute::parse(url).unwrap();

    assert_eq!(route.api.version, "v1");
    assert_eq!(route.api.resource, "users");
    assert_eq!(route.api.id, Some(123));
    assert_eq!(route.api.action, Some("edit".to_string()));
    assert_eq!(route.query.q, Some("test".to_string()));
    assert_eq!(route.query.role, Some(UserRole::Admin));
    assert_eq!(route.query.page, Some(1));

    let formatted = route.format();
    let reparsed = FullRoute::parse(&formatted).unwrap();
    assert_eq!(route.api.version, reparsed.api.version);
    assert_eq!(route.api.resource, reparsed.api.resource);
    assert_eq!(route.api.id, reparsed.api.id);
    assert_eq!(route.api.action, reparsed.api.action);
  }

  #[test]
  fn test_error_handling() {
    // 测试无效的 API 路由
    assert!(ApiRoute::parse("/invalid").is_err());
    assert!(ApiRoute::parse("/api").is_err());
    assert!(ApiRoute::parse("/api/v1").is_err());

    // 测试无效的用户角色
    let query_str = "role=invalid_role";
    assert!(AdvancedSearchQuery::parse(query_str).is_err());

    // 测试无效的数字
    let query_str = "page=not_a_number";
    assert!(AdvancedSearchQuery::parse(query_str).is_err());
  }

  #[test]
  fn test_url_encoding() {
    let query_str = "q=hello%20world&tags=rust%2Bprogramming";
    let query = AdvancedSearchQuery::parse(query_str).unwrap();
    assert_eq!(query.q, Some("hello world".to_string()));
    assert_eq!(query.tags, vec!["rust+programming"]);
  }

  #[test]
  fn test_empty_and_default_values() {
    let query = AdvancedSearchQuery::parse("").unwrap();
    assert_eq!(query, AdvancedSearchQuery::default());

    let formatted = query.format();
    assert!(formatted.is_empty() || formatted == "?");
  }
}
