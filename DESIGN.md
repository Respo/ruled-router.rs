# Ruled Router 技术设计文档

## 1. 设计理念：面向数据编程

### 1.1 核心思想

Ruled Router 采用**面向数据编程**的设计理念，将路由定义视为数据结构，通过类型系统和宏系统实现路由的自动化处理。这种设计具有以下优势：

- **数据即代码**：路由结构直接映射为 Rust 数据结构，类型安全且易于理解
- **自动化生成**：通过派生宏自动生成解析和格式化逻辑，减少样板代码
- **组合性**：支持递归嵌套的路由结构，可以组合出复杂的路由树
- **DRY 原则**：路由信息只需定义一次，自动提取和复用

### 1.2 架构模式：RouteMatcher ↔ Router 交替模式

```
层级 1: RouteMatcher (枚举) → Router (结构体)
层级 2: RouteMatcher (枚举) → Router (结构体)  
层级 3: RouteMatcher (枚举) → Router (结构体)
...
```

这种交替模式确保了：
- **RouteMatcher**：负责路由分发和匹配，使用枚举表示不同的路由选择
- **Router**：负责具体路由的解析和格式化，使用结构体表示路由数据

### 1.3 模块结构

```
ruled-router/
├── ruled-router/           # 主库
│   ├── src/
│   │   ├── lib.rs          # 库入口，导出公共 API
│   │   ├── traits.rs       # 核心 trait 定义
│   │   ├── error.rs        # 错误类型定义
│   │   ├── formatter.rs    # 格式化器实现
│   │   ├── parser/         # 解析器模块
│   │   └── utils.rs        # 工具函数
│   └── examples/
│       └── nested_router_usage.rs  # 完整的嵌套路由示例
└── ruled-router-derive/    # 派生宏库
    └── src/
        ├── lib.rs          # 宏入口
        ├── router_match.rs # RouterMatch 派生宏
        ├── route.rs        # Router 派生宏
        ├── query.rs        # Query 派生宏
        └── querystring.rs  # QueryString 派生宏
```

## 2. 核心 Trait 设计

### 2.1 Router Trait - 路由数据结构

```rust
/// 路由解析和格式化的核心 trait
/// 用于具体的路由结构体，负责路径参数的解析和格式化
pub trait Router: Sized {
    /// 从路径字符串解析路由
    fn parse(path: &str) -> Result<Self, ParseError>;

    /// 支持子路由的解析方法
    fn parse_with_sub(path: &str) -> Result<(Self, Option<String>), ParseError>;

    /// 将路由格式化为路径字符串
    fn format(&self) -> String;

    /// 获取路由模式（用于自动前缀提取）
    fn pattern() -> &'static str;
}
```

### 2.2 RouteMatcher Trait - 路由匹配器

```rust
/// 路由匹配器 trait
/// 用于枚举类型，负责路由分发和匹配
pub trait RouteMatcher: Sized {
    /// 尝试解析路径，返回匹配的路由
    fn try_parse(path: &str) -> Result<Self, ParseError>;

    /// 将路由匹配器格式化为路径字符串
    fn format(&self) -> String;

    /// 获取所有支持的路由模式
    fn patterns() -> Vec<&'static str>;

    /// 支持剩余路径的解析（用于嵌套路由）
    fn try_parse_with_remaining(path: &str) -> Result<(Self, Option<String>), ParseError>;
}
```

### 2.3 查询参数相关 Trait

```rust
/// 查询参数解析和格式化的 trait
pub trait Query: Sized {
    /// 从查询字符串解析参数
    fn parse(query: &str) -> Result<Self, ParseError>;

    /// 将参数格式化为查询字符串
    fn format(&self) -> String;
}

/// 类型转换 trait，用于路径参数的类型转换
pub trait FromParam: Sized {
    fn from_param(param: &str) -> Result<Self, ParseError>;
}

/// 类型格式化 trait，用于将参数转换为字符串
pub trait ToParam {
    fn to_param(&self) -> String;
}
```

## 3. 自动前缀提取系统

### 3.1 设计原理

