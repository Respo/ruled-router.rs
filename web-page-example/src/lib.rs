//! Web Page Example for ruled-router
//!
//! 这个示例展示了如何在浏览器环境中使用 ruled-router 的 DOM 功能，
//! 包括路由监听、导航和页面渲染。

use std::cell::RefCell;
use std::rc::Rc;

use ruled_router::prelude::*;
use ruled_router::RouteMatcher;
use ruled_router_derive::RouterMatch;
use serde::Serialize;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, window, Event, HtmlElement, HtmlInputElement};

/// 应用状态管理结构体
#[derive(Debug, Clone)]
struct AppState {
  current_route: Option<AppRoute>,
}

impl AppState {
  fn new() -> Self {
    Self { current_route: None }
  }

  fn set_route(&mut self, route: AppRoute) {
    self.current_route = Some(route);
  }

  fn get_route(&self) -> Option<&AppRoute> {
    self.current_route.as_ref()
  }

  fn format_current_url(&self) -> String {
    if let Some(route) = &self.current_route {
      route.format()
    } else {
      "/".to_string()
    }
  }
}

/// 应用路由匹配器 - 顶层路由
#[derive(Debug, Clone, PartialEq, Serialize, RouterMatch)]
enum AppRoute {
  Home(HomeRoute),
  User(UserRoute),
  BlogPost(BlogPostRoute),
  Search(SearchRoute),
}

/// 应用根路由器 - 包装 AppRoute 以符合 DomRouter 的要求
#[derive(Debug, Clone, Router)]
#[router(pattern = "/")]
struct AppRouter {
  #[sub_router]
  sub_router: Option<AppRoute>,
}

/// 首页路由
#[derive(Debug, Clone, PartialEq, Serialize, Router)]
#[router(pattern = "/")]
struct HomeRoute {
  #[query]
  query: SimpleQuery,
}

/// 用户路由
#[derive(Debug, Clone, PartialEq, Serialize, Router)]
#[router(pattern = "/users/:id")]
struct UserRoute {
  id: u32,
  #[query]
  query: SimpleQuery,
}

/// 博客文章路由
#[derive(Debug, Clone, PartialEq, Serialize, Router)]
#[router(pattern = "/blog/:year/:month/:slug")]
struct BlogPostRoute {
  year: u32,
  month: u32,
  slug: String,
  #[query]
  query: SimpleQuery,
}

/// 搜索路由
#[derive(Debug, Clone, PartialEq, Serialize, Router)]
#[router(pattern = "/search")]
struct SearchRoute {
  #[query]
  query: SearchQuery,
}

/// 简单查询参数
#[derive(Debug, Clone, PartialEq, Default, Serialize, Query)]
struct SimpleQuery {
  #[query(name = "format")]
  format: Option<String>,
}

/// 搜索查询参数
#[derive(Debug, Clone, PartialEq, Default, Query, Serialize)]
struct SearchQuery {
  q: Option<String>,
  page: Option<u32>,
  tags: Vec<String>,
}

/// 应用状态
struct App {
  router: AppRoute,
  content_element: HtmlElement,
  state: Rc<RefCell<AppState>>,
}

impl App {
  /// 创建新的应用实例
  fn new() -> Result<Self, JsValue> {
    let document = helpers::get_document()?;
    let content_element = document
      .get_element_by_id("content")
      .ok_or("无法找到 #content 元素")?
      .dyn_into::<HtmlElement>()?;
    let state = Rc::new(RefCell::new(AppState::new()));

    Ok(App {
      router: AppRoute::Home(HomeRoute {
        query: SimpleQuery {
          format: Some("json".to_string()),
        },
      }),
      content_element,
      state,
    })
  }

