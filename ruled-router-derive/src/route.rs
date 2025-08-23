//! Implementation of the Router derive macro

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Type};

use crate::extract_route_config;

/// 从路径模式中提取参数名
fn extract_path_params(pattern: &str) -> Vec<String> {
  let mut params = Vec::new();
  let segments: Vec<&str> = pattern.split('/').collect();

  for segment in segments {
    if segment.starts_with(':') {
      // 支持 :param 格式
      params.push(segment.strip_prefix(':').unwrap().to_string());
    } else if segment.starts_with('{') && segment.ends_with('}') {
      // 支持 {param} 格式
      params.push(segment[1..segment.len() - 1].to_string());
    }
  }

  params
}

/// 分离路径字段和查询字段
/// 提取字段信息（包括属性）
fn extract_route_fields(data: &Data) -> syn::Result<Vec<(syn::Ident, Type, bool, bool)>> {
  match data {
    Data::Struct(data_struct) => match &data_struct.fields {
      Fields::Named(fields_named) => {
        let mut field_info = Vec::new();
        for field in &fields_named.named {
          if let Some(ident) = &field.ident {
            let is_query = has_query_attribute(field);
            let is_sub_router = has_sub_router_attribute(field);
            field_info.push((ident.clone(), field.ty.clone(), is_query, is_sub_router));
          }
        }
        Ok(field_info)
      }
      _ => Err(syn::Error::new_spanned(&data_struct.fields, "Only named fields are supported")),
    },
    _ => Err(syn::Error::new(proc_macro2::Span::call_site(), "Only structs are supported")),
  }
}

/// 检查字段是否有 #[query] 属性
fn has_query_attribute(field: &syn::Field) -> bool {
  for attr in &field.attrs {
    if attr.path().is_ident("query") {
      return true;
    }
  }
  false
}

/// 检查字段是否有 #[sub_router] 属性
fn has_sub_router_attribute(field: &syn::Field) -> bool {
  for attr in &field.attrs {
    if attr.path().is_ident("sub_router") {
      return true;
    }
  }
  false
}

// Define type aliases to improve readability and reduce complexity
type RouteField = (syn::Ident, Type, bool, bool);
type ParsedField = (syn::Ident, Type);

fn separate_fields(fields: &[RouteField], param_names: &[String]) -> (Vec<ParsedField>, Vec<ParsedField>) {
  let mut path_fields = Vec::new();
  let mut query_fields = Vec::new();

  for (field_name, field_type, is_query, is_sub_router) in fields {
    let field_name_str = field_name.to_string();

    if *is_query {
      // 这是查询字段（有 #[query] 属性）
      query_fields.push((field_name.clone(), field_type.clone()));
    } else if param_names.contains(&field_name_str) {
      // 这是路径参数
      path_fields.push((field_name.clone(), field_type.clone()));
    } else if *is_sub_router {
      // 忽略子路由字段，它们由 RouterMatch trait 处理
    }
    // 忽略其他字段
  }

  (path_fields, query_fields)
}

