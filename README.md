# Ruled Router - 基于宏的路由解析库

## 🆕 新增功能 (v0.1.1)

### 🌐 DOM Feature

为 Web 浏览器环境新增了完整的路由管理功能：

**功能特性:**

- **路由监听**: 监听浏览器前进/后退按钮事件
- **路由跳转**: 使用 History API 进行 SPA 导航
- **URL 管理**: 完整的 URL 解析和格式化
- **查询参数**: 高级查询参数处理

**使用方法:**

```toml
[dependencies]
ruled-router = { version = "0.1.0", features = ["dom"] }
```

```rust
use ruled_router::prelude::*;

// 创建 DOM 路由管理器
let mut router = DomRouter::<MyRoute>::new()?;

// 添加路由变化监听器
router.add_listener(|route: &MyRoute| {
    // 处理路由变化
});

// 开始监听浏览器事件
router.start_listening()?;

// 导航到新路由
router.navigate_to(&MyRoute::Home, false)?;
```

### 📱 Web Page Example

新增了完整的 Web 示例项目 `web-page-example`：

- **完整 SPA 演示**: 多页面单页应用
- **WASM 编译**: 使用 `wasm-pack` 编译为 WebAssembly
- **交互式 UI**: 现代化的 Web 界面
- **实时路由**: 完整的路由功能展示

**快速体验:**

```bash
cd web-page-example
./build.sh
python3 -m http.server 8000
# 打开 http://localhost:8000
```

---

## 项目概述

`ruled-router` 是一个受 `argh` 启发的 Rust 路由解析库，通过宏定义自动生成路由解析器和格式化器。主要用于解析和格式化 Web 前端路由，支持复杂的嵌套路由结构和查询参数处理。

## 项目结构

这是一个 Cargo workspace 项目，包含以下 crate：

- `ruled-router` - 主库，包含核心 trait 和实现
- `ruled-router-derive` - 过程宏库，提供 `#[derive(Router)]` 和 `#[derive(Query)]` 宏

## 快速开始

在您的 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
ruled-router = "0.1.0"
```

使用派生宏定义路由：

```rust
use ruled_router::prelude::*;

#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "users/:id")]
struct UserRoute {
    id: u32,
}

#[derive(Debug, Clone, PartialEq, Query)]
struct UserQuery {
    tab: Option<String>,
    active: Option<bool>,
}

fn main() {
    // 解析路径
    let route = UserRoute::parse("/users/123").unwrap();
    println!("用户 ID: {}", route.id); // 用户 ID: 123

    // 格式化路径
    let path = route.format();
    println!("路径: {}", path); // 路径: /users/123

    // 解析查询参数
    let query = UserQuery::parse("tab=profile&active=true").unwrap();
    println!("查询: {:?}", query);
}
```

## 核心设计理念

### 1. 宏驱动的代码生成

- 参考 `argh` 的实现方式，使用过程宏自动生成解析和格式化逻辑
- 通过属性宏标注结构体字段，定义路由段的解析规则
- 编译时生成高效的解析器代码，运行时零成本抽象

### 2. 结构化路由定义

- 使用 Rust 结构体定义路由结构
- 支持嵌套结构体组合复杂路由路径
- 类型安全的路由参数处理

## 功能需求

### 核心功能

#### 1. 路由解析 (Parse)

- 将 URL 路径字符串解析为结构化的路由对象
- 支持路径参数提取和类型转换
- 支持可选路径段
- 错误处理和验证

#### 2. 路由格式化 (Format)

- 将结构化路由对象转换为 URL 路径字符串
- 支持参数插值
- 保证往返一致性 (parse -> format -> parse)

#### 3. 查询参数处理

- 解析 URL 查询字符串
- 支持多值参数
- 类型转换和验证

### 高级功能

#### 1. 嵌套路由支持

```rust
#[derive(Router)]
struct AppRouter {
    #[router("/api")]
    api: ApiRouter,
    #[router("/admin")]
    admin: AdminRouter,
}

