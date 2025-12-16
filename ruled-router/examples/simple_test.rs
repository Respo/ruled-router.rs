//! 一个简单的测试示例，用于验证文档注释提取功能

use ruled_router::prelude::*;
use ruled_router_derive::{RouterData, RouterMatch};

/// 这是一个测试路由
#[derive(Debug, Clone, RouterData)]
#[router(pattern = "/test")]
pub struct TestRoute {}

/// 这是一个测试路由器匹配器
#[derive(Debug, Clone, RouterMatch)]
pub enum TestRouterMatch {
  /// 测试路由变体
  Test(TestRoute),
}

fn main() {
  let route = TestRoute {};
  let matcher = TestRouterMatch::Test(route);

  println!("Debug format output:");
  println!("{}", matcher.debug_format(0));
}