传统的路由系统需要在多个地方重复定义相同的路径信息：

```rust
// 传统方式 - 路径信息重复定义
#[derive(RouterMatch)]
enum AppRouterMatch {
    #[route("/users")]     // 重复定义路径
    User(UserRoute),
}

#[derive(Router)]
#[router(pattern = "/users")]  // 重复定义路径
struct UserRoute { ... }
```

**自动前缀提取**消除了这种重复，实现了 DRY 原则：

```rust
// 新方式 - 路径信息只定义一次
#[derive(RouterMatch)]
enum AppRouterMatch {
    User(UserRoute),  // 自动从 UserRoute::pattern() 提取前缀
}

#[derive(Router)]
#[router(pattern = "/users")]  // 只在这里定义一次
struct UserRoute { ... }
```

### 3.2 实现机制

RouterMatch 派生宏会自动调用每个变体对应类型的 `Router::pattern()` 方法：

```rust
// 生成的代码（简化版）
impl RouteMatcher for AppRouterMatch {
    fn try_parse(path: &str) -> Result<Self, ParseError> {
        // 自动提取前缀
        let user_prefix = <UserRoute as Router>::pattern(); // "/users"
        
        if path.starts_with(user_prefix) {
            if let Ok((route, sub_router)) = UserRoute::parse_with_sub(path) {
                return Ok(Self::User(route));
            }
        }
        // ...
    }
}
```

## 4. 宏系统设计

### 4.1 Router 派生宏

#### 输入结构体示例

```rust
#[derive(Router)]
#[router(pattern = "/users/:id/profile")]
struct UserProfile {
    id: u32,
    #[query]
    options: ProfileOptions,
    #[sub_router]
    sub_router: Option<UserProfileSubRouterMatch>,
}
```

#### 生成的代码结构

```rust
impl Router for UserProfile {
    fn parse(path: &str) -> Result<Self, ParseError> {
        // 1. 分离路径和查询参数
        let (path_part, query_part) = split_path_query(path);

        // 2. 解析路径参数
        let path_segments = parse_path_segments(path_part)?;
        let route_pattern = "/users/:id/profile";
        let params = match_pattern(route_pattern, &path_segments)?;

        // 3. 提取并转换参数
        let id = u32::from_param(params.get("id").ok_or(ParseError::MissingParameter("id"))?)?;

        // 4. 解析查询参数
        let options = ProfileOptions::parse(query_part.unwrap_or(""))?;

        // 5. 处理子路由（如果存在）
        let sub_router = None; // 在 parse 方法中不处理子路由

        Ok(UserProfile { id, options, sub_router })
    }

    fn parse_with_sub(path: &str) -> Result<(Self, Option<String>), ParseError> {
        // 1. 分离路径和查询参数
        let (path_part, query_part) = split_path_query(path);

        // 2. 匹配当前路由模式
        let route_pattern = "/users/:id/profile";
        let pattern_segments = parse_path_segments(route_pattern)?;
        let path_segments = parse_path_segments(path_part)?;

        // 3. 检查是否有足够的段来匹配当前模式
        if path_segments.len() < pattern_segments.len() {
            return Err(ParseError::InvalidPath("Not enough path segments".to_string()));
        }

        // 4. 匹配当前路由的段
        let current_segments = &path_segments[..pattern_segments.len()];
        let params = match_pattern(route_pattern, current_segments)?;

        // 5. 提取参数
        let id = u32::from_param(params.get("id").ok_or(ParseError::MissingParameter("id"))?)?;

        // 6. 解析查询参数
        let options = ProfileOptions::parse(query_part.unwrap_or(""))?;

        // 7. 计算剩余路径
        let remaining_path = if path_segments.len() > pattern_segments.len() {
            let remaining_segments = &path_segments[pattern_segments.len()..];
            Some("/".to_string() + &remaining_segments.join("/"))
        } else {
            None
        };

        Ok((UserProfile { id, options, sub_router: None }, remaining_path))
    }

    fn format(&self) -> String {
        let mut path = format!("/users/{}/profile", self.id.to_param());
        
        // 添加子路由路径
        if let Some(sub) = &self.sub_router {
            path.push_str(&sub.format());
        }
        
        // 添加查询参数
        let query = self.options.format();
        if !query.is_empty() {
            path.push('?');
            path.push_str(&query);
        }

        path
    }

    fn pattern() -> &'static str {
        "/users/:id/profile"
    }
}
```

