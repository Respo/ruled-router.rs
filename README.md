# Ruled Router

一个基于面向数据编程的 Rust 路由解析库，通过自动前缀提取实现零重复的类型安全路由系统。

## 项目概述

**Ruled Router** 采用面向数据编程范式，让你通过定义数据结构来声明路由，所有解析和格式化逻辑由宏自动生成。核心特性：

- 🎯 **面向数据编程**：路由定义即数据结构，逻辑自动生成
- 🔄 **自动前缀提取**：RouterMatch 自动从 Router 类型提取路由前缀，实现 DRY 原则
- 🚀 **零运行时开销**：所有解析逻辑在编译时生成
- 🔒 **类型安全**：路由参数和查询参数都有严格的类型检查
- 🌳 **递归嵌套路由**：支持任意深度的路由嵌套，每层可有独立的子路由
- 📝 **声明式语法**：通过结构体和枚举定义路由，无需手写解析代码
- ⚡ **高性能**：编译时优化，运行时零分配
- 🔧 **零维护成本**：路由变更只需修改数据结构，逻辑自动更新

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

### 基本用法：单层路由

```rust
use ruled_router::prelude::*;

// 定义路由结构体
#[derive(Router)]
#[router(pattern = "/users/:id")]  // 只需定义一次路径模式
struct UserRoute {
    id: u32,
    #[query]
    query: UserQuery,
}

// 定义查询参数
#[derive(Query)]
struct UserQuery {
    #[query(name = "tab")]
    tab: Option<String>,
    #[query(name = "page", default = "1")]
    page: u32,
}

fn main() {
    // 解析路由
    let path = "/users/123?tab=profile&page=2";
    let route = UserRoute::parse(path).unwrap();
    
    println!("用户ID: {}", route.id);
    println!("标签页: {:?}", route.query.tab);
    println!("页码: {}", route.query.page);
    
    // 格式化路由
    let formatted = route.format();
    println!("格式化结果: {}", formatted);
    // 输出: /users/123?tab=profile&page=2
}
```

### 自动前缀提取：路由匹配器

```rust
use ruled_router::prelude::*;

// 路由匹配器 - 自动前缀提取，无需重复定义路径
#[derive(RouterMatch)]
enum AppRouterMatch {
    User(UserRoute),    // 自动提取前缀: "/users"
    Blog(BlogRoute),    // 自动提取前缀: "/blog"
    Api(ApiRoute),      // 自动提取前缀: "/api"
}

#[derive(Router)]
#[router(pattern = "/users/:id")]
struct UserRoute { id: u32 }

#[derive(Router)]
#[router(pattern = "/blog/:slug")]
struct BlogRoute { slug: String }

#[derive(Router)]
#[router(pattern = "/api/v1")]
struct ApiRoute;

fn main() {
    // 自动路由匹配
    let paths = [
        "/users/123",
        "/blog/hello-world", 
        "/api/v1"
    ];
    
    for path in paths {
        match AppRouterMatch::try_parse(path) {
            Ok(route) => println!("匹配成功: {} -> {:?}", path, route),
            Err(e) => println!("匹配失败: {} -> {:?}", path, e),
        }
    }
}
```

### 递归嵌套路由：无限深度支持

```rust
use ruled_router::prelude::*;

// 三层嵌套路由示例
#[derive(RouterMatch)]
enum AppRouterMatch {
    User(ModuleRoute),   // 自动提取: "/user"
    Shop(ModuleRoute),   // 自动提取: "/shop"
    Admin(ModuleRoute),  // 自动提取: "/admin"
}

#[derive(Router)]
#[router(pattern = "/:module")]  // 动态模块路由
struct ModuleRoute {
    module: String,
    #[sub_router]
    sub_router: Option<SubRouterMatch>,
}

#[derive(RouterMatch)]
enum SubRouterMatch {
    Category(CategoryRoute),  // 自动提取: "/category"
    Settings(SettingsRoute), // 自动提取: "/settings"
}

#[derive(Router)]
#[router(pattern = "/category/:category_id")]
struct CategoryRoute {
    category_id: u32,
    #[query]
    query: CategoryQuery,
    #[sub_router]
    sub_router: Option<DetailRouterMatch>,  // 支持更深层嵌套
}

#[derive(RouterMatch)]
enum DetailRouterMatch {
    Item(ItemDetailRoute),     // 自动提取: "/item"
    Review(ReviewDetailRoute), // 自动提取: "/review"
}

#[derive(Router)]
#[router(pattern = "/item/:item_id")]
struct ItemDetailRoute {
    item_id: u32,
    #[query]
    query: ItemQuery,
}

#[derive(Query)]
struct CategoryQuery {
    #[query(name = "page", default = "1")]
    page: u32,
    #[query(name = "limit", default = "10")]
    limit: u32,
}

#[derive(Query)]
struct ItemQuery {
    #[query(name = "format")]
    format: Option<String>,
    #[query(name = "include", multiple)]
    include: Vec<String>,
}

fn main() {
    // 解析三层嵌套路由
    let path = "/user/category/123/item/456?format=json&include=details&include=reviews";
    
    if let Ok(route) = AppRouterMatch::try_parse(path) {
        match route {
            AppRouterMatch::User(module_route) => {
                println!("模块: {}", module_route.module);
                
                if let Some(SubRouterMatch::Category(category_route)) = &module_route.sub_router {
                    println!("分类ID: {}", category_route.category_id);
                    
                    if let Some(DetailRouterMatch::Item(item_route)) = &category_route.sub_router {
                        println!("商品ID: {}", item_route.item_id);
                        println!("格式: {:?}", item_route.query.format);
                        println!("包含: {:?}", item_route.query.include);
                    }
                }
            }
            _ => {}
        }
    }
    
    // 输出:
    // 模块: user
    // 分类ID: 123
    // 商品ID: 456
    // 格式: Some("json")
    // 包含: ["details", "reviews"]
}
```

