//! Query 派生宏使用示例
//!
//! 这个示例专门展示 #[derive(Query)] 派生宏的各种用法：
//! - 基本查询参数解析
//! - 可选参数处理
//! - 数组参数处理
//! - 布尔值参数处理
//! - 数字类型参数处理
//! - 复杂查询结构
//! - 查询参数格式化
//! - 错误处理

use ruled_router::prelude::*;

/// 基础搜索查询
#[derive(Debug, Clone, PartialEq, Default, Query)]
struct SearchQuery {
    /// 搜索关键词
    q: Option<String>,
    /// 页码
    page: Option<u32>,
    /// 每页数量
    limit: Option<u32>,
    /// 标签列表
    tags: Vec<String>,
}

/// 过滤查询
#[derive(Debug, Clone, PartialEq, Default, Query)]
struct FilterQuery {
    /// 是否激活
    active: Option<bool>,
    /// 排序字段
    sort_by: Option<String>,
    /// 排序顺序
    order: Option<String>,
    /// 分类列表
    categories: Vec<String>,
    /// 最小价格
    min_price: Option<f64>,
    /// 最大价格
    max_price: Option<f64>,
}

/// 用户偏好查询
#[derive(Debug, Clone, PartialEq, Default, Query)]
struct UserPreferencesQuery {
    /// 主题
    theme: Option<String>,
    /// 语言
    lang: Option<String>,
    /// 时区
    timezone: Option<String>,
    /// 通知设置
    notifications: Option<bool>,
}

/// 分页查询
#[derive(Debug, Clone, PartialEq, Default, Query)]
struct PaginationQuery {
    /// 页码（从1开始）
    page: Option<u32>,
    /// 每页数量
    per_page: Option<u32>,
    /// 偏移量
    offset: Option<u32>,
}

/// 复杂查询 - 包含多种类型的参数
#[derive(Debug, Clone, PartialEq, Default, Query)]
struct ComplexQuery {
    // 字符串参数
    query: Option<String>,
    title: Option<String>,
    author: Option<String>,
    
    // 数字参数
    page: Option<u32>,
    limit: Option<u32>,
    min_rating: Option<f32>,
    max_rating: Option<f32>,
    year: Option<i32>,
    
    // 布尔参数
    published: Option<bool>,
    featured: Option<bool>,
    free: Option<bool>,
    
    // 数组参数
    tags: Vec<String>,
    categories: Vec<String>,
    authors: Vec<String>,
    
    // 枚举风格的字符串
    sort: Option<String>,
    format: Option<String>,
    status: Option<String>,
}