### 4.2 RouterMatch 派生宏

#### 输入枚举示例

```rust
#[derive(RouterMatch)]
enum AppRouterMatch {
    User(UserRoute),
    Blog(BlogRoute),
    Api(ApiRoute),
}
```

#### 生成的代码结构（自动前缀提取）

```rust
impl RouteMatcher for AppRouterMatch {
    fn try_parse(path: &str) -> Result<Self, ParseError> {
        // 自动从每个变体的 Router::pattern() 提取前缀
        
        // 尝试匹配 UserRoute
        let user_prefix = <UserRoute as Router>::pattern(); // 例如: "/users"
        if path.starts_with(user_prefix) {
            if let Ok((route, _)) = UserRoute::parse_with_sub(path) {
                return Ok(Self::User(route));
            }
        }
        
        // 尝试匹配 BlogRoute
        let blog_prefix = <BlogRoute as Router>::pattern(); // 例如: "/blog"
        if path.starts_with(blog_prefix) {
            if let Ok((route, _)) = BlogRoute::parse_with_sub(path) {
                return Ok(Self::Blog(route));
            }
        }
        
        // 尝试匹配 ApiRoute
        let api_prefix = <ApiRoute as Router>::pattern(); // 例如: "/api"
        if path.starts_with(api_prefix) {
            if let Ok((route, _)) = ApiRoute::parse_with_sub(path) {
                return Ok(Self::Api(route));
            }
        }
        
        Err(ParseError::InvalidPath("No matching route found".to_string()))
    }
    
    fn try_parse_with_remaining(path: &str) -> Result<(Self, Option<String>), ParseError> {
        // 尝试匹配并返回剩余路径
        
        // 尝试匹配 UserRoute
        let user_prefix = <UserRoute as Router>::pattern();
        if path.starts_with(user_prefix) {
            if let Ok((route, remaining)) = UserRoute::parse_with_sub(path) {
                return Ok((Self::User(route), remaining));
            }
        }
        
        // 类似地处理其他路由...
        
        Err(ParseError::InvalidPath("No matching route found".to_string()))
    }
    
    fn format(&self) -> String {
        match self {
            Self::User(route) => route.format(),
            Self::Blog(route) => route.format(),
            Self::Api(route) => route.format(),
        }
    }
    
    fn patterns() -> Vec<&'static str> {
        vec![
            <UserRoute as Router>::pattern(),
            <BlogRoute as Router>::pattern(),
            <ApiRoute as Router>::pattern(),
        ]
    }
}
```

#### 手动前缀覆盖（可选）

如果需要覆盖自动提取的前缀，可以使用 `route` 属性：

```rust
#[derive(RouterMatch)]
enum AppRouterMatch {
    #[route("/custom-users")] // 覆盖自动提取的前缀
    User(UserRoute),
    Blog(BlogRoute), // 使用自动提取的前缀
    Api(ApiRoute),
}
```

### 4.3 嵌套路由示例

以下是基于 `nested_router_usage.rs` 的完整嵌套路由示例，展示了三层嵌套、自动前缀提取、参数传递等功能：

