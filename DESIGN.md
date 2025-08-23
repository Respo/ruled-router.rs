# Ruled Router 技术设计文档

## 1. 架构概览

### 1.1 模块结构

```
ruled-router/
├── src/
│   ├── lib.rs              # 库入口，导出公共 API
│   ├── macros/
│   │   ├── mod.rs           # 宏模块入口
│   │   ├── router.rs        # Router 派生宏实现
│   │   ├── query.rs         # Query 派生宏实现
│   │   └── utils.rs         # 宏工具函数
│   ├── parser/
│   │   ├── mod.rs           # 解析器模块入口
│   │   ├── path.rs          # 路径解析器
│   │   ├── query.rs         # 查询参数解析器
│   │   └── types.rs         # 类型转换器
│   ├── formatter/
│   │   ├── mod.rs           # 格式化器模块入口
│   │   ├── path.rs          # 路径格式化器
│   │   └── query.rs         # 查询参数格式化器
│   ├── error.rs             # 错误类型定义
│   └── traits.rs            # 核心 trait 定义
└── tests/
    ├── integration.rs       # 集成测试
    └── examples/            # 示例测试
```

### 1.2 核心 Trait 设计

```rust
/// 路由解析和格式化的核心 trait
pub trait Router: Sized {
    /// 从路径字符串解析路由
    fn parse(path: &str) -> Result<Self, ParseError>;

    /// 将路由格式化为路径字符串
    fn format(&self) -> String;

    /// 获取路由模式（用于调试和文档生成）
    fn pattern() -> &'static str;
}

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

## 2. 宏系统设计

### 2.1 Router 派生宏

#### 输入结构体示例

```rust
#[derive(Router)]
struct UserProfile {
    #[router("/user/:id/profile/:section")]
    id: u32,
    section: String,
    #[query]
    options: ProfileOptions,
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
        let route_pattern = "/user/:id/profile/:section";
        let params = match_pattern(route_pattern, &path_segments)?;

        // 3. 提取并转换参数
        let id = u32::from_param(params.get("id").ok_or(ParseError::MissingParameter("id"))?)?;
        let section = String::from_param(params.get("section").ok_or(ParseError::MissingParameter("section"))?)?;

        // 4. 解析查询参数
        let options = ProfileOptions::parse(query_part.unwrap_or(""))?;

        Ok(UserProfile { id, section, options })
    }

    fn format(&self) -> String {
        let mut path = String::from("/user/");
        path.push_str(&self.id.to_param());
        path.push_str("/profile/");
        path.push_str(&self.section.to_param());

        let query = self.options.format();
        if !query.is_empty() {
            path.push('?');
            path.push_str(&query);
        }

        path
    }

    fn pattern() -> &'static str {
        "/user/:id/profile/:section"
    }
}
```

### 2.1.1 嵌套路由示例

以下是一个完整的嵌套路由示例，展示了多层嵌套、参数传递、查询字符串等功能：

```rust
use ruled_router::prelude::*;

// 第一层：应用主路由
#[derive(Router)]
#[route_prefix("/api/v1")]
enum AppRoute {
    #[route("/users")]
    Users(UserRouterMatch),
    #[route("/shop")]
    Shop(ShopRouterMatch),
    #[route("/admin")]
    Admin(AdminRouterMatch),
}

// 第二层：用户相关路由
#[derive(RouterMatch)]
#[route_prefix("/users")]
enum UserRouterMatch {
    #[route("/")]
    List(UserListRoute),
    #[route("/:id")]
    Detail(UserDetailRouterMatch),
    #[route("/search")]
    Search(UserSearchRoute),
}

// 第三层：用户详情子路由
#[derive(RouterMatch)]
#[route_prefix("/:id")]
enum UserDetailRouterMatch {
    #[route("/")]
    Profile(UserProfileRoute),
    #[route("/posts")]
    Posts(UserPostsRouterMatch),
    #[route("/settings")]
    Settings(UserSettingsRoute),
}

// 第四层：用户文章子路由
#[derive(RouterMatch)]
#[route_prefix("/posts")]
enum UserPostsRouterMatch {
    #[route("/")]
    List(PostListRoute),
    #[route("/:post_id")]
    Detail(PostDetailRoute),
    #[route("/create")]
    Create(PostCreateRoute),
}

// 具体的路由实现
#[derive(Router)]
#[route("/")]
struct UserListRoute {
    #[query]
    params: UserListQuery,
}

#[derive(Query)]
struct UserListQuery {
    #[query(name = "page", default = "1")]
    page: u32,
    #[query(name = "limit", default = "20")]
    limit: u32,
    #[query(name = "sort")]
    sort: Option<String>,
    #[query(name = "filter", multiple)]
    filters: Vec<String>,
}

#[derive(Router)]
#[route("/:id")]
struct UserProfileRoute {
    id: u32,
    #[query]
    params: ProfileQuery,
}

#[derive(Query)]
struct ProfileQuery {
    #[query(name = "include", multiple)]
    include: Vec<String>,
    #[query(name = "format")]
    format: Option<String>,
}