#[derive(Router)]
struct ApiRouter {
    #[router("/users/:id")]
    user: UserRoute,
    #[router("/posts")]
    posts: PostsRoute,
}
```

#### 2. 路径参数类型

- 字符串参数: `:name`
- 数字参数: `:id` (自动转换为 u32, i32 等)
- 可选参数: `?:optional`
- 通配符: `*path` (捕获剩余路径)

#### 3. 查询参数集成

```rust
#[derive(Router)]
struct SearchRoute {
    #[router("/search/:category")]
    category: String,
    #[query]
    params: SearchParams,
}

#[derive(Query)]
struct SearchParams {
    q: String,
    page: Option<u32>,
    limit: Option<u32>,
}
```

## 技术架构

### 宏系统设计

#### 1. 主要宏

- `#[derive(Router)]`: 为结构体生成路由解析器
- `#[derive(Query)]`: 为结构体生成查询参数解析器
- `#[router("path")]`: 定义路由路径模式
- `#[query]`: 标记查询参数字段

#### 2. 生成的 Trait

```rust
trait Router: Sized {
    fn parse(path: &str) -> Result<Self, ParseError>;
    fn format(&self) -> String;
}

trait Query: Sized {
    fn parse(query: &str) -> Result<Self, ParseError>;
    fn format(&self) -> String;
}
```

### 解析器架构

#### 1. 路径分段器

- 将 URL 路径分解为段
- 处理 URL 编码/解码
- 参数提取和验证

#### 2. 类型转换器

- 字符串到各种类型的转换
- 自定义类型转换支持
- 错误处理和回退机制

#### 3. 路由匹配器

- 模式匹配算法
- 优先级处理
- 冲突检测

## 使用示例

### 基本用法

```rust
use ruled_router::{Router, Query};

#[derive(Router, Debug, PartialEq)]
struct UserProfile {
    #[router("/user/:id/profile")]
    id: u32,
    #[query]
    options: ProfileOptions,
}

#[derive(Query, Debug, PartialEq)]
struct ProfileOptions {
    tab: Option<String>,
    edit: Option<bool>,
}

fn main() {
    // 解析路由
    let route = UserProfile::parse("/user/123/profile?tab=settings&edit=true").unwrap();
    assert_eq!(route.id, 123);
    assert_eq!(route.options.tab, Some("settings".to_string()));
    assert_eq!(route.options.edit, Some(true));

    // 格式化路由
    let url = route.format();
    assert_eq!(url, "/user/123/profile?tab=settings&edit=true");
}
```

### 嵌套路由

```rust
#[derive(Router)]
struct AppRouter {
    #[router("/")]
    home: HomeRoute,
    #[router("/api/v1")]
    api: ApiRouter,
}

#[derive(Router)]
struct ApiRouter {
    #[router("/users/:id")]
    user: UserRoute,
    #[router("/posts/:slug")]
    post: PostRoute,
}
```

## 错误处理

```rust
#[derive(Debug)]
enum ParseError {
    InvalidPath(String),
    MissingParameter(String),
    TypeConversion(String),
    InvalidQuery(String),
}
```

## 性能目标

- 编译时代码生成，运行时零分配
- 高效的字符串解析算法
- 最小化内存占用
- 支持 `no_std` 环境

## 开发计划

### Phase 1: 核心功能

- [ ] 基础宏系统实现
- [ ] 简单路径解析
- [ ] 基本类型转换
- [ ] 错误处理框架

### Phase 2: 高级功能

- [ ] 嵌套路由支持
- [ ] 查询参数处理
- [ ] 自定义类型转换
- [ ] 性能优化

### Phase 3: 生态集成

- [ ] 文档和示例
- [ ] 测试覆盖
- [ ] 基准测试
- [ ] 社区反馈集成

## 依赖项

- `proc-macro2`: 宏实现
- `quote`: 代码生成
- `syn`: AST 解析
- `serde` (可选): 序列化支持

## 许可证

MIT License