```rust
use ruled_router::prelude::*;
use ruled_router::error::RouteState;

// 顶层路由匹配器 - 自动前缀提取
#[derive(RouterMatch)]
enum AppRouterMatch {
    User(ModuleRoute),    // 自动提取前缀: "/user"
    Shop(ModuleRoute),    // 自动提取前缀: "/shop" 
    Admin(ModuleRoute),   // 自动提取前缀: "/admin"
}

// 模块路由 - 通用的模块入口
#[derive(Router)]
#[router(pattern = "/:module")]  // 动态模块名
struct ModuleRoute {
    module: String,
    #[sub_router]
    sub_router: RouteState<SubRouterMatch>,
}

// 子路由匹配器 - 自动前缀提取
#[derive(RouterMatch)]
enum SubRouterMatch {
    User(CategoryRoute),     // 用户相关路由
    Shop(CategoryRoute),     // 商店相关路由
    Admin(CategoryRoute),    // 管理相关路由
}

// 分类路由
#[derive(Router)]
#[router(pattern = "/category/:category_id")]
struct CategoryRoute {
    category_id: u32,
    #[query]
    query: SimpleQuery,
    #[sub_router]
    sub_router: RouteState<DetailRouterMatch>,
}

// 详情路由匹配器 - 第三层嵌套
#[derive(RouterMatch)]
enum DetailRouterMatch {
    UserProfile(UserSettingsRoute),
    UserContent(UserPostRoute),
    ShopProduct(ProductDetailRoute),
    ShopOrder(OrderDetailRoute),
    AdminUser(AdminUserManageRoute),
    AdminSystem(SystemConfigRoute),
}

// 具体的详情路由实现
#[derive(Router)]
#[router(pattern = "/settings/:setting_id")]
struct UserSettingsRoute {
    setting_id: u32,
    #[query]
    query: SimpleQuery,
}

#[derive(Router)]
#[router(pattern = "/post/:post_id")]
struct UserPostRoute {
    post_id: u32,
    #[query]
    query: SimpleQuery,
}

#[derive(Router)]
#[router(pattern = "/product/:product_id")]
struct ProductDetailRoute {
    product_id: u32,
    #[query]
    query: SimpleQuery,
}

#[derive(Router)]
#[router(pattern = "/order/:order_id")]
struct OrderDetailRoute {
    order_id: u32,
    #[query]
    query: SimpleQuery,
}

#[derive(Router)]
#[router(pattern = "/user/:user_id")]
struct AdminUserManageRoute {
    user_id: u32,
    #[query]
    query: SimpleQuery,
}

#[derive(Router)]
#[router(pattern = "/config/:config_id")]
struct SystemConfigRoute {
    config_id: u32,
    #[query]
    query: SimpleQuery,
}

// 简单查询参数定义
#[derive(Query)]
struct SimpleQuery {
    #[query(name = "format")]
    format: Option<String>,
    #[query(name = "page", default = "1")]
    page: u32,
}

// 使用示例
fn main() {
    // 解析三层嵌套路由
    let path = "/user/category/123/settings/456?format=json&page=2";

    if let Ok(route) = AppRouterMatch::try_parse(path) {
        match route {
            AppRouterMatch::User(module_route) => {
                println!("模块: {}", module_route.module);
                
                match module_route.sub_router {
                    RouteState::SubRoute(sub) => {
                        match sub {
                            SubRouterMatch::User(category_route) => {
                                println!("分类ID: {}", category_route.category_id);
                                
                                match category_route.sub_router {
                                    RouteState::SubRoute(detail) => {
                                        match detail {
                                            DetailRouterMatch::UserProfile(settings_route) => {
                                                println!("设置ID: {}", settings_route.setting_id);
                                                println!("格式: {:?}", settings_route.query.format);
                                                println!("页码: {}", settings_route.query.page);
                                            }
                                            DetailRouterMatch::ShopProduct(product_route) => {
                                                println!("产品ID: {}", product_route.product_id);
                                            }
                                            _ => {}
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            _ => println!("其他路由"),
        }
    }

    // 格式化路由
    let route = AppRouterMatch::User(
        ModuleRoute {
            module: "user".to_string(),
            sub_router: RouteState::SubRoute(
                SubRouterMatch::User(
                    CategoryRoute {
                        category_id: 123,
                        query: SimpleQuery { 
                            format: Some("json".to_string()),
                            page: 2,
                        },
                        sub_router: RouteState::SubRoute(
                            DetailRouterMatch::UserProfile(
                                UserSettingsRoute {
                                    setting_id: 456,
                                    query: SimpleQuery {
                                        format: Some("json".to_string()),
                                        page: 2,
                                    },
                                }
                            )
                        ),
                    }
                )
            ),
        }
    );

    let formatted = route.format();
    println!("格式化结果: {}", formatted);
    // 输出: /user/category/123/settings/456?format=json&page=2

    // 获取路由模式
    let patterns = AppRouterMatch::patterns();
    for pattern in patterns {
        println!("支持的路由模式: {}", pattern);
    }
}
```

