//! Implementation of the Query derive macro

use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Type, GenericArgument, PathArguments, TypePath};

use crate::extract_struct_fields;

/// Expand the Query derive macro
pub fn expand_query_derive(input: DeriveInput) -> syn::Result<TokenStream> {
    let struct_name = &input.ident;
    let fields = extract_struct_fields(&input.data)?;
    
    // 生成解析逻辑
    let parse_fields = generate_parse_fields(&fields)?;
    
    // 生成格式化逻辑
    let format_fields = generate_format_fields(&fields);
    
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
        }
    };
    
    Ok(expanded)
}

/// 生成解析字段的代码
fn generate_parse_fields(fields: &[(syn::Ident, Type)]) -> syn::Result<Vec<TokenStream>> {
    let mut parse_fields = Vec::new();
    
    for (field_name, field_type) in fields {
        let field_name_str = field_name.to_string();
        
        let parse_code = if is_option_type(field_type) {
            // Option<T> 类型使用 get_optional
            quote! {
                #field_name: parser.get_optional(#field_name_str)?
            }
        } else if is_vec_type(field_type) {
            // Vec<T> 类型使用 get_all
            quote! {
                #field_name: parser.get_all(#field_name_str).iter().map(|s| s.to_string()).collect()
            }
        } else {
            // 其他类型使用 get_parsed
            quote! {
                #field_name: parser.get_parsed(#field_name_str)?
            }
        };
        
        parse_fields.push(parse_code);
    }
    
    Ok(parse_fields)
}

/// 生成格式化字段的代码
fn generate_format_fields(fields: &[(syn::Ident, Type)]) -> Vec<TokenStream> {
    let mut format_fields = Vec::new();
    
    for (field_name, field_type) in fields {
        let field_name_str = field_name.to_string();
        
        let format_code = if is_option_type(field_type) {
            // Option<T> 类型
            quote! {
                if let Some(ref value) = self.#field_name {
                    formatter.set(#field_name_str, ::ruled_router::traits::ToParam::to_param(value));
                }
            }
        } else if is_vec_type(field_type) {
            // Vec<T> 类型
            quote! {
                for value in &self.#field_name {
                    formatter.add(#field_name_str, ::ruled_router::traits::ToParam::to_param(value));
                }
            }
        } else {
            // 其他类型
            quote! {
                formatter.set(#field_name_str, ::ruled_router::traits::ToParam::to_param(&self.#field_name));
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