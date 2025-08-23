//! Implementation of the Query derive macro

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, GenericArgument, Lit, Meta, PathArguments, Type, TypePath};

/// Expand the Query derive macro
pub fn expand_query_derive(input: DeriveInput) -> syn::Result<TokenStream> {
  let struct_name = &input.ident;
  let fields = extract_query_fields(&input.data)?;

  // 生成解析逻辑
  let parse_fields = generate_parse_fields(&fields)?;

  // 生成格式化逻辑
  let format_fields = generate_format_fields(&fields);

  // 生成 from_query_map 解析逻辑
  let from_query_map_fields = generate_from_query_map_fields(&fields)?;

  let expanded = quote! {
      impl ::ruled_router::traits::Query for #struct_name {
          fn parse(query: &str) -> Result<Self, ::ruled_router::error::ParseError> {
              let parser = ::ruled_router::parser::QueryParser::new(query)?;

              Ok(Self {
                  #(#parse_fields),*
              })
          }

          fn format(&self) -> String {
              let mut formatter = ::ruled_router::formatter::QueryFormatter::new();

              #(#format_fields)*

              formatter.format()
          }

          fn from_query_map(query_map: &std::collections::HashMap<String, Vec<String>>) -> Result<Self, ::ruled_router::error::ParseError> {
              Ok(Self {
                  #(#from_query_map_fields),*
              })
          }

          fn to_query_string(&self) -> String {
              self.format()
          }
      }
  };

  Ok(expanded)
}

/// 字段信息结构
struct FieldInfo {
  name: syn::Ident,
  ty: Type,
  query_name: String,
  default_value: Option<String>,
}

/// 提取查询字段信息（包括属性）
fn extract_query_fields(data: &Data) -> syn::Result<Vec<FieldInfo>> {
  match data {
    Data::Struct(data_struct) => match &data_struct.fields {
      Fields::Named(fields_named) => {
        let mut field_info = Vec::new();
        for field in &fields_named.named {
          if let Some(ident) = &field.ident {
            let (query_name, default_value) = extract_query_attributes(field, ident)?;
            field_info.push(FieldInfo {
              name: ident.clone(),
              ty: field.ty.clone(),
              query_name,
              default_value,
            });
          }
        }
        Ok(field_info)
      }
      _ => Err(syn::Error::new_spanned(&data_struct.fields, "Only named fields are supported")),
    },
    _ => Err(syn::Error::new(proc_macro2::Span::call_site(), "Only structs are supported")),
  }
}

/// 提取字段的查询属性（支持 rename 和 default 属性）
fn extract_query_attributes(field: &syn::Field, default_name: &syn::Ident) -> syn::Result<(String, Option<String>)> {
  let mut query_name = default_name.to_string();
  let mut default_value = None;

  for attr in &field.attrs {
    if attr.path().is_ident("query") {
      if let Meta::List(meta_list) = &attr.meta {
        let parser = meta_list.parse_args_with(syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated)?;

        for meta in parser {
          if let Meta::NameValue(name_value) = meta {
            if name_value.path.is_ident("rename") || name_value.path.is_ident("name") {
              if let syn::Expr::Lit(expr_lit) = &name_value.value {
                if let Lit::Str(lit_str) = &expr_lit.lit {
                  query_name = lit_str.value();
                }
              }
            } else if name_value.path.is_ident("default") {
              if let syn::Expr::Lit(expr_lit) = &name_value.value {
                if let Lit::Str(lit_str) = &expr_lit.lit {
                  default_value = Some(lit_str.value());
                }
              }
            }
          }
        }
      }
    }
  }
  Ok((query_name, default_value))
}