这个示例展示了以下功能：

1. **三层嵌套结构**：`AppRouterMatch > ModuleRoute > SubRouterMatch > DetailRouterMatch`
2. **自动前缀提取**：RouterMatch 枚举自动从对应的 Router 类型提取路由前缀
3. **动态模块路由**：使用参数化的模块名支持多个相似的模块
4. **递归子路由**：每层都可以有可选的子路由，支持任意深度嵌套
5. **查询参数支持**：每个路由都可以有自己的查询参数定义
6. **类型安全**：所有参数都有明确的类型，编译时检查
7. **模式匹配**：可以精确匹配到具体的嵌套路由
8. **格式化输出**：可以将路由结构重新格式化为 URL

### 4.4 Query 派生宏

#### 输入结构体示例

```rust
#[derive(Query)]
struct CategoryQuery {
    #[query(name = "page", default = "1")]
    page: u32,
    #[query(name = "limit", default = "10")]
    limit: u32,
    #[query(name = "sort")]
    sort: Option<String>,
    #[query(name = "filter", multiple)]
    filters: Vec<String>,
}
```

#### 生成的代码结构

```rust
impl Query for CategoryQuery {
    fn parse(query: &str) -> Result<Self, ParseError> {
        let params = parse_query_string(query)?;
        
        let page = params.get("page")
            .and_then(|v| v.first())
            .map(|s| u32::from_param(s))
            .transpose()?
            .unwrap_or(1);
            
        let limit = params.get("limit")
            .and_then(|v| v.first())
            .map(|s| u32::from_param(s))
            .transpose()?
            .unwrap_or(10);
            
        let sort = params.get("sort")
            .and_then(|v| v.first())
            .map(|s| String::from_param(s))
            .transpose()?;
            
        let filters = params.get("filter")
            .map(|v| v.iter().map(|s| s.clone()).collect())
            .unwrap_or_default();
            
        Ok(CategoryQuery {
            page,
            limit,
            sort,
            filters,
        })
    }
    
    fn format(&self) -> String {
        let mut parts = Vec::new();
        
        if self.page != 1 {
            parts.push(format!("page={}", self.page.to_param()));
        }
        
        if self.limit != 10 {
            parts.push(format!("limit={}", self.limit.to_param()));
        }
        
        if let Some(sort) = &self.sort {
            parts.push(format!("sort={}", sort.to_param()));
        }
        
        for filter in &self.filters {
            parts.push(format!("filter={}", filter.to_param()));
        }
        
        parts.join("&")
    }
}
```

## 5. 解析器实现

### 3.1 路径解析器

```rust
/// 路径段解析器
pub struct PathParser {
    pattern: &'static str,
}

impl PathParser {
    pub fn new(pattern: &'static str) -> Self {
        Self { pattern }
    }

    /// 将路径分解为段
    pub fn split_segments(path: &str) -> Vec<&str> {
        path.trim_start_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .collect()
    }

    /// 匹配路径模式并提取参数
    pub fn match_pattern(&self, path_segments: &[&str]) -> Result<HashMap<String, String>, ParseError> {
        let pattern_segments = Self::split_segments(self.pattern);

        if path_segments.len() != pattern_segments.len() {
            return Err(ParseError::InvalidPath("Segment count mismatch".to_string()));
        }

        let mut params = HashMap::new();

        for (pattern_seg, path_seg) in pattern_segments.iter().zip(path_segments.iter()) {
            if pattern_seg.starts_with(':') {
                // 参数段
                let param_name = &pattern_seg[1..];
                params.insert(param_name.to_string(), path_seg.to_string());
            } else if pattern_seg != path_seg {
                // 字面量段不匹配
                return Err(ParseError::InvalidPath(format!(
                    "Expected '{}', found '{}'", pattern_seg, path_seg
                )));
            }
        }

        Ok(params)
    }
}
```

