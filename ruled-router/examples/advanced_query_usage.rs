//! 高级 Query 使用示例
//!
//! 这个示例展示了如何在实际应用中结合使用 Router 和 Query 派生宏：
//! - 路由与查询参数的组合使用
//! - 复杂的 Web API 路由设计
//! - 查询参数验证和默认值
//! - 实际的 URL 解析和构建场景

use ruled_router::prelude::*;
use std::collections::HashMap;

// ===== 用户管理 API =====

/// 用户路由
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/api/v1/users/:id")]
struct UserRoute {
    id: u32,
}

/// 用户查询参数
#[derive(Debug, Clone, PartialEq, Default, Query)]
struct UserQuery {
    /// 包含的字段
    include: Vec<String>,
    /// 是否包含敏感信息
    include_sensitive: Option<bool>,
    /// 响应格式
    format: Option<String>,
}

// ===== 博客文章 API =====

/// 博客文章路由
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/blog/:year/:month/:slug")]
struct BlogPostRoute {
    year: u32,
    month: u32,
    slug: String,
}

/// 博客文章查询参数
#[derive(Debug, Clone, PartialEq, Default, Query)]
struct BlogPostQuery {
    /// 是否包含评论
    include_comments: Option<bool>,
    /// 评论排序方式
    comment_sort: Option<String>,
    /// 评论分页
    comment_page: Option<u32>,
    /// 是否包含草稿
    include_draft: Option<bool>,
}

// ===== 搜索 API =====

/// 搜索路由
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/api/:version/search")]
struct SearchRoute {
    version: String,
}

/// 搜索查询参数
#[derive(Debug, Clone, PartialEq, Default, Query)]
struct SearchQuery {
    /// 搜索关键词
    q: String,
    /// 搜索类型
    search_type: Option<String>,
    /// 分类过滤
    categories: Vec<String>,
    /// 标签过滤
    tags: Vec<String>,
    /// 分页
    page: Option<u32>,
    /// 每页数量
    per_page: Option<u32>,
    /// 排序字段
    sort_by: Option<String>,
    /// 排序顺序
    order: Option<String>,
    /// 日期范围
    date_from: Option<String>,
    date_to: Option<String>,
    /// 价格范围
    price_min: Option<f64>,
    price_max: Option<f64>,
    /// 布尔过滤器
    in_stock: Option<bool>,
    featured: Option<bool>,
    on_sale: Option<bool>,
}

// ===== 电商产品 API =====

/// 产品路由
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/api/v1/products/:id")]
struct ProductRoute {
    id: u64,
}

/// 产品查询参数
#[derive(Debug, Clone, PartialEq, Default, Query)]
struct ProductQuery {
    /// 包含的关联数据
    include: Vec<String>,
    /// 变体选择
    variant: Option<String>,
    /// 地区
    region: Option<String>,
    /// 货币
    currency: Option<String>,
}

/// 产品列表路由
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/api/v1/categories/:category/products")]
struct ProductListRoute {
    category: String,
}

/// 产品列表查询参数
#[derive(Debug, Clone, PartialEq, Default, Query)]
struct ProductListQuery {
    /// 分页
    page: Option<u32>,
    limit: Option<u32>,
    /// 排序
    sort: Option<String>,
    order: Option<String>,
    /// 过滤器
    brand: Vec<String>,
    color: Vec<String>,
    size: Vec<String>,
    /// 价格范围
    min_price: Option<f64>,
    max_price: Option<f64>,
    /// 评分过滤
    min_rating: Option<f32>,
    /// 库存状态
    in_stock: Option<bool>,
    /// 促销状态
    on_sale: Option<bool>,
    /// 新品标识
    is_new: Option<bool>,
}

// ===== 实用函数 =====

/// 解析完整的 URL（路径 + 查询参数）
fn parse_full_url<R, Q>(url: &str) -> Result<(R, Q), Box<dyn std::error::Error>>
where
    R: Router,
    Q: Query,
{
    let (path, query) = ruled_router::utils::split_path_query(url);
    let route = R::parse(path)?;
    let query_params = Q::parse(query.unwrap_or(""))?;
    Ok((route, query_params))
}

/// 构建完整的 URL
fn build_full_url<R, Q>(route: &R, query: &Q) -> String
where
    R: Router,
    Q: Query,
{
    let path = route.format();
    let query_string = query.format();
    if query_string.is_empty() {
        path
    } else {
        format!("{}?{}", path, query_string)
    }
}

