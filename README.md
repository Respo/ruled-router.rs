# Ruled Router

一个基于面向数据编程的 Rust 路由解析库，通过自动前缀提取实现零重复的类型安全路由系统。

A data-oriented Rust routing library that implements zero-duplication type-safe routing through automatic prefix extraction.

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

**Ruled Router** 采用面向数据编程范式，让你通过定义数据结构来声明路由，所有解析和格式化逻辑由宏自动生成。核心特性：

- 🎯 **面向数据编程**：路由定义即数据结构，逻辑自动生成
- 🔄 **自动前缀提取**：RouterMatch 自动从 RouterData 类型提取路由前缀，实现 DRY 原则
- 🚀 **零运行时开销**：所有解析逻辑在编译时生成
- 🔒 **类型安全**：路由参数和查询参数都有严格的类型检查
- 🌳 **递归嵌套路由**：支持任意深度的路由嵌套，每层可有独立的子路由
- 📝 **声明式语法**：通过结构体和枚举定义路由，无需手写解析代码
- ⚡ **高性能**：编译时优化，运行时零分配
- 🔧 **零维护成本**：路由变更只需修改数据结构，逻辑自动更新

## 项目结构

这是一个 Cargo workspace 项目，包含以下 crate：

- `ruled-router` - 主库，包含核心 trait 和实现
- `ruled-router-derive` - 过程宏库，提供 `#[derive(RouterData)]` 和 `#[derive(Query)]` 宏

### 开发说明

> **注意**：本库的大部分代码由 Claude Sonnet AI 生成，如果考虑使用, 请先贡献测试用例。

## 快速开始

在您的 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
ruled-router = "0.1.0"
```

## 重要概念说明

### RouterData vs RouterMatch

- **RouterData**: 用于定义单个路由类型，不能直接用作顶层路由
- **RouterMatch**: 用于顶层路由，通过 enum 组合多个 RouterData

```rust
// ❌ 错误：RouterData 不能直接用作顶层路由
#[derive(RouterData)]
#[router(pattern = "/users/:id")]
struct UserRoute { id: u32 }

// ✅ 正确：使用 RouterMatch enum 作为顶层路由
#[derive(RouterMatch)]
enum AppRouter {
    User(UserRoute),  // 自动提取 "/users" 前缀
    // ... 其他路由
}
```

## 基本用法

### 单层路由

查看完整示例：[examples/basic_usage.rs](ruled-router/examples/basic_usage.rs)

```rust
use ruled_router::prelude::*;

// Define route structure
#[derive(RouterData, Debug)]
#[router(pattern = "/users/:id")]
struct UserRoute {
    id: u32,
    #[query]
    query: UserQuery,
}

// Define query parameters
#[derive(Query, Debug)]
struct UserQuery {
    #[query(name = "tab")]
    tab: Option<String>,
    #[query(name = "page", default = "1")]
    page: u32,
}

fn main() {
    // Parse route
    let path = "/users/123?tab=profile&page=2";
    let route = UserRoute::parse(path).unwrap();

    println!("用户ID: {}", route.id);
    println!("标签页: {:?}", route.query.tab);
    println!("页码: {}", route.query.page);

    // Format route
    let formatted = route.format();
    println!("格式化结果: {}", formatted);
    // Output: /users/123?tab=profile&page=2
}
```

### 自动前缀提取：路由匹配器

查看完整示例：[examples/auto_prefix_extraction.rs](ruled-router/examples/auto_prefix_extraction.rs)

```rust
use ruled_router::prelude::*;

// Route matcher - automatic prefix extraction
#[derive(RouterMatch, Debug)]
enum AppRouterMatch {
    User(UserRoute),    // Auto-extracted prefix: "/users"
    Blog(BlogRoute),    // Auto-extracted prefix: "/blog"
    Api(ApiRoute),      // Auto-extracted prefix: "/api"
}

#[derive(RouterData, Debug)]
#[router(pattern = "/users/:id")]
struct UserRoute { id: u32 }

#[derive(RouterData, Debug)]
#[router(pattern = "/blog/:slug")]
struct BlogRoute { slug: String }

#[derive(RouterData, Debug)]
#[router(pattern = "/api/v1")]
struct ApiRoute;

fn main() {
    let paths = ["/users/123", "/blog/hello-world", "/api/v1"];

    for path in paths {
        match AppRouterMatch::try_parse(path) {
            Ok(route) => println!("匹配成功: {} -> {:?}", path, route),
            Err(e) => println!("匹配失败: {} -> {:?}", path, e),
        }
    }
}
```

### 递归嵌套路由

支持任意深度的嵌套路由，实现复杂的应用架构。查看完整示例：[examples/nested_routing.rs](ruled-router/examples/nested_routing.rs)

```rust
use ruled_router::prelude::*;
use ruled_router::error::RouteState;

// 顶层路由匹配器 - 自动前缀提取
#[derive(RouterMatch, Debug)]
enum AppRouterMatch {
    User(ModuleRoute),    // 自动提取前缀: "/user"
    Shop(ModuleRoute),    // 自动提取前缀: "/shop"
    Admin(ModuleRoute),   // 自动提取前缀: "/admin"
}

// 模块路由 - 通用的模块入口
#[derive(RouterData, Debug)]
#[router(pattern = "/:module")]  // 动态模块名
struct ModuleRoute {
    module: String,
    #[sub_router]
    sub_router: RouteState<SubRouterMatch>,
}

// 子路由匹配器 - 自动前缀提取
#[derive(RouterMatch, Debug)]
enum SubRouterMatch {
    Category(CategoryRoute),     // 分类路由
}