  /// 初始化应用
  fn init(&mut self) -> Result<(), JsValue> {
    console::log_1(&"开始初始化路由器".into());
    // 设置路由监听器
    let content_element = self.content_element.clone();

    // 渲染当前路由
    console::log_1(&"获取当前路由".into());
    render_route(&self.router, &content_element)?;

    // 设置导航按钮事件监听器
    self.setup_navigation()?;

    // 添加URL监听器（监听浏览器前进/后退按钮）
    self.setup_url_listener()?;

    Ok(())
  }

  /// 设置URL监听器（监听浏览器前进/后退按钮）
  fn setup_url_listener(&self) -> Result<(), JsValue> {
    let app_state = self.state.clone();
    let content_element = self.content_element.clone();

    let closure = Closure::wrap(Box::new(move |_event: Event| {
      console::log_1(&"URL变化被检测到".into());

      // 获取当前URL路径
      if let Some(window) = window() {
        let location = window.location();
        if let Ok(pathname) = location.pathname() {
          console::log_1(&format!("当前URL路径: {pathname}").into());

          // 尝试解析当前URL
          if let Ok(app_router) = AppRouter::parse(&pathname) {
            if let Some(new_route) = app_router.sub_router {
              // 检查是否与内存状态不一致
              let current_state_route = app_state.borrow().get_route().cloned();
              if current_state_route.as_ref() != Some(&new_route) {
                console::log_1(&format!("状态不一致，更新内存状态: {new_route:?}").into());

                // 更新内存状态
                app_state.borrow_mut().set_route(new_route.clone());

                // 更新页面内容
                if let Err(e) = render_route(&new_route, &content_element) {
                  console::log_1(&format!("渲染错误: {e:?}").into());
                }
                if let Err(e) = update_route_json(&new_route) {
                  console::log_1(&format!("更新路由JSON错误: {e:?}").into());
                }
              } else {
                console::log_1(&"状态一致，无需更新".into());
              }
            }
          }
        }
      }
    }) as Box<dyn FnMut(_)>);

    if let Some(window) = window() {
      window.set_onpopstate(Some(closure.as_ref().unchecked_ref()));
    }

    closure.forget();
    Ok(())
  }

