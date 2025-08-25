//! Web Page Example for ruled-router
//!
//! 这个示例展示了如何在浏览器环境中使用 ruled-router 的 DOM 功能，
//! 包括路由监听、导航和页面渲染。

use ruled_router::prelude::*;
use ruled_router::NoSubRouter;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, window, Event, HtmlElement, HtmlInputElement};

/// 应用路由定义
#[derive(Debug, Clone, PartialEq)]
enum AppRoute {
  Home,
  User { id: u32 },
  BlogPost { year: u32, month: u32, slug: String },
  Search,
}

impl Router for AppRoute {
  type SubRouterMatch = NoSubRouter;

  fn parse(path: &str) -> Result<Self, ParseError> {
    // 手动解析路径
    let (path_part, _) = if let Some(pos) = path.find('?') {
      path.split_at(pos)
    } else {
      (path, "")
    };

    match path_part {
      "/" => Ok(AppRoute::Home),
      "/search" => Ok(AppRoute::Search),
      path if path.starts_with("/users/") => {
        let id_str = &path[7..]; // 跳过 "/users/"
        let id = id_str
          .parse::<u32>()
          .map_err(|_| ParseError::TypeConversion(format!("无法将 '{id_str}' 转换为数字")))?;
        Ok(AppRoute::User { id })
      }
      path if path.starts_with("/blog/") => {
        let parts: Vec<&str> = path[6..].split('/').collect(); // 跳过 "/blog/"
        if parts.len() == 3 {
          let year = parts[0]
            .parse::<u32>()
            .map_err(|_| ParseError::TypeConversion(format!("无法将 '{}' 转换为年份", parts[0])))?;
          let month = parts[1]
            .parse::<u32>()
            .map_err(|_| ParseError::TypeConversion(format!("无法将 '{}' 转换为月份", parts[1])))?;
          let slug = parts[2].to_string();
          Ok(AppRoute::BlogPost { year, month, slug })
        } else {
          Err(ParseError::InvalidPath(format!("博客路径格式错误: {path}")))
        }
      }
      _ => Err(ParseError::InvalidPath(format!("无法识别的路径: {path}"))),
    }
  }

  fn format(&self) -> String {
    match self {
      AppRoute::Home => "/".to_string(),
      AppRoute::User { id } => format!("/users/{id}"),
      AppRoute::BlogPost { year, month, slug } => format!("/blog/{year}/{month}/{slug}"),
      AppRoute::Search => "/search".to_string(),
    }
  }

  fn pattern() -> &'static str {
    "AppRoute patterns: /, /users/:id, /blog/:year/:month/:slug, /search"
  }
}

/// 搜索查询参数
#[derive(Debug, Clone, PartialEq, Default, Query)]
struct SearchQuery {
  q: Option<String>,
  page: Option<u32>,
  tags: Vec<String>,
}

/// 完整的搜索路由，包含查询参数
#[derive(Debug, Clone, PartialEq)]
struct SearchRoute {
  query: SearchQuery,
}

impl Router for SearchRoute {
  type SubRouterMatch = NoSubRouter;

  fn parse(path: &str) -> Result<Self, ParseError> {
    let (path_part, query_part) = if let Some(pos) = path.find('?') {
      (&path[..pos], &path[pos + 1..])
    } else {
      (path, "")
    };

    if path_part == "/search" {
      let query = if query_part.is_empty() {
        SearchQuery::default()
      } else {
        SearchQuery::parse(query_part)?
      };
      Ok(SearchRoute { query })
    } else {
      Err(ParseError::InvalidPath(format!("不是搜索路径: {path_part}")))
    }
  }

  fn format(&self) -> String {
    let query_string = self.query.format();
    if query_string.is_empty() {
      "/search".to_string()
    } else {
      format!("/search?{query_string}")
    }
  }

  fn pattern() -> &'static str {
    "/search?q=...&page=...&tags=..."
  }
}

/// 应用状态
struct App {
  router: DomRouter<AppRoute>,
  content_element: HtmlElement,
}

impl App {
  /// 创建新的应用实例
  fn new() -> Result<Self, JsValue> {
    let router = DomRouter::<AppRoute>::new()?;
    let document = helpers::get_document()?;
    let content_element = document
      .get_element_by_id("content")
      .ok_or("无法找到 #content 元素")?
      .dyn_into::<HtmlElement>()?;

    Ok(App { router, content_element })
  }

  /// 初始化应用
  fn init(&mut self) -> Result<(), JsValue> {
    // 设置路由监听器
    let content_element = self.content_element.clone();
    self.router.add_listener(move |route: &AppRoute| {
      if let Err(e) = render_route(route, &content_element) {
        console::log_1(&format!("渲染错误: {e:?}").into());
      }
    });

    // 开始监听路由变化
    self.router.start_listening()?;

    // 渲染当前路由
    if let Ok(current_route) = self.router.current_route() {
      render_route(&current_route, &self.content_element)?;
    }

    // 设置导航按钮事件监听器
    self.setup_navigation()?;

    Ok(())
  }

