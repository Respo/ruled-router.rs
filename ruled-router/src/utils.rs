//! 工具函数
//!
//! 提供 URL 编码/解码、路径分离等基础功能

use crate::error::{ParseError, ParseResult};
use std::collections::HashMap;

/// URL 编码函数
/// 
/// 将字符串进行 URL 编码，遵循 RFC 3986 标准
/// 
/// # 参数
/// 
/// * `input` - 要编码的字符串
/// 
/// # 返回值
/// 
/// URL 编码后的字符串
/// 
/// # 示例
/// 
/// ```rust
/// use ruled_router::utils::url_encode;
/// 
/// let encoded = url_encode("hello world");
/// assert_eq!(encoded, "hello%20world");
/// ```
pub fn url_encode(input: &str) -> String {
    input
        .chars()
        .map(|c| match c {
            // 不需要编码的字符（RFC 3986 unreserved characters）
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
            // 空格编码为 %20（而不是 +，这是 application/x-www-form-urlencoded 的规则）
            ' ' => "%20".to_string(),
            // 其他字符进行百分号编码
            _ => {
                let mut buf = [0; 4];
                let encoded = c.encode_utf8(&mut buf);
                encoded
                    .bytes()
                    .map(|b| format!("%{b:02X}"))
                    .collect::<String>()
            }
        })
        .collect()
}

/// URL 解码函数
/// 
/// 将 URL 编码的字符串解码为原始字符串
/// 
/// # 参数
/// 
/// * `input` - 要解码的字符串
/// 
/// # 返回值
/// 
/// 解码后的字符串，如果解码失败则返回错误
/// 
/// # 示例
/// 
/// ```rust
/// use ruled_router::utils::url_decode;
/// 
/// let decoded = url_decode("hello%20world").unwrap();
/// assert_eq!(decoded, "hello world");
/// ```
pub fn url_decode(input: &str) -> ParseResult<String> {
    let mut result = Vec::new();
    let mut chars = input.chars().peekable();
    
    while let Some(c) = chars.next() {
        match c {
            '%' => {
                // 读取两个十六进制字符
                let hex1 = chars.next().ok_or_else(|| {
                    ParseError::url_encoding("Incomplete percent encoding")
                })?;
                let hex2 = chars.next().ok_or_else(|| {
                    ParseError::url_encoding("Incomplete percent encoding")
                })?;
                
                let hex_str = format!("{hex1}{hex2}");
                let byte = u8::from_str_radix(&hex_str, 16).map_err(|_| {
                    ParseError::url_encoding(format!("Invalid hex in percent encoding: {hex_str}"))
                })?;
                
                result.push(byte);
            }
            '+' => {
                // 在查询参数中，+ 表示空格
                result.push(b' ');
            }
            _ => {
                // 普通字符，转换为 UTF-8 字节
                let mut buf = [0; 4];
                let encoded = c.encode_utf8(&mut buf);
                result.extend_from_slice(encoded.as_bytes());
            }
        }
    }
    
    String::from_utf8(result).map_err(|_| {
        ParseError::url_encoding("Invalid UTF-8 sequence after URL decoding")
    })
}

/// 分离路径和查询参数
/// 
/// 将完整的 URL 路径分离为路径部分和查询参数部分
/// 
/// # 参数
/// 
/// * `url` - 完整的 URL 路径字符串
/// 
/// # 返回值
/// 
/// 返回元组 (路径部分, 查询参数部分)，如果没有查询参数则第二个元素为 None
/// 
/// # 示例
/// 
/// ```rust
/// use ruled_router::utils::split_path_query;
/// 
/// let (path, query) = split_path_query("/user/123?tab=profile&edit=true");
/// assert_eq!(path, "/user/123");
/// assert_eq!(query, Some("tab=profile&edit=true"));
/// 
/// let (path, query) = split_path_query("/user/123");
/// assert_eq!(path, "/user/123");
/// assert_eq!(query, None);
/// ```
pub fn split_path_query(url: &str) -> (&str, Option<&str>) {
    if let Some(question_pos) = url.find('?') {
        let path = &url[..question_pos];
        let query = &url[question_pos + 1..];
        (path, Some(query))
    } else {
        (url, None)
    }
}

/// 将路径分解为段
/// 
/// 将路径字符串分解为各个段，忽略空段
/// 
/// # 参数
/// 
/// * `path` - 路径字符串
/// 
/// # 返回值
/// 
/// 路径段的向量
/// 
/// # 示例
/// 
/// ```rust
/// use ruled_router::utils::split_path_segments;
/// 
/// let segments = split_path_segments("/user/123/profile");
/// assert_eq!(segments, vec!["user", "123", "profile"]);
/// 
/// let segments = split_path_segments("/");
/// assert_eq!(segments, Vec::<&str>::new());
/// ```
pub fn split_path_segments(path: &str) -> Vec<&str> {
    path.trim_start_matches('/')
        .split('/')
        .filter(|s| !s.is_empty())
        .collect()
}