  /// 设置导航按钮的事件监听器
  fn setup_navigation(&self) -> Result<(), JsValue> {
    console::log_1(&"开始设置导航按钮事件监听器".into());
    let document = helpers::get_document()?;
    let _router = &self.router;

    // 首页按钮
    if let Some(home_btn) = document.get_element_by_id("home-btn") {
      console::log_1(&"找到首页按钮，设置点击事件".into());
      let app_state = self.state.clone();
      let content_element = self.content_element.clone();
      let closure = Closure::wrap(Box::new(move |event: Event| {
        console::log_1(&"首页按钮被点击".into());
        event.prevent_default();
        let home_route = AppRoute::Home(HomeRoute {
          query: SimpleQuery::default(),
        });
        console::log_1(&format!("准备导航到首页: {home_route:?}").into());
        // 使用状态管理的navigate_to_route方法
        app_state.borrow_mut().set_route(home_route.clone());
        let url = app_state.borrow().format_current_url();
        if let Some(window) = window() {
          if let Ok(history) = window.history() {
            let _ = history.push_state_with_url(&JsValue::NULL, "", Some(&url));
          }
        }
        // 更新页面内容和路由序列化数据
        if let Err(e) = render_route(&home_route, &content_element) {
          console::log_1(&format!("渲染错误: {e:?}").into());
        }
        if let Err(e) = update_route_json(&home_route) {
          console::log_1(&format!("更新路由JSON错误: {e:?}").into());
        }
        console::log_1(&"首页导航成功".into());
      }) as Box<dyn Fn(Event)>);

      home_btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
      closure.forget();
    } else {
      console::log_1(&"警告: 未找到首页按钮 (home-btn)".into());
    }

    // 用户页面按钮
    if let Some(user_btn) = document.get_element_by_id("user-btn") {
      console::log_1(&"找到用户页面按钮，设置点击事件".into());
      let app_state = self.state.clone();
      let content_element = self.content_element.clone();
      let closure = Closure::wrap(Box::new(move |event: Event| {
        console::log_1(&"用户页面按钮被点击".into());
        event.prevent_default();
        let user_route = AppRoute::User(UserRoute {
          id: 123,
          query: SimpleQuery::default(),
        });
        console::log_1(&format!("准备导航到用户页面: {user_route:?}").into());
        // 使用状态管理的navigate_to_route方法
        app_state.borrow_mut().set_route(user_route.clone());
        let url = app_state.borrow().format_current_url();
        console::log_1(&format!("准备导航到用户页面: {url}").into());
        if let Some(window) = window() {
          if let Ok(history) = window.history() {
            let _ = history.push_state_with_url(&JsValue::NULL, "", Some(&url));
            console::log_1(&format!("用户页面导航成功，URL: {url}").into());
          }
        }
        // 更新页面内容和路由序列化数据
        if let Err(e) = render_route(&user_route, &content_element) {
          console::log_1(&format!("渲染错误: {e:?}").into());
        }
        if let Err(e) = update_route_json(&user_route) {
          console::log_1(&format!("更新路由JSON错误: {e:?}").into());
        }
        console::log_1(&"用户页面导航成功".into());
      }) as Box<dyn Fn(Event)>);

      user_btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
      closure.forget();
    } else {
      console::log_1(&"警告: 未找到用户页面按钮 (user-btn)".into());
    }

    // 博客页面按钮
    if let Some(blog_btn) = document.get_element_by_id("blog-btn") {
      console::log_1(&"找到博客页面按钮，设置点击事件".into());
      let app_state = self.state.clone();
      let content_element = self.content_element.clone();
      let closure = Closure::wrap(Box::new(move |event: Event| {
        console::log_1(&"博客页面按钮被点击".into());
        event.prevent_default();
        let blog_route = AppRoute::BlogPost(BlogPostRoute {
          year: 2024,
          month: 12,
          slug: "hello-world".to_string(),
          query: SimpleQuery::default(),
        });
        console::log_1(&format!("准备导航到博客页面: {blog_route:?}").into());
        // 使用状态管理的navigate_to_route方法
        app_state.borrow_mut().set_route(blog_route.clone());
        let url = app_state.borrow().format_current_url();
        if let Some(window) = window() {
          if let Ok(history) = window.history() {
            let _ = history.push_state_with_url(&JsValue::NULL, "", Some(&url));
          }
        }
        // 更新页面内容和路由序列化数据
        if let Err(e) = render_route(&blog_route, &content_element) {
          console::log_1(&format!("渲染错误: {e:?}").into());
        }
        if let Err(e) = update_route_json(&blog_route) {
          console::log_1(&format!("更新路由JSON错误: {e:?}").into());
        }
        console::log_1(&"博客页面导航成功".into());
      }) as Box<dyn Fn(Event)>);

      blog_btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
      closure.forget();
    } else {
      console::log_1(&"警告: 未找到博客页面按钮 (blog-btn)".into());
    }

    // 搜索页面按钮
    if let Some(search_btn) = document.get_element_by_id("search-btn") {
      console::log_1(&"找到搜索页面按钮，设置点击事件".into());
      let app_state = self.state.clone();
      let content_element = self.content_element.clone();
      let closure = Closure::wrap(Box::new(move |event: Event| {
        console::log_1(&"搜索页面按钮被点击".into());
        event.prevent_default();
        let search_route = AppRoute::Search(SearchRoute {
          query: SearchQuery::default(),
        });
        console::log_1(&format!("准备导航到搜索页面: {search_route:?}").into());
        // 使用状态管理的navigate_to_route方法
        app_state.borrow_mut().set_route(search_route.clone());
        let url = app_state.borrow().format_current_url();
        if let Some(window) = window() {
          if let Ok(history) = window.history() {
            let _ = history.push_state_with_url(&JsValue::NULL, "", Some(&url));
          }
        }
        // 更新页面内容和路由序列化数据
        if let Err(e) = render_route(&search_route, &content_element) {
          console::log_1(&format!("渲染错误: {e:?}").into());
        }
        if let Err(e) = update_route_json(&search_route) {
          console::log_1(&format!("更新路由JSON错误: {e:?}").into());
        }
        console::log_1(&"搜索页面导航成功".into());
      }) as Box<dyn Fn(Event)>);

      search_btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
      closure.forget();
    } else {
      console::log_1(&"警告: 未找到搜索页面按钮 (search-btn)".into());
    }

    console::log_1(&"导航按钮事件监听器设置完成".into());
    Ok(())
  }
}