/// 应用查询参数的默认值
fn apply_search_defaults(mut query: SearchQuery) -> SearchQuery {
    if query.page.is_none() {
        query.page = Some(1);
    }
    if query.per_page.is_none() {
        query.per_page = Some(20);
    }
    if query.sort_by.is_none() {
        query.sort_by = Some("relevance".to_string());
    }
    if query.order.is_none() {
        query.order = Some("desc".to_string());
    }
    query
}

/// 验证搜索查询参数
fn validate_search_query(query: &SearchQuery) -> Result<(), String> {
    if query.q.trim().is_empty() {
        return Err("搜索关键词不能为空".to_string());
    }
    
    if let Some(page) = query.page {
        if page == 0 {
            return Err("页码必须大于 0".to_string());
        }
    }
    
    if let Some(per_page) = query.per_page {
        if per_page == 0 || per_page > 100 {
            return Err("每页数量必须在 1-100 之间".to_string());
        }
    }
    
    if let Some(price_min) = query.price_min {
        if price_min < 0.0 {
            return Err("最低价格不能为负数".to_string());
        }
    }
    
    if let Some(price_max) = query.price_max {
        if price_max < 0.0 {
            return Err("最高价格不能为负数".to_string());
        }
    }
    
    if let (Some(min), Some(max)) = (query.price_min, query.price_max) {
        if min > max {
            return Err("最低价格不能大于最高价格".to_string());
        }
    }
    
    Ok(())
}

