use ruled_router::prelude::*;

#[derive(Debug, PartialEq, Query)]
struct UserQuery {
    page: Option<u32>,
}

#[derive(Debug, PartialEq, Router)]
#[router(pattern = "/users/{id}")]
struct UserRoute {
    id: u32,
    #[query]
    query: UserQuery,
}

fn main() {
    println!("Testing Router with Query field...");
    
    let url = "/users/123?page=2";
    match UserRoute::parse(url) {
        Ok(route) => {
            println!("Parsed route: {route:?}");
            println!("Formatted: {}", route.format());
        }
        Err(e) => {
            println!("Parse error: {e:?}");
        }
    }
}