// 分类路由 - 第二层嵌套
#[derive(RouterData, Debug)]
#[router(pattern = "/category/:category_id")]
struct CategoryRoute {
    category_id: u32,
    #[query]
    query: SimpleQuery,
    #[sub_router]
    sub_router: RouteState<DetailRouterMatch>,
}

// 详情路由匹配器 - 第三层嵌套
#[derive(RouterMatch, Debug)]
enum DetailRouterMatch {
    Settings(UserSettingsRoute),  // 用户设置
    Product(ProductDetailRoute),  // 产品详情
}

// 具体的详情路由实现
#[derive(RouterData, Debug)]
#[router(pattern = "/settings/:setting_id")]
struct UserSettingsRoute {
    setting_id: u32,
    #[query]
    query: SimpleQuery,
}

#[derive(RouterData, Debug)]
#[router(pattern = "/product/:product_id")]
struct ProductDetailRoute {
    product_id: u32,
    #[query]
    query: SimpleQuery,
}

// 简单查询参数定义
#[derive(Query, Debug)]
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
                            SubRouterMatch::Category(category_route) => {
                                println!("分类ID: {}", category_route.category_id);

                                match category_route.sub_router {
                                    RouteState::SubRoute(detail) => {
                                        match detail {
                                            DetailRouterMatch::Settings(settings) => {
                                                println!("设置ID: {}", settings.setting_id);
                                                println!("格式: {:?}", settings.query.format);
                                                println!("页码: {}", settings.query.page);
                                            }
                                            _ => {}
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    // 格式化三层嵌套路由
    let route = AppRouterMatch::User(
        ModuleRoute {
            module: "user".to_string(),
            sub_router: RouteState::SubRoute(
                SubRouterMatch::Category(
                    CategoryRoute {
                        category_id: 123,
                        query: SimpleQuery {
                            format: Some("json".to_string()),
                            page: 2,
                        },
                        sub_router: RouteState::SubRoute(
                            DetailRouterMatch::Settings(
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
}
```

## 功能特性

### 🎯 面向数据编程

- **数据即代码**：通过数据结构定义路由，逻辑自动生成
- **零重复定义**：自动前缀提取，路径信息只需定义一次
- **组合式设计**：通过结构体和枚举组合实现复杂路由
- **声明式语法**：无需手写解析代码，专注业务逻辑

### 🔄 自动前缀提取

- **DRY 原则**：RouterMatch 自动从 RouterData 类型提取路由前缀
- **零维护成本**：路径变更只需修改一处，逻辑自动更新
- **类型安全**：编译时验证路由前缀的一致性

### 🌳 递归嵌套路由

- **无限深度**：支持任意层级的路由嵌套
- **独立子路由**：每层可有独立的 `#[sub_router]` 字段
- **参数传递**：父路由参数自动传递给子路由
- **模块化设计**：每个路由层级可独立开发和测试

### 🔒 类型安全解析

- **编译时检查**：路径参数和查询参数类型在编译时验证
- **自动类型转换**：支持 `u32`、`String`、`bool` 等常见类型
- **自定义类型**：通过 `FromParam` 和 `ToParam` trait 支持自定义类型
- **错误处理**：详细的解析错误信息

### 📝 查询参数处理

- **多种数据类型**：字符串、数字、布尔值、枚举等
- **数组参数**：支持 `?tags=rust&tags=web` 形式的多值参数
- **可选参数**：`Option<T>` 类型支持可选查询参数
- **默认值**：`#[query(default = "value")]` 属性设置默认值
- **自定义参数名**：`#[query(name = "custom_name")]` 映射参数名

### ⚡ 高性能设计

- **零运行时开销**：所有解析逻辑在编译时生成
- **零分配解析**：避免不必要的内存分配
- **编译时优化**：编译器可进行深度优化
- **缓存友好**：生成的代码对 CPU 缓存友好

## 属性说明

- `#[router(pattern = "...")]`：定义路由模式，支持参数占位符 `:param`
- `#[query]`：标记查询参数字段
- `#[sub_router]`：标记子路由字段，支持嵌套路由
- `#[query(name = "...")]`：自定义查询参数名
- `#[query(default = "...")]`：设置查询参数默认值

## 示例项目

查看 `examples/` 目录中的完整示例：

- [`basic_usage.rs`](ruled-router/examples/basic_usage.rs) - 基本路由解析和格式化
- [`auto_prefix_extraction.rs`](ruled-router/examples/auto_prefix_extraction.rs) - 自动前缀提取示例
- [`nested_routing.rs`](ruled-router/examples/nested_routing.rs) - 嵌套路由示例
- [`query_params.rs`](ruled-router/examples/query_params.rs) - 查询参数处理

运行示例：

```bash
# Run basic usage example
cargo run --example basic_usage

# Run auto prefix extraction example
cargo run --example auto_prefix_extraction

# Run nested routing example
cargo run --example nested_routing
```

## 测试

运行测试套件：

```bash
# Run all tests
cargo test

# Run specific tests
cargo test router_derive_tests
cargo test query_derive_tests
```

## 贡献指南

我们欢迎各种形式的贡献！

### 如何贡献

1. **Fork** 本仓库
2. 创建你的特性分支 (`git checkout -b feature/amazing-feature`)
3. 提交你的更改 (`git commit -m 'Add some amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 打开一个 **Pull Request**

### 开发环境

```bash
# Clone repository
git clone https://github.com/your-username/ruled-router.rs.git
cd ruled-router.rs

# Install dependencies
cargo build

# Run tests
cargo test

# Check code format
cargo fmt --check

# Run clippy
cargo clippy -- -D warnings
```

## 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 致谢

- 感谢 [argh](https://github.com/google/argh) 项目的设计灵感
- 感谢 Rust 社区的宏系统设计
- 感谢所有贡献者的努力
