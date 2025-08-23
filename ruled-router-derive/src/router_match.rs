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
      "RouterMatch variants must have exactly one field containing a Router or RouterMatch type",
    )),
  }
}

/// 提取变体的 route_prefix 属性
fn extract_route_prefix(variant: &Variant) -> syn::Result<Option<String>> {
  for attr in &variant.attrs {
    if attr.path().is_ident("route_prefix") {
      // 解析 #[route_prefix = "value"] 语法
      if let syn::Meta::NameValue(meta_name_value) = &attr.meta {
        if let syn::Expr::Lit(expr_lit) = &meta_name_value.value {
          if let syn::Lit::Str(lit_str) = &expr_lit.lit {
            return Ok(Some(lit_str.value()));
          }
        }
      }
      return Err(syn::Error::new_spanned(attr, "route_prefix must be a string literal in the format #[route_prefix = \"value\"]"));
    }
  }
  Ok(None)
}

/// 生成 try_parse 方法的实现
/// 这个实现会根据 route_prefix 属性进行前缀匹配，然后尝试解析
fn generate_try_parse_impl(variants: &[&Variant]) -> syn::Result<TokenStream> {
  let mut match_arms = Vec::new();

  for variant in variants {
    let variant_name = &variant.ident;
    let route_type = extract_route_type(variant)?;
    let route_prefix = extract_route_prefix(variant)?;

    let match_arm = if let Some(prefix) = route_prefix {
       // 如果有 route_prefix 属性，先检查前缀匹配
       quote! {
         if path.starts_with(#prefix) {
           return Ok(Self::#variant_name(#route_type {}));
         }
       }
     } else {
      // 没有 route_prefix 属性，尝试直接解析
      quote! {
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

/// 生成 try_parse_with_remaining 方法的实现
fn generate_try_parse_with_remaining_impl(variants: &[&Variant]) -> syn::Result<TokenStream> {
  let mut match_arms = Vec::new();

  for variant in variants {
    let variant_name = &variant.ident;
    let route_type = extract_route_type(variant)?;

    let match_arm = quote! {
      if let Ok((route, remaining)) = <#route_type as ::ruled_router::traits::Router>::parse_with_sub(path) {
        let consumed = path.len() - remaining.map_or(0, |_| 0);
        let remaining_path = if consumed < path.len() {
          &path[consumed..]
        } else {
          ""
        };
        return Ok((Self::#variant_name(route), remaining_path));
      }
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
  let try_parse_with_remaining_impl = generate_try_parse_with_remaining_impl(&variants)?;

  let expanded = quote! {
    impl ::ruled_router::traits::RouterMatch for #name {
      #try_parse_impl

      #format_impl

      #patterns_impl

      #try_parse_with_remaining_impl
    }
  };

  Ok(expanded)
}