### 3.2 查询参数解析器

```rust
/// 查询参数解析器
pub struct QueryParser;

impl QueryParser {
    /// 解析查询字符串为参数映射
    pub fn parse_query_string(query: &str) -> Result<HashMap<String, Vec<String>>, ParseError> {
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
                (url_decode(pair)?, String::new())
            };

            params.entry(key).or_insert_with(Vec::new).push(value);
        }

        Ok(params)
    }
}
```

### 3.3 类型转换器

```rust
/// 为基本类型实现 FromParam
macro_rules! impl_from_param {
    ($($t:ty),*) => {
        $(
            impl FromParam for $t {
                fn from_param(param: &str) -> Result<Self, ParseError> {
                    param.parse().map_err(|_| {
                        ParseError::TypeConversion(format!(
                            "Cannot convert '{}' to {}", param, stringify!($t)
                        ))
                    })
                }
            }

            impl ToParam for $t {
                fn to_param(&self) -> String {
                    self.to_string()
                }
            }
        )*
    };
}

impl_from_param!(u8, u16, u32, u64, i8, i16, i32, i64, f32, f64, bool);

/// String 的特殊实现
impl FromParam for String {
    fn from_param(param: &str) -> Result<Self, ParseError> {
        Ok(param.to_string())
    }
}

impl ToParam for String {
    fn to_param(&self) -> String {
        self.clone()
    }
}

/// Option 的实现
impl<T: FromParam> FromParam for Option<T> {
    fn from_param(param: &str) -> Result<Self, ParseError> {
        if param.is_empty() {
            Ok(None)
        } else {
            T::from_param(param).map(Some)
        }
    }
}
```

## 6. 错误处理

```rust
/// 解析错误类型
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    /// 无效的路径格式
    InvalidPath(String),
    /// 缺少必需的参数
    MissingParameter(String),
    /// 类型转换失败
    TypeConversion(String),
    /// 无效的查询参数
    InvalidQuery(String),
    /// URL 编码/解码错误
    UrlEncoding(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InvalidPath(msg) => write!(f, "Invalid path: {}", msg),
            ParseError::MissingParameter(param) => write!(f, "Missing parameter: {}", param),
            ParseError::TypeConversion(msg) => write!(f, "Type conversion error: {}", msg),
            ParseError::InvalidQuery(msg) => write!(f, "Invalid query: {}", msg),
            ParseError::UrlEncoding(msg) => write!(f, "URL encoding error: {}", msg),
        }
    }
}

impl std::error::Error for ParseError {}
```

## 7. 工具函数

```rust
/// URL 编码/解码工具
pub fn url_encode(input: &str) -> String {
    // 简单的 URL 编码实现
    input.chars()
        .map(|c| match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
            ' ' => "+".to_string(),
            _ => format!("%{:02X}", c as u8),
        })
        .collect()
}

pub fn url_decode(input: &str) -> Result<String, ParseError> {
    let mut result = String::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '%' => {
                let hex: String = chars.by_ref().take(2).collect();
                if hex.len() != 2 {
                    return Err(ParseError::UrlEncoding("Invalid percent encoding".to_string()));
                }
                let byte = u8::from_str_radix(&hex, 16)
                    .map_err(|_| ParseError::UrlEncoding("Invalid hex in percent encoding".to_string()))?;
                result.push(byte as char);
            }
            '+' => result.push(' '),
            _ => result.push(c),
        }
    }

    Ok(result)
}

/// 分离路径和查询参数
pub fn split_path_query(url: &str) -> (&str, Option<&str>) {
    if let Some(question_pos) = url.find('?') {
        let path = &url[..question_pos];
        let query = &url[question_pos + 1..];
        (path, Some(query))
    } else {
        (url, None)
    }
}
```

## 8. 性能优化策略

### 6.1 编译时优化

- 使用 `const` 函数进行编译时计算
- 预编译路径模式为状态机
- 静态字符串池避免运行时分配