## 核心设计理念

### 面向数据编程

**Ruled Router** 采用面向数据编程范式，核心思想是"数据即代码"：

- **数据结构定义路由**：通过 struct 和 enum 声明路由形状
- **宏自动生成逻辑**：解析、格式化、匹配逻辑完全自动化
- **零重复定义**：自动前缀提取消除路径信息重复
- **组合式设计**：通过数据结构组合实现复杂路由

```rust
// 传统方式：手写解析逻辑
fn parse_user_route(path: &str) -> Result<UserRoute, Error> {
    // 大量手写的字符串解析代码...
}

// 面向数据编程：声明式定义
#[derive(Router)]
#[router(pattern = "/users/:id")]
struct UserRoute { id: u32 }
// 解析逻辑自动生成，零错误，零维护
```

### 自动前缀提取：DRY 原则

传统路由系统需要重复定义路径信息：

```rust
// ❌ 传统方式 - 路径重复定义
#[derive(RouterMatch)]
enum AppRouterMatch {
    #[route("/users")]     // 重复定义
    User(UserRoute),
}

#[derive(Router)]
#[router(pattern = "/users/:id")]  // 重复定义
struct UserRoute { id: u32 }
```

**Ruled Router** 通过自动前缀提取实现 DRY 原则：

```rust
// ✅ 自动前缀提取 - 路径只定义一次
#[derive(RouterMatch)]
enum AppRouterMatch {
    User(UserRoute),  // 自动从 UserRoute::pattern() 提取前缀
}

#[derive(Router)]
#[router(pattern = "/users/:id")]  // 只在这里定义一次
struct UserRoute { id: u32 }
```

### 宏驱动的代码生成
- 参考 `argh` 的实现方式，使用过程宏自动生成解析和格式化逻辑
- 通过属性宏标注结构体字段，定义路由段的解析规则
- 编译时生成高效的解析器代码，运行时零成本抽象

### 结构化路由定义
- 使用 Rust 结构体定义路由结构
- 支持嵌套结构体组合复杂路由路径
- 类型安全的路由参数处理

## 功能特性

### 🎯 面向数据编程
- **数据即代码**：通过数据结构定义路由，逻辑自动生成
- **零重复定义**：自动前缀提取，路径信息只需定义一次
- **组合式设计**：通过结构体和枚举组合实现复杂路由
- **声明式语法**：无需手写解析代码，专注业务逻辑

### 🔄 自动前缀提取
- **DRY 原则**：RouterMatch 自动从 Router 类型提取路由前缀
- **零维护成本**：路径变更只需修改一处，逻辑自动更新
- **类型安全**：编译时验证路由前缀的一致性
- **手动覆盖**：支持 `#[route]` 属性手动指定前缀（可选）

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

## 使用指南

### 安装

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
ruled-router = "0.1.0"
```

### 基本概念

#### Router vs RouterMatch

- **Router**：具体的路由结构体，负责解析路径参数和查询参数
- **RouterMatch**：路由匹配器枚举，负责路由分发和前缀匹配

```rust
// Router - 具体路由
#[derive(Router)]
#[router(pattern = "/users/:id")]
struct UserRoute {
    id: u32,
    #[query]
    query: UserQuery,
}

// RouterMatch - 路由匹配器
#[derive(RouterMatch)]
enum AppRouterMatch {
    User(UserRoute),    // 自动提取前缀: "/users"
    Blog(BlogRoute),    // 自动提取前缀: "/blog"
}
```

#### 属性说明

- `#[router(pattern = "...")]`：定义路由模式，支持参数占位符 `:param`
- `#[query]`：标记查询参数字段
- `#[sub_router]`：标记子路由字段，支持嵌套路由
- `#[query(name = "...")]`：自定义查询参数名
- `#[query(default = "...")]`：设置查询参数默认值
- `#[query(multiple)]`：支持多值查询参数

### 最佳实践

#### 1. 模块化路由设计

```rust
// 按功能模块组织路由
mod user {
    use super::*;
    
    #[derive(RouterMatch)]
    pub enum UserRouterMatch {
        Profile(UserProfileRoute),
        Settings(UserSettingsRoute),
        Posts(UserPostsRoute),
    }
    
    #[derive(Router)]
    #[router(pattern = "/profile/:id")]
    pub struct UserProfileRoute {
        pub id: u32,
        #[query]
        pub query: ProfileQuery,
    }
}

mod blog {
    // 博客相关路由...
}

// 顶层路由聚合
#[derive(RouterMatch)]
enum AppRouterMatch {
    User(user::UserRouterMatch),
    Blog(blog::BlogRouterMatch),
}
```