/// 更新路由序列化JSON显示
fn update_route_json(route: &AppRoute) -> Result<(), JsValue> {
  let document = helpers::get_document()?;

  if let Some(json_element) = document.get_element_by_id("route-json") {
    // 根据路由类型获取对应的pattern
    let pattern = match route {
      AppRoute::Home(_) => "/",
      AppRoute::User(_) => "/users/:id",
      AppRoute::BlogPost(_) => "/blog/:year/:month/:slug",
      AppRoute::Search(_) => "/search",
    };

    // 根据路由类型格式化路径
    let formatted_path = match route {
      AppRoute::Home(_) => "/".to_string(),
      AppRoute::User(user_route) => format!("/user/{}", user_route.id),
      AppRoute::BlogPost(blog_route) => format!("/blog/{}/{}/{}", blog_route.year, blog_route.month, blog_route.slug),
      AppRoute::Search(_) => "/search".to_string(),
    };

    // 创建序列化数据结构
    let route_data = serde_json::json!({
      "current_route": route,
      "formatted_path": formatted_path,
      "pattern": pattern,
      "timestamp": js_sys::Date::now(),
      "route_type": match route {
        AppRoute::Home(_) => "Home",
        AppRoute::User(_) => "User",
        AppRoute::BlogPost(_) => "BlogPost",
        AppRoute::Search(_) => "Search",
      },
      "status": "active"
    });

    // 格式化JSON字符串
    let json_str = serde_json::to_string_pretty(&route_data).map_err(|e| JsValue::from_str(&format!("JSON序列化错误: {e}")))?;

    json_element.set_text_content(Some(&json_str));
  }

  Ok(())
}

/// 根据路由渲染页面内容
fn render_route(route: &AppRoute, content_element: &HtmlElement) -> Result<(), JsValue> {
  let html = match route {
    AppRoute::Home(_) => {
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
    AppRoute::User(user_route) => {
      helpers::set_title(&format!("用户 {} - Ruled Router Demo", user_route.id))?;
      &format!(
        r#"
                <h1>用户页面</h1>
                <p>当前查看用户 ID: <strong>{}</strong></p>
                <p>这个页面展示了路径参数的解析功能。</p>
                <div>
                    <button onclick="history.back()">返回</button>
                </div>
            "#,
        user_route.id
      )
    }
    AppRoute::BlogPost(blog_route) => {
      helpers::set_title(&format!("{} - Ruled Router Blog", blog_route.slug))?;
      &format!(
        r#"
                <h1>博客文章</h1>
                <p><strong>标题:</strong> {}</p>
                 <p><strong>发布时间:</strong> {}/{}</p>
                <p>这个页面展示了多个路径参数的解析功能。</p>
                <div>
                    <button onclick="history.back()">返回</button>
                </div>
            "#,
        blog_route.slug, blog_route.year, blog_route.month
      )
    }
    AppRoute::Search(_) => {
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

  // 更新路由JSON显示
  if let Err(e) = update_route_json(route) {
    console::log_1(&format!("更新JSON显示错误: {e:?}").into());
  }

  // 如果是搜索页面，设置搜索功能
  if let AppRoute::Search(_) = route {
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
