//! DOM 功能模块
//!
//! 此模块提供了与 web 浏览器 DOM 交互的功能，包括：
//! - 路由监听 (popstate 事件)
//! - 路由跳转 (History API)
//! - URL 解析和处理
//!
//! 只有在启用 `dom` feature 时才会编译此模块。

use crate::error::ParseError;
use crate::traits::Router;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, window, Document, Event, History, HtmlElement, Location};

/// DOM 路由管理器
///
/// 负责管理浏览器路由的监听和跳转功能
#[derive(Clone)]
pub struct DomRouter<T: Router> {
  history: History,
  location: Location,
  _phantom: PhantomData<T>,
  listeners: Rc<RefCell<Vec<Box<dyn Fn(&T)>>>>,
}

impl<T: Router + Clone + 'static> DomRouter<T> {
  /// 创建新的 DOM 路由管理器
  ///
  /// # 返回值
  ///
  /// 成功时返回 DomRouter 实例，失败时返回 JsValue 错误
  ///
  /// # 示例
  ///
  /// ```no_run
  /// use ruled_router::dom::DomRouter;
  /// use ruled_router::prelude::*;
  /// use ruled_router::NoSubRouter;
  ///
  /// #[derive(Debug, Clone, PartialEq)]
  /// struct MyRoute;
  ///
  /// impl Router for MyRoute {
  ///     type SubRouterMatch = NoSubRouter;
  ///     fn parse(path: &str) -> Result<Self, ParseError> { Ok(MyRoute) }
  ///     fn format(&self) -> String { "/".to_string() }
  ///     fn pattern() -> &'static str { "/" }
  /// }
  ///
  /// let router = DomRouter::<MyRoute>::new()?;
  /// # Ok::<(), wasm_bindgen::JsValue>(())
  /// ```
  pub fn new() -> Result<Self, JsValue> {
    let window = window().ok_or("无法获取 window 对象")?;
    let history = window.history()?;
    let location = window.location();

    Ok(DomRouter {
      history,
      location,
      _phantom: PhantomData,
      listeners: Rc::new(RefCell::new(Vec::new())),
    })
  }

  /// 获取当前路径
  ///
  /// # 返回值
  ///
  /// 当前的完整路径，包括查询参数
  pub fn current_path(&self) -> Result<String, JsValue> {
    let pathname = self.location.pathname()?;
    let search = self.location.search()?;
    Ok(format!("{pathname}{search}"))
  }

  /// 解析当前路径为路由对象
  ///
  /// # 返回值
  ///
  /// 成功时返回解析后的路由对象，失败时返回 ParseError
  pub fn current_route(&self) -> Result<T, ParseError> {
    let path = self
      .current_path()
      .map_err(|_| ParseError::InvalidPath("无法获取当前路径".to_string()))?;
    T::parse(&path)
  }

  /// 导航到指定路由
  ///
  /// # 参数
  ///
  /// * `route` - 要导航到的路由对象
  /// * `replace` - 是否替换当前历史记录条目（true）还是添加新条目（false）
  ///
  /// # 返回值
  ///
  /// 成功时返回 ()，失败时返回 JsValue 错误
  ///
  /// # 示例
  ///
  /// ```no_run
  /// # use ruled_router::dom::DomRouter;
  /// # use ruled_router::prelude::*;
  /// # use ruled_router::NoSubRouter;
  /// # #[derive(Debug, Clone, PartialEq)]
  /// # struct MyRoute;
  /// # impl Router for MyRoute {
  /// #     type SubRouterMatch = NoSubRouter;
  /// #     fn parse(path: &str) -> Result<Self, ParseError> { Ok(MyRoute) }
  /// #     fn format(&self) -> String { "/".to_string() }
  /// #     fn pattern() -> &'static str { "/" }
  /// # }
  /// # let router = DomRouter::<MyRoute>::new()?;
  /// # let route = MyRoute;
  /// // 添加新的历史记录条目
  /// router.navigate_to(&route, false)?;
  ///
  /// // 替换当前历史记录条目
  /// router.navigate_to(&route, true)?;
  /// # Ok::<(), wasm_bindgen::JsValue>(())
  /// ```
  pub fn navigate_to(&self, route: &T, replace: bool) -> Result<(), JsValue> {
    let url = route.format();

    if replace {
      self.history.replace_state_with_url(&JsValue::NULL, "", Some(&url))?;
    } else {
      self.history.push_state_with_url(&JsValue::NULL, "", Some(&url))?;
    }

    // 手动触发路由变化事件
    self.trigger_route_change();

    Ok(())
  }

  /// 后退到上一个页面
  ///
  /// # 返回值
  ///
  /// 成功时返回 ()，失败时返回 JsValue 错误
  pub fn go_back(&self) -> Result<(), JsValue> {
    self.history.back()
  }

  /// 前进到下一个页面
  ///
  /// # 返回值
  ///
  /// 成功时返回 ()，失败时返回 JsValue 错误
  pub fn go_forward(&self) -> Result<(), JsValue> {
    self.history.forward()
  }

  /// 添加路由变化监听器
  ///
  /// # 参数
  ///
  /// * `callback` - 路由变化时的回调函数，接收新路由作为参数
  ///
  /// # 示例
  ///
  /// ```no_run
  /// # use ruled_router::dom::DomRouter;
  /// # use ruled_router::prelude::*;
  /// # use ruled_router::NoSubRouter;
  /// # use web_sys::console;
  /// # #[derive(Debug, Clone, PartialEq)]
  /// # struct MyRoute;
  /// # impl Router for MyRoute {
  /// #     type SubRouterMatch = NoSubRouter;
  /// #     fn parse(path: &str) -> Result<Self, ParseError> { Ok(MyRoute) }
  /// #     fn format(&self) -> String { "/".to_string() }
  /// #     fn pattern() -> &'static str { "/" }
  /// # }
  /// # let router = DomRouter::<MyRoute>::new().unwrap();
  /// router.add_listener(|route: &MyRoute| {
  ///     console::log_1(&format!("路由变化: {:?}", route).into());
  /// });
  /// ```
  pub fn add_listener<F>(&self, callback: F)
  where
    F: Fn(&T) + 'static,
  {
    self.listeners.borrow_mut().push(Box::new(callback));
  }

  /// 开始监听路由变化
  ///
  /// 这会设置 popstate 事件监听器来响应浏览器的前进/后退按钮
  ///
  /// # 返回值
  ///
  /// 成功时返回 ()，失败时返回 JsValue 错误
  pub fn start_listening(&self) -> Result<(), JsValue> {
    let current_window = window().ok_or("无法获取 window 对象")?;
    let listeners = self.listeners.clone();

    let closure = Closure::wrap(Box::new(move |_event: Event| {
      // 尝试解析当前路径
      if let Some(win) = window() {
        let location = win.location();
        if let (Ok(pathname), Ok(search)) = (location.pathname(), location.search()) {
          let path = format!("{pathname}{search}");
          if let Ok(route) = T::parse(&path) {
            // 调用所有监听器
            for listener in listeners.borrow().iter() {
              listener(&route);
            }
          }
        }
      }
    }) as Box<dyn Fn(Event)>);

    current_window.add_event_listener_with_callback("popstate", closure.as_ref().unchecked_ref())?;

    // 防止闭包被垃圾回收
    closure.forget();

    Ok(())
  }

  /// 手动触发路由变化事件
  fn trigger_route_change(&self) {
    if let Ok(route) = self.current_route() {
      for listener in self.listeners.borrow().iter() {
        listener(&route);
      }
    }
  }
}

