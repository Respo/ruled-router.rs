//! Implementation of the RouterMatch derive macro

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Variant};

/// 提取枚举变体信息
fn extract_enum_variants(data: &Data) -> syn::Result<Vec<&Variant>> {
  match data {
    Data::Enum(data_enum) => Ok(data_enum.variants.iter().collect()),
    _ => Err(syn::Error::new(
      proc_macro2::Span::call_site(),
      "RouterMatch can only be derived for enums",
    )),
  }
}

/// 提取变体的路由类型
fn extract_route_type(variant: &Variant) -> syn::Result<&syn::Type> {
  match &variant.fields {
    Fields::Unnamed(fields) if fields.unnamed.len() == 1 => Ok(&fields.unnamed.first().unwrap().ty),
    Fields::Named(fields) if fields.named.len() == 1 => Ok(&fields.named.first().unwrap().ty),
    _ => Err(syn::Error::new_spanned(
      variant,
      "RouterMatch variants must have exactly one field containing a RouterData or RouteMatcher type",
    )),
  }
}

/// 自动从路由结构体的 RouterData trait 实现中获取 pattern
/// 不再支持手动指定 #[route] 或 #[route_prefix] 属性，完全依赖自动提取
fn extract_route_prefix(variant: &Variant) -> syn::Result<Option<TokenStream>> {
  // 直接从路由结构体的 RouterData trait 获取 pattern
  let route_type = extract_route_type(variant)?;
  Ok(Some(quote! { <#route_type as ::ruled_router::traits::RouterData>::pattern() }))
}

/// 生成 try_parse 方法的实现
/// 这个实现会根据 route_prefix 属性进行前缀匹配，然后尝试解析
fn generate_try_parse_impl(variants: &[&Variant]) -> syn::Result<TokenStream> {
  let mut match_arms = Vec::new();

  for variant in variants {
    let variant_name = &variant.ident;
    let route_type = extract_route_type(variant)?;
    let route_prefix = extract_route_prefix(variant)?;

    let match_arm = if let Some(prefix_expr) = route_prefix {
      // 如果有 route_prefix 或 route 属性，先检查前缀匹配，然后解析
      quote! {
        {
          let prefix = #prefix_expr;
          if path.starts_with(prefix) {
            // 分离路径和查询参数
            let (path_part, query_part) = ::ruled_router::utils::split_path_query(path);

            if path_part.starts_with(prefix) {
              // 构造完整的路径用于解析（前缀 + 查询参数）
              let full_path = if let Some(query) = query_part {
                format!("{}?{}", prefix, query)
              } else {
                prefix.to_string()
              };

              // 尝试使用 parse_with_sub 进行递归解析
              if let Ok((route, sub_router_state)) = <#route_type as ::ruled_router::traits::RouterData>::parse_with_sub(path) {
                // 无论是否有子路由，都直接返回解析结果
                // 子路由信息已经包含在 parse_with_sub 的结果中
                return Ok(Self::#variant_name(route));
              }
              // 如果递归解析失败，回退到普通解析
              if let Ok(route) = <#route_type as ::ruled_router::traits::RouterData>::parse(&full_path) {
                return Ok(Self::#variant_name(route));
              }
            }
          }
        }
      }
    } else {
      // 这个分支现在不会被执行，因为我们总是返回 Some
      quote! {
        // 尝试使用 parse_with_sub 进行递归解析
          if let Ok((route, sub_router_state)) = <#route_type as ::ruled_router::traits::RouterData>::parse_with_sub(path) {
            // 无论是否有子路由，都直接返回解析结果
            // 子路由信息已经包含在 parse_with_sub 的结果中
            return Ok(Self::#variant_name(route));
          }
        // 如果递归解析失败，回退到普通解析
        if let Ok(route) = <#route_type as ::ruled_router::traits::RouterData>::parse(path) {
          return Ok(Self::#variant_name(route));
        }
      }
    };
    match_arms.push(match_arm);
  }

  Ok(quote! {
    fn try_parse(path: &str) -> Result<Self, ::ruled_router::error::ParseError> {
      #(#match_arms)*
      Err(::ruled_router::error::ParseError::invalid_path(
        format!("No matching route found for path: {}", path)
      ))
    }
  })
}

/// 生成 format 方法的实现
fn generate_format_impl(variants: &[&Variant]) -> TokenStream {
  let mut match_arms = Vec::new();

  for variant in variants {
    let variant_name = &variant.ident;

    let match_arm = quote! {
      Self::#variant_name(route) => route.format(),
    };
    match_arms.push(match_arm);
  }

  quote! {
    fn format(&self) -> String {
      match self {
        #(#match_arms)*
      }
    }
  }
}

/// 生成 patterns 方法的实现
/// 这个实现假设所有变体都实现了 Router trait
/// 生成 patterns 方法的实现
fn generate_patterns_impl(variants: &[&Variant]) -> syn::Result<TokenStream> {
  let mut pattern_calls = Vec::new();

  for variant in variants {
    let route_type = extract_route_type(variant)?;

    let pattern_call = quote! {
      <#route_type as ::ruled_router::traits::RouterData>::pattern()
    };
    pattern_calls.push(pattern_call);
  }

  Ok(quote! {
    fn patterns() -> Vec<&'static str> {
      vec![
        #(#pattern_calls,)*
      ]
    }
  })
}

/// 提取 enum 级别的 route_prefix 属性
fn extract_enum_route_prefix(input: &DeriveInput) -> syn::Result<Option<String>> {
  for attr in &input.attrs {
    if attr.path().is_ident("route_prefix") {
      if let Ok(lit_str) = attr.parse_args::<syn::LitStr>() {
        return Ok(Some(lit_str.value()));
      }
    }
  }
  Ok(None)
}

/// 生成 try_parse_with_remaining 方法的实现
fn generate_try_parse_with_remaining_impl(input: &DeriveInput, variants: &[&Variant]) -> syn::Result<TokenStream> {
  let enum_route_prefix = extract_enum_route_prefix(input)?;
  let mut match_arms = Vec::new();

  for variant in variants {
    let variant_name = &variant.ident;
    let route_type = extract_route_type(variant)?;
    let route_prefix = extract_route_prefix(variant)?;

    let match_arm = if let Some(prefix) = route_prefix {
      if let Some(enum_prefix) = &enum_route_prefix {
        // 有 enum 级别的 route_prefix，先检查 enum_prefix，然后用剩余路径解析子路由
        quote! {
          if path.starts_with(#enum_prefix) && path.len() > #enum_prefix.len() {
            let remaining_after_enum_prefix = &path[#enum_prefix.len()..];

            // 检查剩余路径是否匹配变体的 route
            if remaining_after_enum_prefix.starts_with(#prefix) {
              // 计算子路由 pattern 应该消耗的路径长度
              let parser = ::ruled_router::parser::PathParser::new(<#route_type as ::ruled_router::traits::RouterData>::pattern())?;
              if let Ok(consumed) = parser.consumed_length(remaining_after_enum_prefix) {
                let route_path = &remaining_after_enum_prefix[..consumed];
                let final_remaining_path = &remaining_after_enum_prefix[consumed..];

                // 尝试解析匹配的路径部分
                if let Ok((route, _)) = <#route_type as ::ruled_router::traits::RouterData>::parse_with_sub(route_path) {
                  return Ok((Self::#variant_name(route), final_remaining_path));
                }
              }
            }
          }
        }
      } else {
        // 没有 enum 级别的 route_prefix，variant 的 route 属性就是完整路径
        quote! {
          if path.starts_with(#prefix) {
            // 计算路由 pattern 应该消耗的路径长度
            let parser = ::ruled_router::parser::PathParser::new(<#route_type as ::ruled_router::traits::RouterData>::pattern())?;
            if let Ok(consumed) = parser.consumed_length(path) {
              let route_path = &path[..consumed];
              let remaining_path = &path[consumed..];

              // 尝试解析匹配的路径部分
              if let Ok((route, _)) = <#route_type as ::ruled_router::traits::RouterData>::parse_with_sub(route_path) {
                return Ok((Self::#variant_name(route), remaining_path));
              }
            }
          }
        }
      }
    } else {
      // 没有 route_prefix 或 route 属性，这种情况不应该存在于 RouterMatch 中
      return Err(syn::Error::new_spanned(
        variant,
        "RouterMatch variants must have either #[route_prefix] or #[route] attribute",
      ));
    };
    match_arms.push(match_arm);
  }

  Ok(quote! {
    fn try_parse_with_remaining(path: &str, _consumed_length: usize) -> Result<(Self, &str), ::ruled_router::error::ParseError> {
      #(#match_arms)*
      Err(::ruled_router::error::ParseError::invalid_path(
        format!("No matching route found for path: {}", path)
      ))
    }
  })
}

/// 生成 ToRouteInfo trait 的实现
fn generate_to_route_info_impl(variants: &[&Variant]) -> syn::Result<TokenStream> {
  let mut match_arms = Vec::new();

  for variant in variants {
    let variant_name = &variant.ident;
    let route_type = extract_route_type(variant)?;

    let match_arm = quote! {
      Self::#variant_name(route) => {
        let sub_route_info = if let Ok((_, sub_route_state)) = <#route_type as ::ruled_router::traits::RouterData>::parse_with_sub(&route.format()) {
          match sub_route_state {
            ::ruled_router::error::RouteState::SubRoute(sub_match) => {
              // 只有当 SubRouterMatch 不是 NoSubRouter 时才调用 to_route_info
              if std::any::type_name::<<#route_type as ::ruled_router::traits::RouterData>::SubRouterMatch>() != std::any::type_name::<::ruled_router::traits::NoSubRouter>() {
                Some(Box::new(sub_match.to_route_info()))
              } else {
                None
              }
            },
            _ => None,
          }
        } else {
          None
        };

        ::ruled_router::traits::RouteInfo {
          pattern: <#route_type as ::ruled_router::traits::RouterData>::pattern(),
          formatted: route.format(),
          sub_route_info,
        }
      }
    };
    match_arms.push(match_arm);
  }

  let expanded_impl = quote! {
    fn to_route_info(&self) -> ::ruled_router::traits::RouteInfo {
      match self {
        #(#match_arms)*
      }
    }
  };

  Ok(expanded_impl)
}

/// 生成 debug_format 方法的实现
fn generate_debug_format_impl(input: &DeriveInput, variants: &[&Variant]) -> syn::Result<TokenStream> {
  let enum_name = &input.ident;
  let mut match_arms = Vec::new();

  for variant in variants {
    let variant_name = &variant.ident;
    let route_type = extract_route_type(variant)?;

    let match_arm = quote! {
      Self::#variant_name(route) => {
        let indent_str = "  ".repeat(indent);
        let mut result = format!("{}{}::{}", indent_str, stringify!(#enum_name), stringify!(#variant_name));

        // 添加精简的路由信息
        result.push_str(&format!("\n{}├─ Pattern: {}", indent_str, <#route_type as ::ruled_router::traits::RouterData>::pattern()));

        // 添加格式化的路径
        let formatted = route.format();
        result.push_str(&format!("\n{}├─ Formatted: {}", indent_str, formatted));

        // 检查是否有查询参数，如果有则显示参数名称
        if formatted.contains('?') {
          let query_keys = <#route_type as ::ruled_router::traits::RouterData>::query_keys();
          if !query_keys.is_empty() {
            let keys_str = query_keys.join(", ");
            result.push_str(&format!("\n{}├─ Query: {}", indent_str, keys_str));
          } else {
            result.push_str(&format!("\n{}├─ Query: ∅", indent_str));
          }
        }

        // 尝试获取子路由信息
        if let Ok((_, sub_route_state)) = <#route_type as ::ruled_router::traits::RouterData>::parse_with_sub(&route.format()) {
          match sub_route_state {
            ::ruled_router::error::RouteState::SubRoute(sub_match) => {
              // 只有当 SubRouterMatch 不是 NoSubRouter 时才调用 debug_format
              if std::any::type_name::<<#route_type as ::ruled_router::traits::RouterData>::SubRouterMatch>() != std::any::type_name::<::ruled_router::traits::NoSubRouter>() {
                result.push_str(&format!("\n{}└─ Sub:", indent_str));
                result.push_str(&format!("\n{}", sub_match.debug_format(indent + 1)));
              } else {
                result.push_str(&format!("\n{}└─ ◉", indent_str));
              }
            }
            _ => {
              result.push_str(&format!("\n{}└─ ◉", indent_str));
            }
          }
        } else {
          result.push_str(&format!("\n{}└─ ◉", indent_str));
        }

        result
      }
    };
    match_arms.push(match_arm);
  }

  Ok(quote! {
    fn debug_format(&self, indent: usize) -> String {
      match self {
        #(#match_arms)*
      }
    }
  })
}

/// 主要的 RouterMatch 派生宏实现
pub fn expand_router_match_derive(input: DeriveInput) -> syn::Result<TokenStream> {
  let name = &input.ident;
  let variants = extract_enum_variants(&input.data)?;

  // 验证所有变体都包含路由类型
  for variant in &variants {
    extract_route_type(variant)?;
  }

  let try_parse_impl = generate_try_parse_impl(&variants)?;
  let format_impl = generate_format_impl(&variants);
  let patterns_impl = generate_patterns_impl(&variants)?;
  let try_parse_with_remaining_impl = generate_try_parse_with_remaining_impl(&input, &variants)?;
  let to_route_info_impl = generate_to_route_info_impl(&variants)?;
  let debug_format_impl = generate_debug_format_impl(&input, &variants)?;

  let expanded = quote! {
    impl ::ruled_router::traits::RouteMatcher for #name {
      #try_parse_impl

      #format_impl

      #patterns_impl

      #try_parse_with_remaining_impl

      #debug_format_impl
    }

    impl ::ruled_router::traits::ToRouteInfo for #name {
      #to_route_info_impl
    }
  };

  Ok(expanded)
}
