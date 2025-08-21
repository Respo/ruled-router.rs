//! Procedural macros for ruled-router
//!
//! This crate provides derive macros for automatically implementing
//! the Router and Query traits.

use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Lit, Meta};

mod query;
mod route;

use query::expand_query_derive;
use route::expand_route_derive;

/// Derive macro for implementing the Router trait
///
/// # Example
///
/// ```rust
/// use ruled_router_derive::Router;
/// use ruled_router::traits::Router;
///
/// #[derive(Router)]
/// #[route(pattern = "/users/:id")]
/// struct UserRoute {
///     id: u32,
/// }
/// ```
#[proc_macro_derive(Router, attributes(route))]
pub fn derive_router(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_route_derive(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Derive macro for implementing the Query trait
///
/// # Example
///
/// ```rust
/// use ruled_router_derive::Query;
/// use ruled_router::traits::Query;
///
/// #[derive(Query)]
/// struct SearchQuery {
///     q: Option<String>,
///     page: Option<u32>,
/// }
/// ```
#[proc_macro_derive(Query, attributes(query))]
pub fn derive_query(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_query_derive(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Extract the pattern from route attribute
fn extract_route_pattern(input: &DeriveInput) -> syn::Result<String> {
    for attr in &input.attrs {
        if attr.path().is_ident("route") {
            if let Meta::List(meta_list) = &attr.meta {
                let nested = meta_list.parse_args::<Meta>()?;
                if let Meta::NameValue(name_value) = nested {
                    if name_value.path.is_ident("pattern") {
                        if let syn::Expr::Lit(expr_lit) = &name_value.value {
                            if let Lit::Str(lit_str) = &expr_lit.lit {
                                return Ok(lit_str.value());
                            }
                        }
                    }
                }
            }
        }
    }
    Err(syn::Error::new_spanned(
        input,
        "Missing #[route(pattern = \"...\")]",
    ))
}

/// Extract field information from struct
fn extract_struct_fields(data: &Data) -> syn::Result<Vec<(syn::Ident, syn::Type)>> {
    match data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields_named) => {
                let mut field_info = Vec::new();
                for field in &fields_named.named {
                    if let Some(ident) = &field.ident {
                        field_info.push((ident.clone(), field.ty.clone()));
                    }
                }
                Ok(field_info)
            }
            _ => Err(syn::Error::new_spanned(
                &data_struct.fields,
                "Only named fields are supported",
            )),
        },
        _ => Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "Only structs are supported",
        )),
    }
}