/// 生成解析字段的代码
fn generate_parse_fields(fields: &[FieldInfo]) -> syn::Result<Vec<TokenStream>> {
  let mut parse_fields = Vec::new();

  for field_info in fields {
    let field_name = &field_info.name;
    let field_type = &field_info.ty;
    let query_name = &field_info.query_name;
    let default_value = &field_info.default_value;

    let parse_code = if is_option_type(field_type) {
      // Option<T> 类型使用 get_optional
      quote! {
          #field_name: parser.get_optional(#query_name)?
      }
    } else if is_vec_type(field_type) {
      // Vec<T> 类型使用 get_all
      quote! {
          #field_name: parser.get_all(#query_name).iter().map(|s| s.to_string()).collect()
      }
    } else if let Some(default_val) = default_value {
      // 有默认值的类型，先尝试解析，失败则使用默认值
      quote! {
          #field_name: parser.get_optional(#query_name)?
              .unwrap_or_else(|| #default_val.parse().unwrap())
      }
    } else {
      // 其他类型使用 get_parsed
      quote! {
          #field_name: parser.get_parsed(#query_name)?
      }
    };

    parse_fields.push(parse_code);
  }

  Ok(parse_fields)
}

/// 生成从查询映射解析字段的代码
fn generate_from_query_map_fields(fields: &[FieldInfo]) -> syn::Result<Vec<TokenStream>> {
  let mut parse_fields = Vec::new();

  for field_info in fields {
    let field_name = &field_info.name;
    let field_type = &field_info.ty;
    let query_name = &field_info.query_name;
    let default_value = &field_info.default_value;

    let parse_code = if is_option_type(field_type) {
      // Option<T> 类型
      quote! {
          #field_name: query_map.get(#query_name)
              .and_then(|values| values.first())
              .and_then(|s| s.parse().ok())
      }
    } else if is_vec_type(field_type) {
      // Vec<T> 类型
      quote! {
          #field_name: query_map.get(#query_name)
              .map(|values| values.iter().map(|s| s.to_string()).collect())
              .unwrap_or_default()
      }
    } else if let Some(default_val) = default_value {
      // 有默认值的类型
      quote! {
          #field_name: query_map.get(#query_name)
              .and_then(|values| values.first())
              .map(|s| s.parse())
              .unwrap_or_else(|| #default_val.parse())
              .map_err(|_| ::ruled_router::error::ParseError::type_conversion(format!("Failed to parse parameter: {}", #query_name)))?
      }
    } else {
      // 其他类型
      quote! {
          #field_name: query_map.get(#query_name)
              .and_then(|values| values.first())
              .ok_or_else(|| ::ruled_router::error::ParseError::missing_parameter(#query_name))?
              .parse()
              .map_err(|_| ::ruled_router::error::ParseError::type_conversion(format!("Failed to parse parameter: {}", #query_name)))?
      }
    };

    parse_fields.push(parse_code);
  }

  Ok(parse_fields)
}

/// 生成格式化字段的代码
fn generate_format_fields(fields: &[FieldInfo]) -> Vec<TokenStream> {
  let mut format_fields = Vec::new();

  for field_info in fields {
    let field_name = &field_info.name;
    let field_type = &field_info.ty;
    let query_name = &field_info.query_name;

    let format_code = if is_option_type(field_type) {
      // Option<T> 类型
      quote! {
          if let Some(ref value) = self.#field_name {
              formatter.set(#query_name, ::ruled_router::traits::ToParam::to_param(value));
          }
      }
    } else if is_vec_type(field_type) {
      // Vec<T> 类型
      quote! {
          for value in &self.#field_name {
              formatter.add(#query_name, ::ruled_router::traits::ToParam::to_param(value));
          }
      }
    } else {
      // 其他类型
      quote! {
          formatter.set(#query_name, ::ruled_router::traits::ToParam::to_param(&self.#field_name));
      }
    };

    format_fields.push(format_code);
  }

  format_fields
}

/// 检查类型是否为 Option<T>
fn is_option_type(ty: &Type) -> bool {
  if let Type::Path(TypePath { path, .. }) = ty {
    if let Some(segment) = path.segments.last() {
      return segment.ident == "Option";
    }
  }
  false
}

/// 检查类型是否为 Vec<T>
fn is_vec_type(ty: &Type) -> bool {
  if let Type::Path(TypePath { path, .. }) = ty {
    if let Some(segment) = path.segments.last() {
      return segment.ident == "Vec";
    }
  }
  false
}

/// 提取 Option<T> 或 Vec<T> 中的内部类型 T
#[allow(dead_code)]
fn extract_inner_type(ty: &Type) -> Option<&Type> {
  if let Type::Path(TypePath { path, .. }) = ty {
    if let Some(segment) = path.segments.last() {
      if let PathArguments::AngleBracketed(args) = &segment.arguments {
        if let Some(GenericArgument::Type(inner_ty)) = args.args.first() {
          return Some(inner_ty);
        }
      }
    }
  }
  None
}
