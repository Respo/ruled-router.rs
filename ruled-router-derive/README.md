# ruled-router-derive

这是 `ruled-router` 的派生宏包，提供了便捷的宏来自动生成路由匹配和查询参数解析代码。

## 基础用法

### RouterMatch 派生宏

为结构体自动生成路由匹配逻辑：

```rust
use ruled_router_derive::RouterMatch;

#[derive(RouterMatch)]
struct UserRoute {
    #[route(pattern = "/users/{id}")]
    user_id: u32,
}
```

### Query 派生宏

为结构体自动生成查询参数解析器：

```rust
use ruled_router_derive::Query;

#[derive(Query)]
struct SearchQuery {
    q: Option<String>,
    page: Option<u32>,
    limit: Option<u32>,
}
```

### Route 派生宏

为结构体自动生成完整的路由处理逻辑：

```rust
use ruled_router_derive::Route;

#[derive(Route)]
#[route(pattern = "/api/v1")]
struct ApiRoute {
    #[query]
    params: SearchQuery,
    #[route]
    user: UserRoute,
}
```

## 完整文档和高级用法

更多详细的使用方法、高级特性和示例，请访问主包文档：

**[https://crates.io/crates/ruled-router](https://crates.io/crates/ruled-router)**

## 许可证

本项目采用 MIT 许可证。详情请参见 [LICENSE](../LICENSE) 文件。