/// 生成解析路径字段的代码
fn generate_parse_path_fields(fields: &[(syn::Ident, Type)], param_names: &[String]) -> syn::Result<Vec<TokenStream>> {
  let mut parse_fields = Vec::new();

  for (field_name, field_type) in fields {
    let field_name_str = field_name.to_string();

    if param_names.contains(&field_name_str) {
      // 这是一个路径参数
      let parse_code = quote! {
          #field_name: {
              let param_value = params.get(#field_name_str)
                  .ok_or_else(|| ::ruled_router::error::ParseError::missing_parameter(#field_name_str))?;
              <#field_type as ::ruled_router::traits::FromParam>::from_param(param_value)?
          }
      };
      parse_fields.push(parse_code);
    }
  }

  Ok(parse_fields)
}

/// 生成解析查询字段的代码
fn generate_parse_query_fields(fields: &[(syn::Ident, Type)]) -> Vec<TokenStream> {
  let mut parse_fields = Vec::new();

  for (field_name, field_type) in fields {
    let parse_code = quote! {
        #field_name: <#field_type>::parse(query_part.unwrap_or(""))?
    };
    parse_fields.push(parse_code);
  }

  parse_fields
}

/// 生成格式化路径字段的代码
fn generate_format_path_fields(fields: &[(syn::Ident, Type)]) -> Vec<TokenStream> {
  let mut format_fields = Vec::new();

  for (field_name, _) in fields {
    let field_name_str = field_name.to_string();
    let format_code = quote! {
        params.insert(#field_name_str.to_string(), ::ruled_router::traits::ToParam::to_param(&self.#field_name));
    };
    format_fields.push(format_code);
  }

  format_fields
}

/// 生成格式化查询逻辑的代码
fn generate_format_query_logic(fields: &[(syn::Ident, Type)]) -> TokenStream {
  if !fields.is_empty() {
    let field_name = &fields[0].0;
    return quote! {
        let query_string = self.#field_name.format();
        if !query_string.is_empty() {
            url.push('?');
            url.push_str(&query_string);
        }
    };
  }

  quote! {
      // 没有查询字段
  }
}

/// 查找子路由字段的类型
fn find_sub_router_type(struct_name: &syn::Ident) -> TokenStream {
  let struct_name_str = struct_name.to_string();

  // 根据结构体名称推断对应的 RouterMatch 类型
  let sub_router_match_name = if struct_name_str.ends_with("CategoryRoute") {
    let prefix = struct_name_str.strip_suffix("CategoryRoute").unwrap();
    format!("{prefix}DetailRouterMatch")
  } else if struct_name_str == "UserModuleRoute" {
    "UserSubRouterMatch".to_string()
  } else if struct_name_str == "ShopModuleRoute" {
    "ShopSubRouterMatch".to_string()
  } else if struct_name_str == "AdminModuleRoute" {
    "AdminSubRouterMatch".to_string()
  } else {
    return quote! { ::ruled_router::traits::NoSubRouter };
  };

  let sub_router_ident = syn::Ident::new(&sub_router_match_name, struct_name.span());
  quote! { #sub_router_ident }
}

/// 生成子路由字段的解析代码
fn generate_parse_sub_router_field(fields: &[RouteField]) -> TokenStream {
  for (field_name, _, _, is_sub_router) in fields {
    if *is_sub_router {
      return quote! {
        #field_name: None,
      };
    }
  }
  quote! {}
}

/// Expand the Router derive macro
pub fn expand_route_derive(input: DeriveInput) -> syn::Result<TokenStream> {
  let struct_name = &input.ident;
  let (pattern, _query_type) = extract_route_config(&input)?;
  let fields = extract_route_fields(&input.data)?;

  // 分析路径模式，提取参数名
  let param_names = extract_path_params(&pattern);

  // 分离路径字段和查询字段
  let (path_fields, query_fields) = separate_fields(&fields, &param_names);

  // 查找子路由字段
  let sub_router_type = find_sub_router_type(struct_name);

  // 生成解析逻辑
  let parse_path_fields = generate_parse_path_fields(&path_fields, &param_names)?;
  let parse_query_fields = generate_parse_query_fields(&query_fields);
  let parse_sub_router_field = generate_parse_sub_router_field(&fields);

  // 生成格式化逻辑
  let format_path_fields = generate_format_path_fields(&path_fields);
  let format_query_logic = generate_format_query_logic(&query_fields);

  let expanded = quote! {
      impl ::ruled_router::traits::Router for #struct_name {
          type SubRouterMatch = #sub_router_type;

          fn parse(path: &str) -> Result<Self, ::ruled_router::error::ParseError> {
              let (path_part, query_part) = ::ruled_router::utils::split_path_query(path);
              let parser = ::ruled_router::parser::PathParser::new(#pattern)?;
              let params = parser.match_path(path_part)?;

              // 解析查询参数
              let query_map = if let Some(query_str) = query_part {
                  ::ruled_router::utils::parse_query_string(query_str)?
              } else {
                  ::std::collections::HashMap::new()
              };

              Ok(Self {
                  #(#parse_path_fields,)*
                  #(#parse_query_fields,)*
                  #parse_sub_router_field
              })
          }

          fn parse_with_sub(path: &str) -> Result<(Self, Option<Self::SubRouterMatch>), ::ruled_router::error::ParseError> {
              let (path_part, query_part) = ::ruled_router::utils::split_path_query(path);
              let parser = ::ruled_router::parser::PathParser::new(#pattern)?;

              // 计算当前模式应该消费的路径长度
              let consumed = parser.consumed_length(path_part)?;

              // 只解析当前路由模式匹配的部分
              let current_path_part = &path_part[..consumed.min(path_part.len())];

              // 尝试匹配当前路由的模式
              let params = match parser.match_path(current_path_part) {
                  Ok(params) => params,
                  Err(_) => {
                      // 如果无法匹配，尝试用完整路径解析（向后兼容）
                      return Self::parse(path).map(|router| (router, None));
                  }
              };

              // 解析查询参数
              let query_map = if let Some(query_str) = query_part {
                  ::ruled_router::utils::parse_query_string(query_str)?
              } else {
                  ::std::collections::HashMap::new()
              };

              let router = Self {
                  #(#parse_path_fields,)*
                  #(#parse_query_fields,)*
                  #parse_sub_router_field
              };

              // 尝试解析子路由
              let remaining_path = &path[consumed..];
              let sub_router = if !remaining_path.is_empty() {
                  Self::SubRouterMatch::try_parse(remaining_path).ok()
              } else {
                  None
              };

              Ok((router, sub_router))
          }

          fn format(&self) -> String {
              let mut params = ::std::collections::HashMap::new();
              #(#format_path_fields)*

              let formatter = ::ruled_router::formatter::PathFormatter::new(#pattern).unwrap();
              let mut url = formatter.format(&params).unwrap();

              #format_query_logic

              url
          }

          fn pattern() -> &'static str {
              #pattern
          }
      }

      impl ::ruled_router::traits::ToRouteInfo for #struct_name {
          fn to_route_info(&self) -> ::ruled_router::traits::RouteInfo {
              let sub_route_info = if let Ok((_, sub_match)) = Self::parse_with_sub(&self.format()) {
                  sub_match.map(|sub| Box::new(sub.to_route_info()))
              } else {
                  None
              };

              ::ruled_router::traits::RouteInfo {
                  pattern: Self::pattern(),
                  formatted: self.format(),
                  sub_route_info,
              }
          }
      }
  };

  Ok(expanded)
}