#[derive(Router)]
#[route("/:post_id")]
struct PostDetailRoute {
    id: u32,        // 来自父路由的用户ID
    post_id: u32,   // 当前路由的文章ID
    #[query]
    params: PostDetailQuery,
}

#[derive(Query)]
struct PostDetailQuery {
    #[query(name = "comments", default = "true")]
    include_comments: bool,
    #[query(name = "version")]
    version: Option<u32>,
}

// 使用示例
fn main() {
    // 解析复杂的嵌套路由
    let path = "/api/v1/users/123/posts/456?comments=true&version=2";

    if let Ok(route) = AppRoute::parse(path) {
        match route {
            AppRoute::Users(UserRouterMatch::Detail(
                UserDetailRouterMatch::Posts(
                    UserPostsRouterMatch::Detail(post_route)
                )
            )) => {
                println!("用户ID: {}", post_route.id);
                println!("文章ID: {}", post_route.post_id);
                println!("包含评论: {}", post_route.params.include_comments);
                println!("版本: {:?}", post_route.params.version);
            }
            _ => println!("其他路由"),
        }
    }

    // 格式化路由
    let route = AppRoute::Users(
        UserRouterMatch::Detail(
            UserDetailRouterMatch::Posts(
                UserPostsRouterMatch::Detail(
                    PostDetailRoute {
                        id: 123,
                        post_id: 456,
                        params: PostDetailQuery {
                            include_comments: true,
                            version: Some(2),
                        },
                    }
                )
            )
        )
    );

    let formatted = route.format();
    println!("格式化结果: {}", formatted);
    // 输出: /api/v1/users/123/posts/456?comments=true&version=2

    // 获取路由模式
    let patterns = AppRoute::patterns();
    for pattern in patterns {
        println!("支持的路由模式: {}", pattern);
    }
    // 输出:
    // /api/v1/users/?page=1&limit=20&sort=&filter=
    // /api/v1/users/:id?include=&format=
    // /api/v1/users/:id/posts/?page=1&limit=20&sort=&filter=
    // /api/v1/users/:id/posts/:post_id?comments=true&version=
    // /api/v1/users/:id/posts/create
    // /api/v1/users/search
    // ...
}
```

这个示例展示了以下功能：

1. **多层嵌套结构**：`AppRoute > UserRouterMatch > UserDetailRouterMatch > UserPostsRouterMatch`
2. **路由前缀**：每层都有自己的前缀，自动组合成完整路径
3. **参数传递**：父路由的参数（如用户 ID）会传递给子路由
4. **查询字符串**：支持复杂的查询参数，包括默认值、多值、可选值
5. **类型安全**：所有参数都有明确的类型，编译时检查
6. **模式匹配**：可以精确匹配到具体的嵌套路由
7. **格式化输出**：可以将路由结构重新格式化为 URL
8. **模式获取**：可以获取所有支持的路由模式

### 2.2 Query 派生宏

#### 输入结构体示例

```rust
#[derive(Query)]
struct SearchParams {
    q: String,
    #[query(name = "page_num")]
    page: Option<u32>,
    #[query(default = "10")]
    limit: u32,
    #[query(multiple)]
    tags: Vec<String>,
}
```

#### 生成的代码结构

```rust
impl Query for SearchParams {
    fn parse(query: &str) -> Result<Self, ParseError> {
        let params = parse_query_string(query)?;

        let q = params.get("q")
            .ok_or(ParseError::MissingParameter("q"))?
            .first()
            .ok_or(ParseError::MissingParameter("q"))?
            .clone();

        let page = params.get("page_num")
            .and_then(|v| v.first())
            .map(|s| u32::from_param(s))
            .transpose()?;

        let limit = params.get("limit")
            .and_then(|v| v.first())
            .map(|s| u32::from_param(s))
            .transpose()?
            .unwrap_or(10);

        let tags = params.get("tags")
            .map(|v| v.iter().map(|s| s.clone()).collect())
            .unwrap_or_default();

        Ok(SearchParams { q, page, limit, tags })
    }

    fn format(&self) -> String {
        let mut parts = Vec::new();

        parts.push(format!("q={}", url_encode(&self.q)));

        if let Some(page) = &self.page {
            parts.push(format!("page_num={}", page.to_param()));
        }

        if self.limit != 10 {
            parts.push(format!("limit={}", self.limit.to_param()));
        }

        for tag in &self.tags {
            parts.push(format!("tags={}", url_encode(tag)));
        }

        parts.join("&")
    }
}
```

## 3. 解析器实现

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

## 4. 错误处理

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

## 5. 工具函数

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

## 6. 性能优化策略

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

## 7. 测试策略

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

## 8. 递归嵌套路由设计

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

## 9. 扩展性设计

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
// 支持解析中间件，用于验证、转换等
trait ParseMiddleware {
    fn before_parse(&self, input: &str) -> Result<String, ParseError>;
    fn after_parse<T>(&self, result: Result<T, ParseError>) -> Result<T, ParseError>;
}
```

### 9.3 插件系统

- 支持第三方扩展
- 可插拔的解析器和格式化器
- 运行时路由注册