### 6.2 运行时优化

- 零拷贝字符串解析
- 内存池复用临时对象
- SIMD 加速字符串操作

### 6.3 内存优化

- 使用 `Cow<str>` 减少不必要的字符串复制
- 栈分配小对象
- 延迟初始化可选字段

## 9. 测试策略

### 7.1 单元测试

- 每个模块的独立测试
- 边界条件测试
- 错误路径测试

### 7.2 集成测试

- 端到端路由解析测试
- 复杂嵌套结构测试
- 性能基准测试

### 7.3 属性测试

- 使用 `proptest` 进行随机输入测试
- 往返一致性测试 (parse -> format -> parse)
- 模糊测试恶意输入

## 10. 递归嵌套路由设计

### 8.1 问题分析

当前的路由系统虽然支持嵌套路由，但在代数结构上存在一个重要问题：

- 路由需要选择（从多个可能的路由中选择一个匹配的）
- 子路由也需要选择（从多个可能的子路由中选择一个匹配的）

这意味着我们需要一个专门的枚举类型来表示"路由选择"的概念，而不仅仅是单个路由的表示。

### 8.2 设计目标

实现 `RouterMatch > Router > RouterMatch > Router` 的嵌套结构：

```
RouterMatch (选择哪个顶级路由)
  └── Router (具体的路由实例)
      └── RouterMatch (选择哪个子路由)
          └── Router (具体的子路由实例)
              └── ...
```

### 8.3 核心概念

#### RouterMatch 枚举

`RouterMatch` 是一个枚举类型，表示从多个可能的路由中选择一个：

```rust
#[derive(Debug, Clone, PartialEq)]
enum AppRouterMatch {
    User(UserRouter),
    Blog(BlogRouter),
    Api(ApiRouter),
}
```

#### 嵌套结构示例

```rust
// 顶级路由选择
enum AppRouterMatch {
    User(UserRouter),
    Blog(BlogRouter),
    Api(ApiRouter),
}

// 用户路由可能有子路由
enum UserSubRouterMatch {
    Profile(UserProfileRouter),
    Posts(UserPostsRouter),
    Settings(UserSettingsRouter),
}

// API 路由可能有版本化的子路由
enum ApiSubRouterMatch {
    V1(ApiV1Router),
    V2(ApiV2Router),
}
```

### 8.4 实现策略

#### RouterMatch Trait

定义一个新的 trait 来处理路由选择：

```rust
pub trait RouteMatcher: Sized {
    /// 尝试从路径解析出匹配的路由
    fn try_parse(path: &str) -> Result<Self, ParseError>;

    /// 格式化为路径字符串
    fn format(&self) -> String;

    /// 获取所有可能的路由模式
    fn patterns() -> Vec<&'static str>;

    /// 尝试解析并返回剩余路径
    fn try_parse_with_remaining(path: &str) -> Result<(Self, &str), ParseError>;

    /// 检查是否为独立路由（不依赖父路由）
    fn is_independent() -> bool;

    /// 获取根前缀
    fn root_prefix() -> &'static str;
}
```

#### 派生宏支持

扩展派生宏以支持生成 RouterMatch 枚举：

```rust
#[derive(RouterMatch)]
enum AppRouterMatch {
    #[route_prefix = "/users"]
    User(UserRouter),

    #[route_prefix = "/blog"]
    Blog(BlogRouter),

    #[route_prefix = "/api"]
    Api(ApiRouter),
}
```

#### 嵌套路由支持

Router trait 需要支持可选的子路由：

```rust
pub trait Router: Sized {
    type SubRouterMatch: RouteMatcher = NoSubRouter;

    // 现有方法...
    fn parse(path: &str) -> Result<Self, ParseError>;
    fn format(&self) -> String;
    fn pattern() -> &'static str;

    // 新增方法
    fn parse_with_sub(path: &str) -> Result<(Self, Option<Self::SubRouterMatch>), ParseError>;
    fn format_with_sub(&self, sub: Option<&Self::SubRouterMatch>) -> String;
    fn consumed_length(&self) -> usize;
    fn parse_recursive(path: &str) -> Result<(Self, Option<Self::SubRouterMatch>), ParseError>;
}
```

