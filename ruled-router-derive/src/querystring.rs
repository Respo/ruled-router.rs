//! Implementation of the QueryString derive macro

use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Type};

use crate::extract_struct_fields;

/// Expand the QueryString derive macro
pub fn expand_querystring_derive(input: DeriveInput) -> syn::Result<TokenStream> {
    let struct_name = &input.ident;
    let fields = extract_struct_fields(&input.data)?;

    // 生成从中间结构解析的代码
    let parse_fields = generate_parse_from_intermediate(&fields);

    // 生成格式化到中间结构的代码
    let format_fields = generate_format_to_intermediate(&fields);

    let expanded = quote! {
        impl #struct_name {
            /// 从中间查询结构解析
            pub fn from_query_map(query_map: &::std::collections::HashMap<String, Vec<String>>) -> Result<Self, ::ruled_router::error::ParseError> {
                Ok(Self {
                    #(#parse_fields),*
                })
            }

            /// 格式化到中间查询结构
            pub fn to_query_map(&self) -> ::std::collections::HashMap<String, Vec<String>> {
                let mut query_map = ::std::collections::HashMap::new();
                #(#format_fields)*
                query_map
            }

            /// 从查询字符串解析
            pub fn from_query_string(query_str: &str) -> Result<Self, ::ruled_router::error::ParseError> {
                let query_map = ::ruled_router::utils::parse_query_string(query_str)?;
                Self::from_query_map(&query_map)
            }

            /// 格式化为查询字符串
            pub fn to_query_string(&self) -> String {
                let query_map = self.to_query_map();
                ::ruled_router::utils::format_query_string(&query_map)
            }
        }
    };

    Ok(expanded)
}

/// 生成从中间结构解析字段的代码
fn generate_parse_from_intermediate(fields: &[(syn::Ident, Type)]) -> Vec<TokenStream> {
    let mut parse_fields = Vec::new();

    for (field_name, field_type) in fields {
        let field_name_str = field_name.to_string();
        
        // 检查字段类型是否是 Option<T> 或 Vec<T>
        let parse_code = if is_option_type(field_type) {
            // Option<T> 类型 - 提取内部类型
            let inner_type = extract_option_inner_type(field_type);
            quote! {
                #field_name: {
                    if let Some(values) = query_map.get(#field_name_str) {
                        if let Some(first_value) = values.first() {
                            Some(<#inner_type as ::ruled_router::traits::FromParam>::from_param(first_value)?)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
            }
        } else if is_vec_type(field_type) {
            // Vec<T> 类型 - 提取内部类型
            let inner_type = extract_vec_inner_type(field_type);
            quote! {
                #field_name: {
                    if let Some(values) = query_map.get(#field_name_str) {
                        let mut result = Vec::new();
                        for value in values {
                            result.push(<#inner_type as ::ruled_router::traits::FromParam>::from_param(value)?);
                        }
                        result
                    } else {
                        Vec::new()
                    }
                }
            }
        } else {
            // 普通类型（必需）
            quote! {
                #field_name: {
                    let values = query_map.get(#field_name_str)
                        .ok_or_else(|| ::ruled_router::error::ParseError::missing_parameter(#field_name_str))?;
                    let first_value = values.first()
                        .ok_or_else(|| ::ruled_router::error::ParseError::missing_parameter(#field_name_str))?;
                    <#field_type as ::ruled_router::traits::FromParam>::from_param(first_value)?
                }
            }
        };
        
        parse_fields.push(parse_code);
    }

    parse_fields
}

/// 生成格式化到中间结构的代码
fn generate_format_to_intermediate(fields: &[(syn::Ident, Type)]) -> Vec<TokenStream> {
    let mut format_fields = Vec::new();

    for (field_name, field_type) in fields {
        let field_name_str = field_name.to_string();
        
        let format_code = if is_option_type(field_type) {
            // Option<T> 类型
            quote! {
                if let Some(ref value) = self.#field_name {
                    query_map.insert(#field_name_str.to_string(), vec![::ruled_router::traits::ToParam::to_param(value)]);
                }
            }
        } else if is_vec_type(field_type) {
            // Vec<T> 类型
            quote! {
                if !self.#field_name.is_empty() {
                    let values: Vec<String> = self.#field_name.iter()
                        .map(|v| ::ruled_router::traits::ToParam::to_param(v))
                        .collect();
                    query_map.insert(#field_name_str.to_string(), values);
                }
            }
        } else {
            // 普通类型
            quote! {
                query_map.insert(#field_name_str.to_string(), vec![::ruled_router::traits::ToParam::to_param(&self.#field_name)]);
            }
        };
        
        format_fields.push(format_code);
    }

    format_fields
}

/// 检查类型是否是 Option<T>
fn is_option_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "Option";
        }
    }
    false
}

/// 检查类型是否是 Vec<T>
fn is_vec_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "Vec";
        }
    }
    false
}

/// 提取 Option<T> 的内部类型 T
fn extract_option_inner_type(ty: &Type) -> &Type {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Option" {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
                        return inner_ty;
                    }
                }
            }
        }
    }
    ty // 如果提取失败，返回原类型
}

/// 提取 Vec<T> 的内部类型 T
fn extract_vec_inner_type(ty: &Type) -> &Type {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Vec" {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
                        return inner_ty;
                    }
                }
            }
        }
    }
    ty // 如果提取失败，返回原类型
}