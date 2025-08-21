//! 完整功能演示
//! 
//! 这个示例展示了 ruled-router 库的所有核心功能：
//! - 使用派生宏自动生成 Router 和 Query trait 实现
//! - 路径参数解析和格式化
//! - 查询参数解析和格式化
//! - 错误处理
//! - 类型转换
//! - URL 构建和解析

use ruled_router::prelude::*;

// 使用派生宏定义路由结构体
#[derive(Debug, Clone, PartialEq, Router)]
#[route(pattern = "/users/:id")]
struct UserRoute {
    id: u32,
}

#[derive(Debug, Clone, PartialEq, Router)]
#[route(pattern = "/blog/:category/:slug")]
struct BlogRoute {
    category: String,
    slug: String,
}

#[derive(Debug, Clone, PartialEq, Router)]
#[route(pattern = "/api/:version/users/:user_id/posts/:post_id")]
struct ApiRoute {
    version: String,
    user_id: u32,
    post_id: u64,
}

// 使用派生宏定义查询结构体
#[derive(Debug, Clone, PartialEq, Query)]
struct SearchQuery {
    q: Option<String>,
    page: Option<u32>,
    limit: Option<u32>,
    tags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Query)]
struct FilterQuery {
    active: Option<bool>,
    sort_by: Option<String>,
    order: Option<String>,
    categories: Vec<String>,
}

// 完整的应用路由枚举
#[derive(Debug, Clone, PartialEq)]
enum AppRoute {
    User(UserRoute),
    Blog(BlogRoute),
    Api(ApiRoute),
    Home,
    NotFound,
}

// 路由匹配器
struct RouteMatcher;

impl RouteMatcher {
    fn match_route(path: &str) -> AppRoute {
        // 尝试匹配用户路由
        if let Ok(user_route) = UserRoute::parse(path) {
            return AppRoute::User(user_route);
        }
        
        // 尝试匹配博客路由
        if let Ok(blog_route) = BlogRoute::parse(path) {
            return AppRoute::Blog(blog_route);
        }
        
        // 尝试匹配 API 路由
        if let Ok(api_route) = ApiRoute::parse(path) {
            return AppRoute::Api(api_route);
        }
        
        // 检查首页
        if path == "/" || path.is_empty() {
            return AppRoute::Home;
        }
        
        AppRoute::NotFound
    }
}

// URL 构建器
struct UrlBuilder;

impl UrlBuilder {
    fn build_user_url(id: u32, query: Option<&SearchQuery>) -> String {
        let route = UserRoute { id };
        let path = route.format();
        
        if let Some(q) = query {
            format!("{}?{}", path, q.format())
        } else {
            path
        }
    }
    
    fn build_blog_url(category: &str, slug: &str, query: Option<&FilterQuery>) -> String {
        let route = BlogRoute {
            category: category.to_string(),
            slug: slug.to_string(),
        };
        let path = route.format();
        
        if let Some(q) = query {
            format!("{}?{}", path, q.format())
        } else {
            path
        }
    }
    
    fn build_api_url(version: &str, user_id: u32, post_id: u64) -> String {
        let route = ApiRoute {
            version: version.to_string(),
            user_id,
            post_id,
        };
        route.format()
    }
}

// 请求处理器
struct RequestHandler;

impl RequestHandler {
    fn handle_request(url: &str) -> String {
        // 分离路径和查询字符串
        let (path, query_string) = if let Some(pos) = url.find('?') {
            (&url[..pos], Some(&url[pos + 1..]))
        } else {
            (url, None)
        };
        
        // 匹配路由
        let route = RouteMatcher::match_route(path);
        
        match route {
            AppRoute::User(user_route) => {
                let mut response = format!("处理用户请求: ID = {}", user_route.id);
                
                // 解析查询参数
                if let Some(qs) = query_string {
                    if let Ok(search_query) = SearchQuery::parse(qs) {
                        response.push_str(&format!("\n查询参数: {search_query:?}"));
                    }
                }
                
                response
            }
            AppRoute::Blog(blog_route) => {
                let mut response = format!(
                    "处理博客请求: 分类 = {}, 文章 = {}",
                    blog_route.category, blog_route.slug
                );
                
                // 解析查询参数
                if let Some(qs) = query_string {
                    if let Ok(filter_query) = FilterQuery::parse(qs) {
                        response.push_str(&format!("\n过滤参数: {filter_query:?}"));
                    }
                }
                
                response
            }
            AppRoute::Api(api_route) => {
                format!(
                    "处理 API 请求: 版本 = {}, 用户ID = {}, 文章ID = {}",
                    api_route.version, api_route.user_id, api_route.post_id
                )
            }
            AppRoute::Home => "欢迎来到首页！".to_string(),
            AppRoute::NotFound => "404 - 页面未找到".to_string(),
        }
    }
}