/// 解析查询字符串为参数映射
/// 
/// 将查询字符串解析为键值对映射，支持多值参数
/// 
/// # 参数
/// 
/// * `query` - 查询字符串，不包含前导的 '?'
/// 
/// # 返回值
/// 
/// 参数映射，每个键对应一个值的向量（支持多值参数）
/// 
/// # 示例
/// 
/// ```rust
/// use ruled_router::utils::parse_query_string;
/// 
/// let params = parse_query_string("q=rust&page=2&tags=web&tags=backend").unwrap();
/// assert_eq!(params.get("q"), Some(&vec!["rust".to_string()]));
/// assert_eq!(params.get("page"), Some(&vec!["2".to_string()]));
/// assert_eq!(params.get("tags"), Some(&vec!["web".to_string(), "backend".to_string()]));
/// ```
pub fn parse_query_string(query: &str) -> ParseResult<HashMap<String, Vec<String>>> {
    let mut params = HashMap::new();
    
    if query.is_empty() {
        return Ok(params);
    }
    
    for pair in query.split('&') {
        if pair.is_empty() {
            continue;
        }
        
        let (key, value) = if let Some(eq_pos) = pair.find('=') {
            let key = &pair[..eq_pos];
            let value = &pair[eq_pos + 1..];
            (url_decode(key)?, url_decode(value)?)
        } else {
            // 没有等号的参数，值为空字符串
            (url_decode(pair)?, String::new())
        };
        
        params.entry(key).or_insert_with(Vec::new).push(value);
    }
    
    Ok(params)
}

/// 将参数映射格式化为查询字符串
/// 
/// 将参数映射转换为查询字符串格式
/// 
/// # 参数
/// 
/// * `params` - 参数映射
/// 
/// # 返回值
/// 
/// 格式化后的查询字符串，不包含前导的 '?'
/// 
/// # 示例
/// 
/// ```rust
/// use ruled_router::utils::format_query_string;
/// use std::collections::HashMap;
/// 
/// let mut params = HashMap::new();
/// params.insert("q".to_string(), vec!["rust".to_string()]);
/// params.insert("tags".to_string(), vec!["web".to_string(), "backend".to_string()]);
/// 
/// let query = format_query_string(&params);
/// // 注意：HashMap 的迭代顺序不确定，所以这里只检查包含关系
/// assert!(query.contains("q=rust"));
/// assert!(query.contains("tags=web"));
/// assert!(query.contains("tags=backend"));
/// ```
pub fn format_query_string(params: &HashMap<String, Vec<String>>) -> String {
    let mut parts = Vec::new();
    
    for (key, values) in params {
        for value in values {
            if value.is_empty() {
                parts.push(url_encode(key));
            } else {
                parts.push(format!("{}={}", url_encode(key), url_encode(value)));
            }
        }
    }
    
    parts.join("&")
}

/// 规范化路径
/// 
/// 移除路径中的多余斜杠，确保路径格式一致
/// 
/// # 参数
/// 
/// * `path` - 原始路径
/// 
/// # 返回值
/// 
/// 规范化后的路径
/// 
/// # 示例
/// 
/// ```rust
/// use ruled_router::utils::normalize_path;
/// 
/// assert_eq!(normalize_path("//user///123//profile/"), "/user/123/profile");
/// assert_eq!(normalize_path("/"), "/");
/// assert_eq!(normalize_path(""), "/");
/// ```
pub fn normalize_path(path: &str) -> String {
    if path.is_empty() {
        return "/".to_string();
    }
    
    let segments = split_path_segments(path);
    if segments.is_empty() {
        "/".to_string()
    } else {
        format!("/{}", segments.join("/"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_encode() {
        assert_eq!(url_encode("hello world"), "hello%20world");
        assert_eq!(url_encode("user@example.com"), "user%40example.com");
        assert_eq!(url_encode("safe-chars_123.~"), "safe-chars_123.~");
        assert_eq!(url_encode("中文"), "%E4%B8%AD%E6%96%87");
    }

    #[test]
    fn test_url_decode() {
        assert_eq!(url_decode("hello%20world").unwrap(), "hello world");
        assert_eq!(url_decode("user%40example.com").unwrap(), "user@example.com");
        assert_eq!(url_decode("safe-chars_123.~").unwrap(), "safe-chars_123.~");
        assert_eq!(url_decode("%E4%B8%AD%E6%96%87").unwrap(), "中文");
        assert_eq!(url_decode("hello+world").unwrap(), "hello world");
        
        // 测试错误情况
        assert!(url_decode("%ZZ").is_err());
        assert!(url_decode("%1").is_err());
    }

    #[test]
    fn test_split_path_query() {
        let (path, query) = split_path_query("/user/123?tab=profile");
        assert_eq!(path, "/user/123");
        assert_eq!(query, Some("tab=profile"));
        
        let (path, query) = split_path_query("/user/123");
        assert_eq!(path, "/user/123");
        assert_eq!(query, None);
        
        let (path, query) = split_path_query("/?empty");
        assert_eq!(path, "/");
        assert_eq!(query, Some("empty"));
    }

    #[test]
    fn test_split_path_segments() {
        assert_eq!(split_path_segments("/user/123/profile"), vec!["user", "123", "profile"]);
        assert_eq!(split_path_segments("/"), Vec::<&str>::new());
        assert_eq!(split_path_segments(""), Vec::<&str>::new());
        assert_eq!(split_path_segments("user/123"), vec!["user", "123"]);
    }

    #[test]
    fn test_parse_query_string() {
        let params = parse_query_string("q=rust&page=2&tags=web&tags=backend").unwrap();
        assert_eq!(params.get("q"), Some(&vec!["rust".to_string()]));
        assert_eq!(params.get("page"), Some(&vec!["2".to_string()]));
        assert_eq!(params.get("tags"), Some(&vec!["web".to_string(), "backend".to_string()]));
        
        let params = parse_query_string("").unwrap();
        assert!(params.is_empty());
        
        let params = parse_query_string("flag").unwrap();
        assert_eq!(params.get("flag"), Some(&vec!["".to_string()]));
    }

    #[test]
    fn test_normalize_path() {
        assert_eq!(normalize_path("//user///123//profile/"), "/user/123/profile");
        assert_eq!(normalize_path("/"), "/");
        assert_eq!(normalize_path(""), "/");
        assert_eq!(normalize_path("user/123"), "/user/123");
    }
}