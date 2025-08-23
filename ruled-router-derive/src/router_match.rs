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
      "RouterMatch variants must have exactly one field containing a Router or RouteMatcher type",
    )),
  }
}

/// 自动从路由结构体的 Router trait 实现中获取 pattern
/// 不再支持手动指定 #[route] 或 #[route_prefix] 属性，完全依赖自动提取
fn extract_route_prefix(variant: &Variant) -> syn::Result<Option<TokenStream>> {
  // 直接从路由结构体的 Router trait 获取 pattern
  let route_type = extract_route_type(variant)?;
  Ok(Some(quote! { <#route_type as ::ruled_router::traits::Router>::pattern() }))
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
              if let Ok((route, sub_router)) = <#route_type as ::ruled_router::traits::Router>::parse_with_sub(path) {
                // 如果有子路由，尝试创建包含子路由的路由实例
                if let Some(_sub) = sub_router {
                  // 对于有子路由的情况，直接返回解析结果
                  // 子路由信息已经包含在 parse_with_sub 的结果中
                  return Ok(Self::#variant_name(route));
                } else {
                  return Ok(Self::#variant_name(route));
                }
              }
              // 如果递归解析失败，回退到普通解析
              if let Ok(route) = <#route_type as ::ruled_router::traits::Router>::parse(&full_path) {
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
          if let Ok((route, sub_router)) = <#route_type as ::ruled_router::traits::Router>::parse_with_sub(path) {
            // 如果有子路由，尝试创建包含子路由的路由实例
            if let Some(_sub) = sub_router {
              // 对于有子路由的情况，直接返回解析结果
              // 子路由信息已经包含在 parse_with_sub 的结果中
              return Ok(Self::#variant_name(route));
            } else {
              return Ok(Self::#variant_name(route));
            }
          }
        // 如果递归解析失败，回退到普通解析
        if let Ok(route) = <#route_type as ::ruled_router::traits::Router>::parse(path) {
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
      <#route_type as ::ruled_router::traits::Router>::pattern()
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
              let parser = ::ruled_router::parser::PathParser::new(<#route_type as ::ruled_router::traits::Router>::pattern())?;
              if let Ok(consumed) = parser.consumed_length(remaining_after_enum_prefix) {
                let route_path = &remaining_after_enum_prefix[..consumed];
                let final_remaining_path = &remaining_after_enum_prefix[consumed..];

                // 尝试解析匹配的路径部分
                if let Ok((route, _)) = <#route_type as ::ruled_router::traits::Router>::parse_with_sub(route_path) {
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
            let parser = ::ruled_router::parser::PathParser::new(<#route_type as ::ruled_router::traits::Router>::pattern())?;
            if let Ok(consumed) = parser.consumed_length(path) {
              let route_path = &path[..consumed];
              let remaining_path = &path[consumed..];

              // 尝试解析匹配的路径部分
              if let Ok((route, _)) = <#route_type as ::ruled_router::traits::Router>::parse_with_sub(route_path) {
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
        let sub_route_info = if let Ok((_, sub_match)) = #route_type::parse_with_sub(&route.format()) {
          sub_match.map(|sub| Box::new(sub.to_route_info()))
        } else {
          None
        };

        ::ruled_router::traits::RouteInfo {
          pattern: #route_type::pattern(),
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

  let expanded = quote! {
    impl ::ruled_router::traits::RouteMatcher for #name {
      #try_parse_impl

      #format_impl

      #patterns_impl

      #try_parse_with_remaining_impl
    }

    impl ::ruled_router::traits::ToRouteInfo for #name {
      #to_route_info_impl
    }
  };

  Ok(expanded)
}