fn main() {
    println!("=== ruled-router 完整功能演示 ===");
    println!();
    
    // 1. 基本路由匹配演示
    println!("1. 基本路由匹配:");
    let test_urls = vec![
        "/users/123",
        "/blog/technology/rust-tutorial",
        "/api/v1/users/456/posts/789",
        "/",
        "/invalid/path",
    ];
    
    for url in &test_urls {
        let route = RouteMatcher::match_route(url);
        println!("  {url} -> {route:?}");
    }
    println!();
    
    // 2. 带查询参数的请求处理
    println!("2. 请求处理演示:");
    let test_requests = vec![
        "/users/123?q=rust&page=1&tags=tutorial&tags=advanced",
        "/blog/technology/rust-tutorial?active=true&sort_by=date&categories=tech",
        "/api/v2/users/789/posts/456",
        "/",
        "/unknown/route",
    ];
    
    for request in &test_requests {
        let response = RequestHandler::handle_request(request);
        println!("  请求: {request}");
        println!("  响应: {response}");
        println!();
    }
    
    // 3. URL 构建演示
    println!("3. URL 构建演示:");
    
    // 构建用户 URL
    let search_query = SearchQuery {
        q: Some("rust".to_string()),
        page: Some(1),
        limit: Some(10),
        tags: vec!["tutorial".to_string(), "beginner".to_string()],
    };
    let user_url = UrlBuilder::build_user_url(123, Some(&search_query));
    println!("  用户 URL: {user_url}");
    
    // 构建博客 URL
    let filter_query = FilterQuery {
        active: Some(true),
        sort_by: Some("date".to_string()),
        order: Some("desc".to_string()),
        categories: vec!["tech".to_string(), "programming".to_string()],
    };
    let blog_url = UrlBuilder::build_blog_url("technology", "rust-tutorial", Some(&filter_query));
    println!("  博客 URL: {blog_url}");
    
    // 构建 API URL
    let api_url = UrlBuilder::build_api_url("v1", 456, 789);
    println!("  API URL: {api_url}");
    println!();
    
    // 4. 错误处理演示
    println!("4. 错误处理演示:");
    let invalid_paths = vec![
        "/users/abc",  // 无效的数字
        "/users",      // 缺少参数
        "/blog/tech",  // 缺少参数
    ];
    
    for path in &invalid_paths {
        match UserRoute::parse(path) {
            Ok(route) => println!("  {path} -> 成功: {route:?}"),
            Err(e) => println!("  {path} -> 错误: {e:?}"),
        }
    }
    println!();
    
    // 5. 性能测试
    println!("5. 性能测试:");
    let start = std::time::Instant::now();
    let iterations = 10000;
    
    for i in 0..iterations {
        let path = format!("/users/{}", i % 1000);
        let route = UserRoute::parse(&path).unwrap();
        let formatted = route.format();
        assert_eq!(path, formatted);
    }
    
    let duration = start.elapsed();
    println!("  {iterations} 次往返解析: {duration:?}");
    println!("  平均每次: {:?}", duration / iterations);
    println!();
    
    // 6. 类型转换演示
    println!("6. 类型转换演示:");
    
    // 演示不同数字类型的转换
    let api_route = ApiRoute {
        version: "v2".to_string(),
        user_id: 42,
        post_id: 1234567890,
    };
    
    let formatted = api_route.format();
    println!("  格式化: {api_route:?} -> {formatted}");
    
    let parsed = ApiRoute::parse(&formatted).unwrap();
    println!("  解析回来: {formatted} -> {parsed:?}");
    
    assert_eq!(api_route, parsed);
    println!("  ✓ 往返转换成功");
    println!();
    
    println!("=== 演示完成 ===");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_route_matching() {
        assert_eq!(
            RouteMatcher::match_route("/users/123"),
            AppRoute::User(UserRoute { id: 123 })
        );
        
        assert_eq!(
            RouteMatcher::match_route("/blog/tech/rust"),
            AppRoute::Blog(BlogRoute {
                category: "tech".to_string(),
                slug: "rust".to_string(),
            })
        );
        
        assert_eq!(
            RouteMatcher::match_route("/"),
            AppRoute::Home
        );
        
        assert_eq!(
            RouteMatcher::match_route("/invalid"),
            AppRoute::NotFound
        );
    }
    
    #[test]
    fn test_url_building() {
        let query = SearchQuery {
            q: Some("test".to_string()),
            page: Some(1),
            limit: None,
            tags: vec![],
        };
        
        let url = UrlBuilder::build_user_url(123, Some(&query));
        assert!(url.contains("/users/123"));
        assert!(url.contains("q=test"));
        assert!(url.contains("page=1"));
    }
    
    #[test]
    fn test_request_handling() {
        let response = RequestHandler::handle_request("/users/123?q=test");
        assert!(response.contains("处理用户请求"));
        assert!(response.contains("ID = 123"));
    }
}