fn main() {
    println!("=== 高级 Query 使用示例 ===");
    
    // 1. 用户 API 示例
    println!("\n1. 用户 API 示例:");
    
    let user_url = "/api/v1/users/123?include=profile&include=preferences&include_sensitive=true&format=json";
    println!("  URL: {}", user_url);
    
    match parse_full_url::<UserRoute, UserQuery>(user_url) {
        Ok((route, query)) => {
            println!("  路由: {:#?}", route);
            println!("  查询: {:#?}", query);
            
            let reconstructed = build_full_url(&route, &query);
            println!("  重构: {}", reconstructed);
        }
        Err(e) => println!("  错误: {}", e),
    }
    
    // 2. 博客文章 API 示例
    println!("\n2. 博客文章 API 示例:");
    
    let blog_url = "/blog/2024/03/rust-async-programming?include_comments=true&comment_sort=newest&comment_page=1";
    println!("  URL: {}", blog_url);
    
    match parse_full_url::<BlogPostRoute, BlogPostQuery>(blog_url) {
        Ok((route, query)) => {
            println!("  路由: {:#?}", route);
            println!("  查询: {:#?}", query);
            
            let reconstructed = build_full_url(&route, &query);
            println!("  重构: {}", reconstructed);
        }
        Err(e) => println!("  错误: {}", e),
    }
    
    // 3. 搜索 API 示例
    println!("\n3. 搜索 API 示例:");
    
    let search_url = "/api/v1/search?q=rust%20programming&categories=tutorial&categories=book&tags=async&tags=web&page=2&per_page=15&sort_by=popularity&order=desc&price_min=10.0&price_max=50.0&in_stock=true&featured=true";
    println!("  URL: {}", search_url);
    
    match parse_full_url::<SearchRoute, SearchQuery>(search_url) {
        Ok((route, mut query)) => {
            println!("  路由: {:#?}", route);
            println!("  原始查询: {:#?}", query);
            
            // 验证查询参数
            match validate_search_query(&query) {
                Ok(()) => println!("  ✓ 查询参数验证通过"),
                Err(e) => println!("  ✗ 查询参数验证失败: {}", e),
            }
            
            // 应用默认值
            query = apply_search_defaults(query);
            println!("  应用默认值后: {:#?}", query);
            
            let reconstructed = build_full_url(&route, &query);
            println!("  重构: {}", reconstructed);
        }
        Err(e) => println!("  错误: {}", e),
    }
    
    // 4. 产品 API 示例
    println!("\n4. 产品 API 示例:");
    
    let product_url = "/api/v1/products/456789?include=reviews&include=variants&variant=red-large&region=us&currency=usd";
    println!("  URL: {}", product_url);
    
    match parse_full_url::<ProductRoute, ProductQuery>(product_url) {
        Ok((route, query)) => {
            println!("  路由: {:#?}", route);
            println!("  查询: {:#?}", query);
            
            let reconstructed = build_full_url(&route, &query);
            println!("  重构: {}", reconstructed);
        }
        Err(e) => println!("  错误: {}", e),
    }
    
    // 5. 产品列表 API 示例
    println!("\n5. 产品列表 API 示例:");
    
    let product_list_url = "/api/v1/categories/electronics/products?page=1&limit=24&sort=price&order=asc&brand=apple&brand=samsung&color=black&color=white&size=medium&size=large&min_price=100.0&max_price=1000.0&min_rating=4.0&in_stock=true&on_sale=false&is_new=true";
    println!("  URL: {}", product_list_url);
    
    match parse_full_url::<ProductListRoute, ProductListQuery>(product_list_url) {
        Ok((route, query)) => {
            println!("  路由: {:#?}", route);
            println!("  查询: {:#?}", query);
            
            let reconstructed = build_full_url(&route, &query);
            println!("  重构: {}", reconstructed);
        }
        Err(e) => println!("  错误: {}", e),
    }
    
    // 6. 错误处理示例
    println!("\n6. 错误处理示例:");
    
    let error_cases = vec![
        ("/api/v1/users/abc", "无效的用户 ID"),
        ("/api/v1/search?q=&page=0", "空搜索关键词和无效页码"),
        ("/api/v1/search?q=test&per_page=200", "每页数量超出限制"),
        ("/api/v1/search?q=test&price_min=100&price_max=50", "价格范围无效"),
    ];
    
    for (url, description) in error_cases {
        println!("  测试: {} ({})", url, description);
        
        if url.contains("/users/") {
            match parse_full_url::<UserRoute, UserQuery>(url) {
                Ok((route, query)) => println!("    意外成功: {:?}, {:?}", route, query),
                Err(e) => println!("    预期错误: {}", e),
            }
        } else if url.contains("/search") {
            match parse_full_url::<SearchRoute, SearchQuery>(url) {
                Ok((route, query)) => {
                    println!("    解析成功: {:?}, {:?}", route, query);
                    match validate_search_query(&query) {
                        Ok(()) => println!("    验证通过"),
                        Err(e) => println!("    验证失败: {}", e),
                    }
                }
                Err(e) => println!("    解析错误: {}", e),
            }
        }
    }
    
    // 7. 手动构建复杂查询示例
    println!("\n7. 手动构建复杂查询示例:");
    
    let search_route = SearchRoute {
        version: "v1".to_string(),
    };
    let mut search_query = SearchQuery {
        q: "rust web framework".to_string(),
        search_type: Some("tutorial".to_string()),
        categories: vec!["programming".to_string(), "web".to_string()],
        tags: vec!["rust".to_string(), "actix".to_string(), "tokio".to_string()],
        page: Some(1),
        per_page: Some(25),
        sort_by: Some("rating".to_string()),
        order: Some("desc".to_string()),
        price_min: Some(0.0),
        price_max: Some(100.0),
        in_stock: Some(true),
        featured: Some(true),
        on_sale: Some(false),
        ..Default::default()
    };
    
    println!("  手动构建的查询: {:#?}", search_query);
    
    // 验证
    match validate_search_query(&search_query) {
        Ok(()) => println!("  ✓ 验证通过"),
        Err(e) => println!("  ✗ 验证失败: {}", e),
    }
    
    // 应用默认值
    search_query = apply_search_defaults(search_query);
    
    // 构建 URL
    let full_url = build_full_url(&search_route, &search_query);
    println!("  构建的 URL: {}", full_url);
    
    // 验证往返转换
    match parse_full_url::<SearchRoute, SearchQuery>(&full_url) {
        Ok((reparsed_route, reparsed_query)) => {
            if reparsed_query == search_query {
                println!("  ✓ 往返转换成功");
            } else {
                println!("  ✗ 往返转换失败");
            }
        }
        Err(e) => println!("  ✗ 重新解析失败: {}", e),
    }
    
    // 8. 性能测试
    println!("\n8. 性能测试:");
    
    use std::time::Instant;
    
    let test_url = "/api/v1/search?q=test&categories=a&categories=b&tags=x&tags=y&page=1&per_page=20&sort_by=date&order=desc&price_min=10.0&price_max=100.0&in_stock=true";
    let iterations = 1000;
    
    // 测试完整 URL 解析性能
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = parse_full_url::<SearchRoute, SearchQuery>(test_url).unwrap();
    }
    let parse_duration = start.elapsed();
    
    // 测试 URL 构建性能
    let (route, query) = parse_full_url::<SearchRoute, SearchQuery>(test_url).unwrap();
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = build_full_url(&route, &query);
    }
    let build_duration = start.elapsed();
    
    println!("  {} 次完整解析耗时: {:?} (平均: {:?})", iterations, parse_duration, parse_duration / iterations);
    println!("  {} 次 URL 构建耗时: {:?} (平均: {:?})", iterations, build_duration, build_duration / iterations);
    
    println!("\n=== 高级 Query 使用示例完成 ===");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_user_api_full_cycle() {
        let url = "/api/v1/users/123?include=profile&include_sensitive=true";
        let (route, query) = parse_full_url::<UserRoute, UserQuery>(url).unwrap();
        
        assert_eq!(route.id, 123);
        assert_eq!(query.include, vec!["profile"]);
        assert_eq!(query.include_sensitive, Some(true));
        
        let reconstructed = build_full_url(&route, &query);
        let (reparsed_route, reparsed_query) = parse_full_url::<UserRoute, UserQuery>(&reconstructed).unwrap();
        
        assert_eq!(route, reparsed_route);
        assert_eq!(query, reparsed_query);
    }
    
    #[test]
    fn test_blog_post_api_full_cycle() {
        let url = "/blog/2024/03/rust-tutorial?include_comments=true&comment_sort=newest";
        let (route, query) = parse_full_url::<BlogPostRoute, BlogPostQuery>(url).unwrap();
        
        assert_eq!(route.year, 2024);
        assert_eq!(route.month, 3);
        assert_eq!(route.slug, "rust-tutorial");
        assert_eq!(query.include_comments, Some(true));
        assert_eq!(query.comment_sort, Some("newest".to_string()));
        
        let reconstructed = build_full_url(&route, &query);
        let (reparsed_route, reparsed_query) = parse_full_url::<BlogPostRoute, BlogPostQuery>(&reconstructed).unwrap();
        
        assert_eq!(route, reparsed_route);
        assert_eq!(query, reparsed_query);
    }
    
    #[test]
    fn test_search_query_validation() {
        // 有效查询
        let valid_query = SearchQuery {
            q: "rust".to_string(),
            page: Some(1),
            per_page: Some(20),
            price_min: Some(10.0),
            price_max: Some(100.0),
            ..Default::default()
        };
        assert!(validate_search_query(&valid_query).is_ok());
        
        // 无效查询 - 空关键词
        let invalid_query = SearchQuery {
            q: "".to_string(),
            ..Default::default()
        };
        assert!(validate_search_query(&invalid_query).is_err());
        
        // 无效查询 - 页码为 0
        let invalid_query = SearchQuery {
            q: "test".to_string(),
            page: Some(0),
            ..Default::default()
        };
        assert!(validate_search_query(&invalid_query).is_err());
        
        // 无效查询 - 价格范围错误
        let invalid_query = SearchQuery {
            q: "test".to_string(),
            price_min: Some(100.0),
            price_max: Some(50.0),
            ..Default::default()
        };
        assert!(validate_search_query(&invalid_query).is_err());
    }
    
    #[test]
    fn test_search_defaults() {
        let mut query = SearchQuery {
            q: "test".to_string(),
            ..Default::default()
        };
        
        query = apply_search_defaults(query);
        
        assert_eq!(query.page, Some(1));
        assert_eq!(query.per_page, Some(20));
        assert_eq!(query.sort_by, Some("relevance".to_string()));
        assert_eq!(query.order, Some("desc".to_string()));
    }
    
    #[test]
    fn test_product_list_complex_query() {
        let url = "/api/v1/categories/electronics/products?brand=apple&brand=samsung&color=black&min_price=100.0&max_price=1000.0&in_stock=true";
        let (route, query) = parse_full_url::<ProductListRoute, ProductListQuery>(url).unwrap();
        
        assert_eq!(route.category, "electronics");
        assert_eq!(query.brand, vec!["apple", "samsung"]);
        assert_eq!(query.color, vec!["black"]);
        assert_eq!(query.min_price, Some(100.0));
        assert_eq!(query.max_price, Some(1000.0));
        assert_eq!(query.in_stock, Some(true));
    }
    
    #[test]
    fn test_empty_query_handling() {
        let url = "/api/v1/search?q=test";
        let (route, query) = parse_full_url::<SearchRoute, SearchQuery>(url).unwrap();
        
        assert_eq!(query.q, "test");
        assert_eq!(query.page, None);
        assert_eq!(query.categories, Vec::<String>::new());
        
        let reconstructed = build_full_url(&route, &query);
        assert!(reconstructed.contains("q=test"));
    }
}