#### 2. 查询参数设计

```rust
#[derive(Query)]
struct ListQuery {
    #[query(name = "page", default = "1")]
    page: u32,
    
    #[query(name = "limit", default = "20")]
    limit: u32,
    
    #[query(name = "sort")]
    sort: Option<SortOrder>,
    
    #[query(name = "filter", multiple)]
    filters: Vec<String>,
}

#[derive(FromParam, ToParam)]
enum SortOrder {
    Asc,
    Desc,
}
```

#### 3. 错误处理

```rust
use ruled_router::ParseError;

fn handle_route(path: &str) {
    match AppRouterMatch::try_parse(path) {
        Ok(route) => {
            // 处理成功解析的路由
            println!("路由解析成功: {:?}", route);
        }
        Err(ParseError::InvalidPath(msg)) => {
            println!("无效路径: {}", msg);
        }
        Err(ParseError::MissingParameter(param)) => {
            println!("缺少参数: {}", param);
        }
        Err(ParseError::InvalidParameter { param, value, expected }) => {
            println!("参数 {} 的值 {} 无效，期望: {}", param, value, expected);
        }
        Err(e) => {
            println!("其他错误: {:?}", e);
        }
    }
}
```

## 技术架构

### 核心组件

1. **ruled-router-core**: 核心 trait 定义和基础功能
2. **ruled-router-derive**: 过程宏实现，包含 Router 和 RouterMatch 派生宏
3. **ruled-router**: 主库，重新导出所有功能

### 宏系统设计

#### 1. 主要宏
- `#[derive(Router)]`: 为结构体生成路由解析器
- `#[derive(RouterMatch)]`: 为枚举生成路由匹配器
- `#[derive(Query)]`: 为结构体生成查询参数解析器
- `#[router(pattern = "...")]`: 定义路由路径模式
- `#[query]`: 标记查询参数字段
- `#[sub_router]`: 标记子路由字段

#### 2. 生成的 Trait
```rust
trait Router: Sized {
    fn parse(path: &str) -> Result<Self, ParseError>;
    fn format(&self) -> String;
    fn pattern() -> &'static str;
}

trait RouterMatch: Sized {
    fn try_parse(path: &str) -> Result<Self, ParseError>;
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
- 自动前缀提取和匹配

#### 2. 类型转换器
- 字符串到各种类型的转换
- 自定义类型转换支持（FromParam/ToParam）
- 错误处理和回退机制

#### 3. 路由匹配器
- 基于前缀的快速匹配算法
- 嵌套路由递归解析
- 优先级处理和冲突检测

### 设计模式

- **面向数据编程**: 数据结构即路由定义，逻辑由宏生成
- **组合模式**: 通过结构体和枚举组合实现复杂路由
- **访问者模式**: 用于遍历和处理嵌套路由结构
- **策略模式**: 支持不同的路由匹配和解析策略

### 性能优化

- **编译时代码生成**: 运行时零开销抽象
- **自动前缀提取**: 避免运行时字符串比较
- **零分配解析**: 使用栈上数据结构，避免堆分配
- **编译器优化**: 生成的代码可被编译器深度优化

## 示例项目

查看 `examples/` 目录中的完整示例：

- `basic_usage.rs` - 基本路由解析和格式化
- `nested_router_usage.rs` - 三层嵌套路由示例
- `query_params.rs` - 复杂查询参数处理
- `custom_types.rs` - 自定义类型支持

运行示例：

```bash
# 运行嵌套路由示例
cargo run --example nested_router_usage

# 运行基本用法示例
cargo run --example basic_usage
```

## 测试

运行测试套件：

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test router_derive_tests
cargo test query_derive_tests

# 运行性能测试
cargo test --release performance_tests
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
# 克隆仓库
git clone https://github.com/your-username/ruled-router.rs.git
cd ruled-router.rs

# 安装依赖
cargo build

# 运行测试
cargo test

# 检查代码格式
cargo fmt --check

# 运行 clippy
cargo clippy -- -D warnings
```

### 代码规范

- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 检查代码质量
- 为新功能添加测试
- 更新相关文档
- 遵循现有的代码风格

### 报告问题

如果你发现了 bug 或有功能建议，请在 [GitHub Issues](https://github.com/your-username/ruled-router.rs/issues) 中创建一个 issue。

## 路线图

- [ ] 支持更多内置类型（DateTime、UUID 等）
- [ ] 添加路由中间件支持
- [ ] 实现路由缓存机制
- [ ] 支持异步路由处理
- [ ] 添加 OpenAPI 文档生成
- [ ] 性能基准测试和优化

## 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 致谢

- 感谢 [argh](https://github.com/google/argh) 项目的设计灵感
- 感谢 Rust 社区的宏系统设计
- 感谢所有贡献者的努力