### 8.5 使用示例

```rust
// 解析嵌套路由
let url = "/users/profile/123";
let app_match = AppRouterMatch::try_parse(url)?;

match app_match {
    AppRouterMatch::User(user_router) => {
        // user_router 包含用户模块信息
        let (_, sub_match) = user_router.parse_recursive(url)?;
        if let Some(sub) = sub_match {
            match sub {
                UserSubRouterMatch::Profile(profile_router) => {
                    // 处理用户资料路由
                }
                // ...
            }
        }
    }
    // ...
}
```

### 8.6 优势

1. **类型安全**：编译时确保路由结构的正确性
2. **可组合性**：支持任意深度的嵌套
3. **清晰的语义**：RouterMatch 明确表示"选择"，Router 表示"具体路由"
4. **向后兼容**：现有的 Router 实现可以继续工作
5. **性能优化**：可以在编译时生成高效的匹配代码
6. **独立性**：每个层级的路由都可以独立解析和格式化

## 11. 扩展性设计

### 9.1 自定义类型支持

```rust
// 用户可以为自定义类型实现 FromParam 和 ToParam
struct UserId(u32);

impl FromParam for UserId {
    fn from_param(param: &str) -> Result<Self, ParseError> {
        u32::from_param(param).map(UserId)
    }
}

impl ToParam for UserId {
    fn to_param(&self) -> String {
        self.0.to_string()
    }
}
```

### 9.2 中间件支持

```rust
// 路由中间件 trait
pub trait RouteMiddleware {
    fn before_parse(&self, path: &str) -> Result<String, ParseError>;
    fn after_parse<T: Router>(&self, route: &T) -> Result<(), ParseError>;
}

// 在路由解析中应用中间件
#[derive(Router)]
#[middleware(AuthMiddleware, LoggingMiddleware)]
struct ProtectedRoute {
    // ...
}
```

### 9.3 插件系统

```rust
// 路由插件 trait
pub trait RoutePlugin {
    fn name(&self) -> &'static str;
    fn process(&self, context: &mut RouteContext) -> Result<(), PluginError>;
}

// 插件注册
RouterBuilder::new()
    .plugin(CachePlugin::new())
    .plugin(MetricsPlugin::new())
    .build()
```

## 12. 总结：面向数据编程的价值

### 12.1 核心优势

**Ruled Router** 通过面向数据编程范式，实现了以下核心价值：

1. **数据即代码**：路由定义就是数据结构，代码逻辑由宏自动生成
2. **零重复**：自动前缀提取彻底消除了路径信息的重复定义
3. **类型安全**：编译时保证路由参数类型正确性
4. **组合性**：通过数据结构的组合实现复杂的嵌套路由
5. **可维护性**：路由变更只需修改数据结构，逻辑自动更新

### 12.2 设计哲学

```rust
// 传统命令式编程：手写解析逻辑
fn parse_user_route(path: &str) -> Result<UserRoute, Error> {
    let segments: Vec<&str> = path.split('/').collect();
    if segments.len() != 3 || segments[1] != "users" {
        return Err(Error::InvalidPath);
    }
    let id = segments[2].parse::<u32>()?;
    Ok(UserRoute { id })
}

// 面向数据编程：声明式数据结构
#[derive(Router)]
#[router(pattern = "/users/:id")]
struct UserRoute {
    id: u32,
}
// 解析逻辑自动生成，零错误，零维护成本
```

### 12.3 生态系统效应

通过面向数据编程，**Ruled Router** 创建了一个自洽的生态系统：

- **数据结构** → 定义路由形状
- **派生宏** → 生成解析/格式化逻辑
- **Trait 系统** → 提供统一接口
- **自动前缀提取** → 消除重复定义
- **递归嵌套** → 支持任意复杂度

这种设计让开发者专注于**业务逻辑的数据建模**，而非底层的**字符串解析实现**，真正实现了"数据驱动代码"的编程范式。
