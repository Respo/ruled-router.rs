//! Implementation of the Router derive macro

use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Type};

use crate::{extract_route_pattern, extract_struct_fields};

/// Expand the Router derive macro
pub fn expand_route_derive(input: DeriveInput) -> syn::Result<TokenStream> {
    let struct_name = &input.ident;
    let pattern = extract_route_pattern(&input)?;
    let fields = extract_struct_fields(&input.data)?;
    
    // 分析路径模式，提取参数名
    let param_names = extract_path_params(&pattern);
    
    // 生成解析逻辑
    let parse_fields = generate_parse_fields(&fields, &param_names)?;
    
    // 生成格式化逻辑
    let format_fields = generate_format_fields(&fields);
    
    let expanded = quote! {
        impl ::ruled_router::traits::Router for #struct_name {
            fn parse(path: &str) -> Result<Self, ::ruled_router::error::ParseError> {
                let (path_part, _) = ::ruled_router::utils::split_path_query(path);
                let parser = ::ruled_router::parser::PathParser::new(#pattern)?;
                let params = parser.match_path(path_part)?;
                
                Ok(Self {
                    #(#parse_fields),*
                })
            }
            
            fn format(&self) -> String {
                let mut params = ::std::collections::HashMap::new();
                #(#format_fields)*
                
                let formatter = ::ruled_router::formatter::PathFormatter::new(#pattern).unwrap();
                formatter.format(&params).unwrap()
            }
            
            fn pattern() -> &'static str {
                #pattern
            }
        }
    };
    
    Ok(expanded)
}

/// 从路径模式中提取参数名
fn extract_path_params(pattern: &str) -> Vec<String> {
    let mut params = Vec::new();
    let segments: Vec<&str> = pattern.split('/').collect();
    
    for segment in segments {
        if segment.starts_with(':') {
            params.push(segment[1..].to_string());
        }
    }
    
    params
}

/// 生成解析字段的代码
fn generate_parse_fields(
    fields: &[(syn::Ident, Type)],
    param_names: &[String],
) -> syn::Result<Vec<TokenStream>> {
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
        } else {
            // 这不是路径参数，可能是查询参数或其他字段
            // 对于 Router，我们假设所有字段都应该是路径参数
            return Err(syn::Error::new_spanned(
                field_name,
                format!("Field '{}' is not found in route pattern '{}'", field_name_str, param_names.join(", ")),
            ));
        }
    }
    
    Ok(parse_fields)
}

/// 生成格式化字段的代码
fn generate_format_fields(fields: &[(syn::Ident, Type)]) -> Vec<TokenStream> {
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