//! debug_format 方法使用示例 - 深层嵌套路由结构
//!
//! 这个示例展示了如何使用 RouterMatch 的 debug_format 方法
//! 来打印复杂的多层嵌套路由结构，用于开发过程中的验证。

use ruled_router::prelude::*;
use ruled_router::RouteMatcher;
use ruled_router_derive::{QueryDerive, RouterData, RouterMatch};

// ===== 查询参数结构 =====

#[derive(Debug, Clone, PartialEq, Default, QueryDerive)]
struct SimpleQuery {
  #[query(name = "page")]
  page: Option<u32>,
  #[query(name = "limit")]
  limit: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Default, QueryDerive)]
struct AdminQuery {
  #[query(name = "token")]
  token: Option<String>,
  #[query(name = "debug")]
  debug: Option<bool>,
}

// ===== 第三层：详细路由（最深层） =====

#[derive(Debug, Clone, PartialEq, RouterData)]
#[router(pattern = "/basic/:id")]
struct UserBasicInfoRoute {
  id: u32,
  #[query]
  query: SimpleQuery,
}

#[derive(Debug, Clone, PartialEq, RouterData)]
#[router(pattern = "/settings")]
struct UserSettingsRoute {
  #[query]
  query: SimpleQuery,
}

#[derive(Debug, Clone, PartialEq, RouterData)]
#[router(pattern = "/posts/:id")]
struct UserPostRoute {
  id: u32,
  #[query]
  query: SimpleQuery,
}

#[derive(Debug, Clone, PartialEq, RouterData)]
#[router(pattern = "/comments/:id")]
struct UserCommentRoute {
  id: u32,
  #[query]
  query: SimpleQuery,
}

#[derive(Debug, Clone, PartialEq, RouterData)]
#[router(pattern = "/manage")]
struct AdminUserManageRoute {
  #[query]
  query: AdminQuery,
}

#[derive(Debug, Clone, PartialEq, RouterData)]
#[router(pattern = "/config")]
struct SystemConfigRoute {
  #[query]
  query: AdminQuery,
}

// ===== 第三层：详情路由匹配器 =====

#[derive(Debug, Clone, PartialEq, RouterMatch)]
enum UserProfileDetailRouterMatch {
  BasicInfo(UserBasicInfoRoute),
  Settings(UserSettingsRoute),
}

#[derive(Debug, Clone, PartialEq, RouterMatch)]
enum UserContentDetailRouterMatch {
  Post(UserPostRoute),
  Comment(UserCommentRoute),
}

#[derive(Debug, Clone, PartialEq, RouterMatch)]
enum AdminUserDetailRouterMatch {
  Manage(AdminUserManageRoute),
}

#[derive(Debug, Clone, PartialEq, RouterMatch)]
enum AdminSystemDetailRouterMatch {
  Config(SystemConfigRoute),
}

// ===== 第二层：分类路由 =====

#[derive(Debug, Clone, PartialEq, RouterData)]
#[router(pattern = "/profile")]
struct UserProfileCategoryRoute {
  #[query]
  query: SimpleQuery,
  #[sub_router]
  sub_router: Option<UserProfileDetailRouterMatch>,
}

#[derive(Debug, Clone, PartialEq, RouterData)]
#[router(pattern = "/content")]
struct UserContentCategoryRoute {
  #[query]
  query: SimpleQuery,
  #[sub_router]
  sub_router: Option<UserContentDetailRouterMatch>,
}

#[derive(Debug, Clone, PartialEq, RouterData)]
#[router(pattern = "/users")]
struct AdminUserCategoryRoute {
  #[query]
  query: AdminQuery,
  #[sub_router]
  sub_router: Option<AdminUserDetailRouterMatch>,
}

#[derive(Debug, Clone, PartialEq, RouterData)]
#[router(pattern = "/system")]
struct AdminSystemCategoryRoute {
  #[query]
  query: AdminQuery,
  #[sub_router]
  sub_router: Option<AdminSystemDetailRouterMatch>,
}

// ===== 第二层：子路由匹配器 =====

#[derive(Debug, Clone, PartialEq, RouterMatch)]
enum UserSubRouterMatch {
  Profile(UserProfileCategoryRoute),
  Content(UserContentCategoryRoute),
}

#[derive(Debug, Clone, PartialEq, RouterMatch)]
enum AdminSubRouterMatch {
  Users(AdminUserCategoryRoute),
  System(AdminSystemCategoryRoute),
}