/// 路由辅助函数集合
pub mod helpers {
  use super::*;

  /// 获取当前页面的路径
  pub fn get_current_path() -> Result<String, JsValue> {
    let window = window().ok_or("无法获取 window 对象")?;
    let location = window.location();
    let pathname = location.pathname()?;
    let search = location.search()?;
    Ok(format!("{pathname}{search}"))
  }

  /// 设置页面标题
  pub fn set_title(title: &str) -> Result<(), JsValue> {
    let window = window().ok_or("无法获取 window 对象")?;
    let document = window.document().ok_or("无法获取 document 对象")?;
    document.set_title(title);
    Ok(())
  }

  /// 获取文档元素
  pub fn get_document() -> Result<Document, JsValue> {
    let window = window().ok_or("无法获取 window 对象")?;
    window.document().ok_or("无法获取 document 对象".into())
  }

  /// 根据 ID 获取 HTML 元素
  pub fn get_element_by_id(id: &str) -> Result<Option<HtmlElement>, JsValue> {
    let document = get_document()?;
    let element = document.get_element_by_id(id);

    match element {
      Some(el) => {
        match el.dyn_into::<HtmlElement>() {
          Ok(html_el) => Ok(Some(html_el)),
          Err(_) => Ok(None), // Element exists but is not an HtmlElement
        }
      }
      None => Ok(None),
    }
  }

  /// 日志输出到浏览器控制台
  pub fn log(message: &str) {
    console::log_1(&message.into());
  }
}

/// 为 DOM 路由管理器实现 Default trait
impl<T: Router + Clone + 'static> Default for DomRouter<T> {
  fn default() -> Self {
    Self::new().expect("无法创建 DomRouter")
  }
}