fn main() {
    println!("=== Query 派生宏使用示例 ===");
    
    // 1. 基础搜索查询示例
    println!("\n1. 基础搜索查询示例:");
    
    let search_query_str = "q=rust&page=1&limit=10&tags=tutorial&tags=beginner";
    println!("  查询字符串: {search_query_str}");
    
    match SearchQuery::parse(search_query_str) {
        Ok(query) => {
            println!("  解析结果: {query:#?}");
            
            let formatted = query.format();
            println!("  格式化: {formatted}");
            
            // 验证往返转换
            match SearchQuery::parse(&formatted) {
                Ok(reparsed) => {
                    if reparsed == query {
                        println!("  ✓ 往返转换成功");
                    } else {
                        println!("  ✗ 往返转换失败");
                    }
                }
                Err(e) => println!("  ✗ 重新解析失败: {e:?}"),
            }
        }
        Err(e) => println!("  解析错误: {e:?}"),
    }
    
    // 2. 过滤查询示例
    println!("\n2. 过滤查询示例:");
    
    let filter_query_str = "active=true&sort_by=date&order=desc&categories=tech&categories=programming&min_price=10.5&max_price=99.99";
    println!("  查询字符串: {filter_query_str}");
    
    match FilterQuery::parse(filter_query_str) {
        Ok(query) => {
            println!("  解析结果: {query:#?}");
            
            let formatted = query.format();
            println!("  格式化: {formatted}");
        }
        Err(e) => println!("  解析错误: {e:?}"),
    }
    
    // 3. 用户偏好查询示例
    println!("\n3. 用户偏好查询示例:");
    
    let prefs_query_str = "theme=dark&lang=zh-CN&timezone=Asia%2FShanghai&notifications=false";
    println!("  查询字符串: {prefs_query_str}");
    
    match UserPreferencesQuery::parse(prefs_query_str) {
        Ok(query) => {
            println!("  解析结果: {query:#?}");
            
            let formatted = query.format();
            println!("  格式化: {formatted}");
        }
        Err(e) => println!("  解析错误: {e:?}"),
    }
    
    // 4. 分页查询示例
    println!("\n4. 分页查询示例:");
    
    let pagination_query_str = "page=2&per_page=20";
    println!("  查询字符串: {pagination_query_str}");
    
    match PaginationQuery::parse(pagination_query_str) {
        Ok(query) => {
            println!("  解析结果: {query:#?}");
            
            let formatted = query.format();
            println!("  格式化: {formatted}");
        }
        Err(e) => println!("  解析错误: {e:?}"),
    }
    
    // 5. 复杂查询示例
    println!("\n5. 复杂查询示例:");
    
    let complex_query_str = "query=advanced%20rust&title=async&author=tokio&page=1&limit=20&min_rating=4.0&max_rating=5.0&year=2023&published=true&featured=false&free=true&tags=async&tags=tokio&tags=performance&categories=tutorial&categories=advanced&authors=alice&authors=bob&sort=popularity&format=pdf&status=published";
    println!("  查询字符串: {complex_query_str}");
    
    match ComplexQuery::parse(complex_query_str) {
        Ok(query) => {
            println!("  解析结果: {query:#?}");
            
            let formatted = query.format();
            println!("  格式化: {formatted}");
        }
        Err(e) => println!("  解析错误: {e:?}"),
    }
    
    // 6. 空查询示例
    println!("\n6. 空查询示例:");
    
    let empty_query_str = "";
    println!("  查询字符串: '{empty_query_str}'");
    
    match SearchQuery::parse(empty_query_str) {
        Ok(query) => {
            println!("  解析结果: {query:#?}");
            
            let formatted = query.format();
            println!("  格式化: '{formatted}'");
        }
        Err(e) => println!("  解析错误: {e:?}"),
    }
    
    // 7. 错误处理示例
    println!("\n7. 错误处理示例:");
    
    let error_cases = vec![
        ("page=abc", "无效的数字"),
        ("active=maybe", "无效的布尔值"),
        ("min_price=not_a_number", "无效的浮点数"),
        ("malformed=query&", "格式错误的查询"),
    ];
    
    for (query_str, description) in error_cases {
        println!("  测试: {query_str} ({description})");
        match SearchQuery::parse(query_str) {
            Ok(query) => println!("    意外成功: {query:#?}"),
            Err(e) => println!("    预期错误: {e:?}"),
        }
    }
    
    // 8. 手动构建查询示例
    println!("\n8. 手动构建查询示例:");
    
    let manual_query = ComplexQuery {
        query: Some("rust programming".to_string()),
        title: Some("async programming".to_string()),
        page: Some(1),
        limit: Some(25),
        min_rating: Some(4.5),
        published: Some(true),
        featured: Some(true),
        tags: vec!["rust".to_string(), "async".to_string(), "tutorial".to_string()],
        categories: vec!["programming".to_string(), "tutorial".to_string()],
        sort: Some("rating".to_string()),
        format: Some("html".to_string()),
        ..Default::default()
    };
    
    println!("  手动构建的查询: {manual_query:#?}");
    
    let formatted = manual_query.format();
    println!("  格式化结果: {formatted}");
    
    // 验证可以重新解析
    match ComplexQuery::parse(&formatted) {
        Ok(reparsed) => {
            if reparsed == manual_query {
                println!("  ✓ 手动构建的查询往返转换成功");
            } else {
                println!("  ✗ 手动构建的查询往返转换失败");
                println!("    原始: {manual_query:#?}");
                println!("    重解析: {reparsed:#?}");
            }
        }
        Err(e) => println!("  ✗ 重新解析手动构建的查询失败: {e:?}"),
    }
    
    // 9. 性能测试示例
    println!("\n9. 性能测试示例:");
    
    use std::time::Instant;
    
    let test_query_str = "q=test&page=1&limit=10&tags=a&tags=b&tags=c&active=true&sort_by=date";
    let iterations = 1000;
    
    // 测试解析性能
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = SearchQuery::parse(test_query_str).unwrap();
    }
    let parse_duration = start.elapsed();
    
    // 测试格式化性能
    let query = SearchQuery::parse(test_query_str).unwrap();
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = query.format();
    }
    let format_duration = start.elapsed();
    
    println!("  {} 次解析耗时: {:?} (平均: {:?})", iterations, parse_duration, parse_duration / iterations);
    println!("  {} 次格式化耗时: {:?} (平均: {:?})", iterations, format_duration, format_duration / iterations);
    
    println!("\n=== Query 派生宏示例完成 ===");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_search_query_parse_and_format() {
        let query_str = "q=rust&page=1&limit=10&tags=tutorial&tags=beginner";
        let query = SearchQuery::parse(query_str).unwrap();
        
        assert_eq!(query.q, Some("rust".to_string()));
        assert_eq!(query.page, Some(1));
        assert_eq!(query.limit, Some(10));
        assert_eq!(query.tags, vec!["tutorial".to_string(), "beginner".to_string()]);
        
        let formatted = query.format();
        let reparsed = SearchQuery::parse(&formatted).unwrap();
        assert_eq!(query, reparsed);
    }
    
    #[test]
    fn test_filter_query_with_floats() {
        let query_str = "active=true&min_price=10.5&max_price=99.99";
        let query = FilterQuery::parse(query_str).unwrap();
        
        assert_eq!(query.active, Some(true));
        assert_eq!(query.min_price, Some(10.5));
        assert_eq!(query.max_price, Some(99.99));
    }
    
    #[test]
    fn test_empty_query() {
        let query = SearchQuery::parse("").unwrap();
        
        assert_eq!(query.q, None);
        assert_eq!(query.page, None);
        assert_eq!(query.limit, None);
        assert!(query.tags.is_empty());
        
        let formatted = query.format();
        assert!(formatted.is_empty());
    }
    
    #[test]
    fn test_complex_query_all_types() {
        let query_str = "query=test&page=1&min_rating=4.5&published=true&tags=rust&tags=async";
        let query = ComplexQuery::parse(query_str).unwrap();
        
        assert_eq!(query.query, Some("test".to_string()));
        assert_eq!(query.page, Some(1));
        assert_eq!(query.min_rating, Some(4.5));
        assert_eq!(query.published, Some(true));
        assert_eq!(query.tags, vec!["rust".to_string(), "async".to_string()]);
    }
    
    #[test]
    fn test_url_encoded_values() {
        let query_str = "query=hello%20world&title=rust%2Basync";
        let query = ComplexQuery::parse(query_str).unwrap();
        
        assert_eq!(query.query, Some("hello world".to_string()));
        assert_eq!(query.title, Some("rust+async".to_string()));
    }
    
    #[test]
    fn test_boolean_parsing() {
        let test_cases = vec![
            ("active=true", Some(true)),
            ("active=false", Some(false)),
            ("active=1", Some(true)),
            ("active=0", Some(false)),
            ("", None),
        ];
        
        for (query_str, expected) in test_cases {
            let query = FilterQuery::parse(query_str).unwrap();
            assert_eq!(query.active, expected, "Failed for: {}", query_str);
        }
    }
    
    #[test]
    fn test_array_parameters() {
        let query_str = "tags=rust&tags=async&tags=tutorial&categories=programming&categories=advanced";
        let query = ComplexQuery::parse(query_str).unwrap();
        
        assert_eq!(query.tags, vec!["rust", "async", "tutorial"]);
        assert_eq!(query.categories, vec!["programming", "advanced"]);
    }
    
    #[test]
    fn test_numeric_types() {
        let query_str = "page=42&min_rating=4.5&year=2023";
        let query = ComplexQuery::parse(query_str).unwrap();
        
        assert_eq!(query.page, Some(42));
        assert_eq!(query.min_rating, Some(4.5));
        assert_eq!(query.year, Some(2023));
    }
}