  /// 设置导航按钮的事件监听器
  fn setup_navigation(&self) -> Result<(), JsValue> {
    let document = helpers::get_document()?;
    let router = Rc::new(RefCell::new(self.router.clone()));

    // 首页按钮
    if let Some(home_btn) = document.get_element_by_id("home-btn") {
      let router = router.clone();
      let closure = Closure::wrap(Box::new(move |_: Event| {
        if let Err(e) = router.borrow().navigate_to(&AppRoute::Home, false) {
          console::log_1(&format!("导航错误: {e:?}").into());
        }
      }) as Box<dyn Fn(Event)>);

      home_btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
      closure.forget();
    }

    // 用户页面按钮
    if let Some(user_btn) = document.get_element_by_id("user-btn") {
      let router = router.clone();
      let closure = Closure::wrap(Box::new(move |_: Event| {
        if let Err(e) = router.borrow().navigate_to(&AppRoute::User { id: 123 }, false) {
          console::log_1(&format!("导航错误: {e:?}").into());
        }
      }) as Box<dyn Fn(Event)>);

      user_btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
      closure.forget();
    }

    // 博客页面按钮
    if let Some(blog_btn) = document.get_element_by_id("blog-btn") {
      let router = router.clone();
      let closure = Closure::wrap(Box::new(move |_: Event| {
        if let Err(e) = router.borrow().navigate_to(
          &AppRoute::BlogPost {
            year: 2024,
            month: 12,
            slug: "hello-world".to_string(),
          },
          false,
        ) {
          console::log_1(&format!("导航错误: {e:?}").into());
        }
      }) as Box<dyn Fn(Event)>);

      blog_btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
      closure.forget();
    }

    // 搜索页面按钮
    if let Some(search_btn) = document.get_element_by_id("search-btn") {
      let router = router.clone();
      let closure = Closure::wrap(Box::new(move |_: Event| {
        if let Err(e) = router.borrow().navigate_to(&AppRoute::Search, false) {
          console::log_1(&format!("导航错误: {e:?}").into());
        }
      }) as Box<dyn Fn(Event)>);

      search_btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
      closure.forget();
    }

    Ok(())
  }
}

/// 根据路由渲染页面内容
fn render_route(route: &AppRoute, content_element: &HtmlElement) -> Result<(), JsValue> {
  let html = match route {
    AppRoute::Home => {
      helpers::set_title("首页 - Ruled Router Demo")?;
      r#"
                <h1>欢迎使用 Ruled Router</h1>
                <p>这是一个展示 ruled-router DOM 功能的示例页面。</p>
                <ul>
                    <li>✅ 路由解析和格式化</li>
                    <li>✅ DOM 监听和导航</li>
                    <li>✅ History API 集成</li>
                    <li>✅ 查询参数支持</li>
                </ul>
            "#
    }
    AppRoute::User { id } => {
      helpers::set_title(&format!("用户 {id} - Ruled Router Demo"))?;
      &format!(
        r#"
                <h1>用户页面</h1>
                <p>当前查看用户 ID: <strong>{id}</strong></p>
                <p>这个页面展示了路径参数的解析功能。</p>
                <div>
                    <button onclick="history.back()">返回</button>
                </div>
            "#
      )
    }
    AppRoute::BlogPost { year, month, slug } => {
      helpers::set_title(&format!("{slug} - Ruled Router Blog"))?;
      &format!(
        r#"
                <h1>博客文章</h1>
                <p><strong>标题:</strong> {slug}</p>
                <p><strong>发布时间:</strong> {year}/{month}</p>
                <p>这个页面展示了多个路径参数的解析功能。</p>
                <div>
                    <button onclick="history.back()">返回</button>
                </div>
            "#
      )
    }
    AppRoute::Search => {
      helpers::set_title("搜索 - Ruled Router Demo")?;
      r#"
                <h1>搜索页面</h1>
                <p>这是搜索功能页面。</p>
                <div>
                    <input type="text" id="search-input" placeholder="输入搜索关键词..." style="margin-right: 10px;">
                    <button id="do-search">搜索</button>
                </div>
                <div id="search-results" style="margin-top: 20px;"></div>
                <div>
                    <button onclick="history.back()">返回</button>
                </div>
            "#
    }
  };

  content_element.set_inner_html(html);

  // 如果是搜索页面，设置搜索功能
  if let AppRoute::Search = route {
    setup_search_functionality()?;
  }

  Ok(())
}

/// 设置搜索页面的功能
fn setup_search_functionality() -> Result<(), JsValue> {
  let document = helpers::get_document()?;

  if let Some(search_btn) = document.get_element_by_id("do-search") {
    let closure = Closure::wrap(Box::new(move |_: Event| {
      if let Err(e) = perform_search() {
        console::log_1(&format!("搜索错误: {e:?}").into());
      }
    }) as Box<dyn Fn(Event)>);

    search_btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
    closure.forget();
  }

  Ok(())
}

/// 执行搜索操作
fn perform_search() -> Result<(), JsValue> {
  let document = helpers::get_document()?;

  if let Some(input) = document.get_element_by_id("search-input") {
    if let Ok(input) = input.dyn_into::<HtmlInputElement>() {
      let query = input.value();

      if !query.is_empty() {
        // 创建搜索路由并导航
        let search_route = SearchRoute {
          query: SearchQuery {
            q: Some(query.clone()),
            page: Some(1),
            tags: vec!["demo".to_string()],
          },
        };

        // 更新 URL
        if let Some(window) = window() {
          if let Ok(history) = window.history() {
            let url = search_route.format();
            let _ = history.push_state_with_url(&JsValue::NULL, "", Some(&url));
          }
        }

        // 显示搜索结果
        if let Some(results_div) = document.get_element_by_id("search-results") {
          results_div.set_inner_html(&format!(
            r#"
                        <h3>搜索结果</h3>
                        <p>搜索关键词: <strong>{}</strong></p>
                        <p>当前 URL: <code>{}</code></p>
                        <p>解析后的查询参数:</p>
                        <pre>{:#?}</pre>
                    "#,
            query,
            search_route.format(),
            search_route.query
          ));
        }
      }
    }
  }

  Ok(())
}

/// WASM 绑定入口点
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
  helpers::log("Ruled Router Web Demo 启动中...");

  // 创建并初始化应用
  let mut app = App::new()?;
  app.init()?;

  helpers::log("Ruled Router Web Demo 启动完成!");

  Ok(())
}
