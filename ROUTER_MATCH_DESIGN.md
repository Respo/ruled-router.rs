# RouterMatch 嵌套路由设计

## 问题分析

当前的路由系统虽然支持嵌套路由，但在代数结构上存在一个重要问题：

- 路由需要选择（从多个可能的路由中选择一个匹配的）
- 子路由也需要选择（从多个可能的子路由中选择一个匹配的）

这意味着我们需要一个专门的枚举类型来表示"路由选择"的概念，而不仅仅是单个路由的表示。

## 设计目标

实现 `RouterMatch > Router > RouterMatch > Router` 的嵌套结构：

```
RouterMatch (选择哪个顶级路由)
  └── Router (具体的路由实例)
      └── RouterMatch (选择哪个子路由)
          └── Router (具体的子路由实例)
              └── ...
```

## 核心概念

### RouterMatch 枚举

`RouterMatch` 是一个枚举类型，表示从多个可能的路由中选择一个：

```rust
#[derive(Debug, Clone, PartialEq)]
enum AppRouterMatch {
    User(UserRouter),
    Blog(BlogRouter),
    Api(ApiRouter),
}
```

### 嵌套结构示例

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

## 实现策略

### 1. RouterMatch Trait

定义一个新的 trait 来处理路由选择：

```rust
pub trait RouterMatch: Sized {
    /// 尝试从路径解析出匹配的路由
    fn try_parse(path: &str) -> Result<Self, ParseError>;
    
    /// 格式化为路径字符串
    fn format(&self) -> String;
    
    /// 获取所有可能的路由模式
    fn patterns() -> Vec<&'static str>;
}
```

### 2. 派生宏支持

扩展派生宏以支持生成 RouterMatch 枚举：

```rust
#[derive(RouterMatch)]
enum AppRouterMatch {
    #[route(UserRouter)]
    User(UserRouter),
    
    #[route(BlogRouter)]
    Blog(BlogRouter),
    
    #[route(ApiRouter)]
    Api(ApiRouter),
}
```

### 3. 嵌套路由支持

 Router trait 需要支持可选的子路由：

```rust
pub trait Router: Sized {
    type SubRouter: RouterMatch = ();
    
    // 现有方法...
    fn parse(path: &str) -> Result<Self, ParseError>;
    fn format(&self) -> String;
    fn pattern() -> &'static str;
    
    // 新增方法
    fn parse_with_sub(path: &str) -> Result<(Self, Option<Self::SubRouter>), ParseError>;
    fn format_with_sub(&self, sub: Option<&Self::SubRouter>) -> String;
}
```

## 使用示例

```rust
// 解析嵌套路由
let url = "/api/v1/users/123/posts/456";
let app_match = AppRouterMatch::try_parse(url)?;

match app_match {
    AppRouterMatch::Api(api_router) => {
        // api_router 包含版本信息
        let sub_match = api_router.parse_sub_route(remaining_path)?;
        match sub_match {
            ApiSubRouterMatch::V1(v1_router) => {
                // 处理 v1 API 路由
            }
            // ...
        }
    }
    // ...
}
```

## 优势

1. **类型安全**：编译时确保路由结构的正确性
2. **可组合性**：支持任意深度的嵌套
3. **清晰的语义**：RouterMatch 明确表示"选择"，Router 表示"具体路由"
4. **向后兼容**：现有的 Router 实现可以继续工作
5. **性能优化**：可以在编译时生成高效的匹配代码

## 实现计划

1. 在 traits.rs 中定义 RouterMatch trait
2. 修改 Router trait 以支持子路由
3. 实现 RouterMatch 派生宏
4. 更新示例代码展示新的嵌套结构
5. 添加相应的测试用例