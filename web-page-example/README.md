# Web Page Example

这是一个使用 `ruled-router` DOM 功能的 Web 示例。演示了如何在浏览器环境中使用路由解析、监听和导航功能。

## 功能特性

- ✅ **路由解析**: 使用 `Router` trait 解析 URL 路径
- ✅ **路由导航**: 使用 `DomRouter` 进行页面导航
- ✅ **事件监听**: 监听浏览器的前进/后退按钮
- ✅ **查询参数**: 支持复杂的查询参数解析和格式化
- ✅ **History API**: 完整集成浏览器 History API

## 构建和运行

### 1. 安装 wasm-pack

如果还没有安装 `wasm-pack`，请运行：

```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

### 2. 构建项目

```bash
./build.sh
```

或者手动构建：

```bash
wasm-pack build --target web --out-dir pkg --dev
```

### 3. 启动开发服务器

选择以下任一方法启动本地服务器：

**方法 1 - Python (推荐)**:

```bash
python3 -m http.server 8000
```

**方法 2 - Node.js**:

```bash
npx http-server -p 8000 -c-1
```

**方法 3 - Rust miniserve**:

```bash
cargo install miniserve
miniserve . -p 8000
```

### 4. 在浏览器中访问

打开浏览器访问: http://localhost:8000

## 代码结构

### 路由定义

```rust
#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "")]
enum AppRoute {
    #[router(pattern = "/")]
    Home,

    #[router(pattern = "/users/:id")]
    User { id: u32 },

    #[router(pattern = "/blog/:year/:month/:slug")]
    BlogPost { year: u32, month: u32, slug: String },

    #[router(pattern = "/search")]
    Search,
}
```

### DOM 路由管理器

```rust
let mut router = DomRouter::<AppRoute>::new()?;

// 添加路由变化监听器
router.add_listener(|route: &AppRoute| {
    // 处理路由变化
    render_route(route);
});

// 开始监听
router.start_listening()?;

// 导航到新路由
router.navigate_to(&AppRoute::User { id: 123 }, false)?;
```

### 查询参数

```rust
#[derive(Debug, Clone, PartialEq, Default, QueryDerive)]
struct SearchQuery {
    q: Option<String>,
    page: Option<u32>,
    tags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Router)]
#[router(pattern = "/search")]
struct SearchRoute {
    #[router(query)]
    query: SearchQuery,
}
```

## 示例页面

1. **首页** (`/`) - 展示库的功能特性
2. **用户页面** (`/users/:id`) - 演示路径参数解析
3. **博客文章** (`/blog/:year/:month/:slug`) - 演示多个路径参数
4. **搜索页面** (`/search?q=...&page=...`) - 演示查询参数功能

## 调试

- 打开浏览器开发者工具查看控制台日志
- 使用浏览器的前进/后退按钮测试路由监听
- 查看 URL 栏的变化来理解路由格式化

## 依赖说明

此示例模块 (`web-page-example`) 设置为 `publish = false`，不会发布到 crates.io。它仅用于演示目的。

主要依赖：

- `ruled-router` (本地路径，启用 `dom` feature)
- `wasm-bindgen` - Rust/WebAssembly 绑定
- `web-sys` - Web API 绑定
