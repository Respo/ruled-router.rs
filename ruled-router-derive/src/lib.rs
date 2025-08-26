//! Procedural macros for ruled-router
//!
//! This crate provides derive macros for automatically implementing
//! the RouterData and Query traits.

use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Lit, Meta};

mod query;
mod querystring;
mod route;
mod router_match;

use query::expand_query_derive;
use querystring::expand_querystring_derive;
use route::expand_route_derive;
use router_match::expand_router_match_derive;

/// sub_router attribute macro
#[proc_macro_attribute]
pub fn sub_router(_args: TokenStream, input: TokenStream) -> TokenStream {
  input
}

/// Derive macro for implementing the RouterData trait
///
/// **Note**: `RouterData` is used for individual route definitions and cannot be used
/// as a top-level router. For top-level routing, use `RouterMatch` with an enum structure
/// that contains multiple `RouterData` implementations.
///
/// # Example
///
/// ```rust
/// use ruled_router_derive::RouterData;
/// use ruled_router::traits::{RouterData, RouteMatcher};
///
/// // Individual route - cannot be used as top-level router
/// #[derive(RouterData)]
/// #[router(pattern = "/users/:id")]
/// struct UserRoute {
///     id: u32,
/// }
///
/// // For top-level routing, use RouterMatch with an enum:
/// // #[derive(RouterMatch)]
/// // enum AppRouter {
/// //     User(UserRoute),
/// //     // ... other routes
/// // }
/// ```
#[proc_macro_derive(RouterData, attributes(router, query, sub_router))]
pub fn derive_router(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  expand_route_derive(input).unwrap_or_else(syn::Error::into_compile_error).into()
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
  expand_query_derive(input).unwrap_or_else(syn::Error::into_compile_error).into()
}

/// Derive macro for implementing querystring parsing and formatting
///
/// This macro automatically implements parsing from and formatting to
/// query string format for structs.
///
/// # Example
///
/// ```rust
/// use ruled_router_derive::QueryString;
///
/// #[derive(QueryString)]
/// struct UserQuery {
///     tab: Option<String>,
///     active: Option<bool>,
/// }
/// ```
#[proc_macro_derive(QueryString)]
pub fn derive_querystring(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  expand_querystring_derive(input)
    .unwrap_or_else(syn::Error::into_compile_error)
    .into()
}

/// Derive macro for implementing the RouterMatch trait
///
/// **This is the recommended approach for top-level routing.** RouterMatch is designed
/// to handle multiple route types in an enum structure, with automatic prefix extraction
/// from each route's pattern.
///
/// This macro generates implementations for parsing and formatting
/// nested router structures with automatic prefix extraction.
///
/// # Example
///
/// ```rust,ignore
/// use ruled_router_derive::{RouterMatch, RouterData};
/// use ruled_router::traits::{RouterMatch, RouterData};
///
/// // Individual routes
/// #[derive(RouterData)]
/// #[router(pattern = "/users/:id")]
/// struct UserRoute { id: u32 }
///
/// #[derive(RouterData)]  
/// #[router(pattern = "/blog/:slug")]
/// struct BlogRoute { slug: String }
///
/// // Top-level router using RouterMatch
/// #[derive(RouterMatch)]
/// enum AppRouter {
///     User(UserRoute),  // Automatically extracts "/users" prefix
///     Blog(BlogRoute),  // Automatically extracts "/blog" prefix  
///     Api(ApiRoute),
/// }
/// ```
#[proc_macro_derive(RouterMatch)]
pub fn derive_router_match(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  expand_router_match_derive(input)
    .unwrap_or_else(syn::Error::into_compile_error)
    .into()
}

/// Extract route configuration from router attribute
fn extract_route_config(input: &DeriveInput) -> syn::Result<(String, Option<String>)> {
  for attr in &input.attrs {
    if attr.path().is_ident("router") {
      if let Meta::List(meta_list) = &attr.meta {
        let mut pattern = None;
        let mut query_type = None;

        // Parse multiple name-value pairs
        let parser = meta_list.parse_args_with(syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated)?;

        for meta in parser {
          if let Meta::NameValue(name_value) = meta {
            if name_value.path.is_ident("pattern") {
              if let syn::Expr::Lit(expr_lit) = &name_value.value {
                if let Lit::Str(lit_str) = &expr_lit.lit {
                  pattern = Some(lit_str.value());
                }
              }
            } else if name_value.path.is_ident("query") {
              if let syn::Expr::Lit(expr_lit) = &name_value.value {
                if let Lit::Str(lit_str) = &expr_lit.lit {
                  query_type = Some(lit_str.value());
                }
              }
            }
          }
        }

        if let Some(pattern) = pattern {
          return Ok((pattern, query_type));
        }
      }
    }
  }
  Err(syn::Error::new_spanned(input, "Missing #[router(pattern = \"...\")]"))
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
      _ => Err(syn::Error::new_spanned(&data_struct.fields, "Only named fields are supported")),
    },
    _ => Err(syn::Error::new(proc_macro2::Span::call_site(), "Only structs are supported")),
  }
}