// ===== 第一层：模块路由 =====

#[derive(Debug, Clone, PartialEq, RouterData)]
#[router(pattern = "/users")]
struct UserModuleRoute {
  #[query]
  query: SimpleQuery,
  #[sub_router]
  sub_router: Option<UserSubRouterMatch>,
}

#[derive(Debug, Clone, PartialEq, RouterData)]
#[router(pattern = "/admin")]
struct AdminModuleRoute {
  #[query]
  query: AdminQuery,
  #[sub_router]
  sub_router: Option<AdminSubRouterMatch>,
}

// ===== 顶层路由匹配器 =====

#[derive(Debug, Clone, PartialEq, RouterMatch)]
enum AppRouterMatch {
  User(UserModuleRoute),
  Admin(AdminModuleRoute),
}

fn main() {
  // 创建深层嵌套的路由实例（3层嵌套结构）

  // 用户模块 -> 个人资料分类 -> 基本信息详情（3层嵌套）
  let user_profile_basic = AppRouterMatch::User(UserModuleRoute {
    query: SimpleQuery {
      page: Some(1),
      limit: Some(10),
    },
    sub_router: Some(UserSubRouterMatch::Profile(UserProfileCategoryRoute {
      query: SimpleQuery {
        page: None,
        limit: Some(20),
      },
      sub_router: Some(UserProfileDetailRouterMatch::BasicInfo(UserBasicInfoRoute {
        id: 123,
        query: SimpleQuery::default(),
      })),
    })),
  });

  // 用户模块 -> 个人资料分类 -> 设置详情（3层嵌套）
  let user_profile_settings = AppRouterMatch::User(UserModuleRoute {
    query: SimpleQuery {
      page: Some(2),
      limit: None,
    },
    sub_router: Some(UserSubRouterMatch::Profile(UserProfileCategoryRoute {
      query: SimpleQuery::default(),
      sub_router: Some(UserProfileDetailRouterMatch::Settings(UserSettingsRoute {
        query: SimpleQuery {
          page: Some(1),
          limit: Some(5),
        },
      })),
    })),
  });

  // 用户模块 -> 内容分类 -> 帖子详情（3层嵌套）
  let user_content_post = AppRouterMatch::User(UserModuleRoute {
    query: SimpleQuery::default(),
    sub_router: Some(UserSubRouterMatch::Content(UserContentCategoryRoute {
      query: SimpleQuery {
        page: Some(3),
        limit: Some(15),
      },
      sub_router: Some(UserContentDetailRouterMatch::Post(UserPostRoute {
        id: 456,
        query: SimpleQuery::default(),
      })),
    })),
  });

  // 用户模块 -> 内容分类 -> 评论详情（3层嵌套）
  let user_content_comment = AppRouterMatch::User(UserModuleRoute {
    query: SimpleQuery::default(),
    sub_router: Some(UserSubRouterMatch::Content(UserContentCategoryRoute {
      query: SimpleQuery::default(),
      sub_router: Some(UserContentDetailRouterMatch::Comment(UserCommentRoute {
        id: 789,
        query: SimpleQuery {
          page: Some(1),
          limit: None,
        },
      })),
    })),
  });

  // 管理员模块 -> 用户管理分类 -> 管理详情（3层嵌套）
  let admin_user_manage = AppRouterMatch::Admin(AdminModuleRoute {
    query: AdminQuery {
      token: Some("admin123".to_string()),
      debug: Some(true),
    },
    sub_router: Some(AdminSubRouterMatch::Users(AdminUserCategoryRoute {
      query: AdminQuery {
        token: Some("user_token".to_string()),
        debug: Some(false),
      },
      sub_router: Some(AdminUserDetailRouterMatch::Manage(AdminUserManageRoute {
        query: AdminQuery {
          token: None,
          debug: Some(true),
        },
      })),
    })),
  });

  // 管理员模块 -> 系统分类 -> 配置详情（3层嵌套）
  let admin_system_config = AppRouterMatch::Admin(AdminModuleRoute {
    query: AdminQuery {
      token: Some("system_admin".to_string()),
      debug: Some(false),
    },
    sub_router: Some(AdminSubRouterMatch::System(AdminSystemCategoryRoute {
      query: AdminQuery::default(),
      sub_router: Some(AdminSystemDetailRouterMatch::Config(SystemConfigRoute {
        query: AdminQuery {
          token: Some("config_token".to_string()),
          debug: Some(true),
        },
      })),
    })),
  });

  // 用户模块（仅2层嵌套）
  let user_profile_only = AppRouterMatch::User(UserModuleRoute {
    query: SimpleQuery {
      page: Some(1),
      limit: Some(25),
    },
    sub_router: Some(UserSubRouterMatch::Profile(UserProfileCategoryRoute {
      query: SimpleQuery::default(),
      sub_router: None, // 没有第三层
    })),
  });

  // 管理员模块（仅1层）
  let admin_only = AppRouterMatch::Admin(AdminModuleRoute {
    query: AdminQuery {
      token: Some("simple_admin".to_string()),
      debug: None,
    },
    sub_router: None, // 没有子路由
  });

  // 创建应用路由实例集合
  let app_routes = vec![
    user_profile_basic,
    user_profile_settings,
    user_content_post,
    user_content_comment,
    admin_user_manage,
    admin_system_config,
    user_profile_only,
    admin_only,
  ];

  // 展示深层嵌套路由的树形结构
  println!("深层嵌套路由树 (1-3层嵌套):");

  // 按模块分组展示路由
  let mut user_routes = Vec::new();
  let mut admin_routes = Vec::new();

  for route in app_routes {
    match route {
      AppRouterMatch::User(user_route) => user_routes.push(user_route),
      AppRouterMatch::Admin(admin_route) => admin_routes.push(admin_route),
    }
  }

  // 展示用户模块路由
  if !user_routes.is_empty() {
    println!("├─ AppRouterMatch::User");
    println!("│  ├─ Pattern: /users");
    println!("│  └─ Sub Routes:");
    for (i, user_route) in user_routes.iter().enumerate() {
      let is_last = i == user_routes.len() - 1;
      let prefix = if is_last { "│     └─" } else { "│     ├─" };
      let continuation = if is_last { "│        " } else { "│     │  " };

      // 根据子路由类型确定路由名称
      let route_name = match &user_route.sub_router {
        Some(UserSubRouterMatch::Profile(_)) => "Profile",
        Some(UserSubRouterMatch::Content(_)) => "Content",
        None => "Base",
      };

      println!("{prefix} {route_name}");

      // 简化查询参数显示
      let query_info = if user_route.query.page.is_some() || user_route.query.limit.is_some() {
        format!(
          "page={:?}, limit={:?}",
          user_route.query.page.unwrap_or(0),
          user_route.query.limit.unwrap_or(0)
        )
      } else {
        "∅".to_string()
      };
      println!("{continuation}├─ Query: {query_info}");

      if let Some(sub_router) = &user_route.sub_router {
        println!("{continuation}└─ Sub:");
        let sub_debug = sub_router.debug_format(0);
        for line in sub_debug.lines() {
          println!("{continuation}   {line}");
        }
      } else {
        println!("{continuation}└─ ◉");
      }
    }
  }

  // 展示管理员模块路由
  if !admin_routes.is_empty() {
    println!("└─ AppRouterMatch::Admin");
    println!("   ├─ Pattern: /admin");
    println!("   └─ Sub Routes:");
    for (i, admin_route) in admin_routes.iter().enumerate() {
      let is_last = i == admin_routes.len() - 1;
      let prefix = if is_last { "      └─" } else { "      ├─" };
      let continuation = if is_last { "         " } else { "      │  " };

      // 根据子路由类型确定路由名称
      let route_name = match &admin_route.sub_router {
        Some(AdminSubRouterMatch::Users(_)) => "Users",
        Some(AdminSubRouterMatch::System(_)) => "System",
        None => "Base",
      };

      println!("{prefix} {route_name}");

      // 简化查询参数显示
      let query_info = if admin_route.query.token.is_some() || admin_route.query.debug.is_some() {
        format!(
          "token={:?}, debug={:?}",
          admin_route.query.token.as_deref().unwrap_or("None"),
          admin_route.query.debug.unwrap_or(false)
        )
      } else {
        "∅".to_string()
      };
      println!("{continuation}├─ Query: {query_info}");

      if let Some(sub_router) = &admin_route.sub_router {
        println!("{continuation}└─ Sub:");
        let sub_debug = sub_router.debug_format(0);
        for line in sub_debug.lines() {
          println!("{continuation}   {line}");
        }
      } else {
        println!("{continuation}└─ ◉");
      }
    }
  }
}
