use ruled_router::prelude::*;

#[derive(Debug, PartialEq, Router)]
#[router(pattern = "/users/{id}")]
struct UserRoute {
  id: u32,
}

fn main() {
  println!("Testing Router without Query field...");

  let url = "/users/123";
  match UserRoute::parse(url) {
    Ok(route) => {
      println!("Parsed route: {route:?}");
      println!("Formatted: {}", route.format());
    }
    Err(e) => {
      println!("Parse error: {e:?}");
    }
  }

  // Test with query string (should ignore it)
  let url_with_query = "/users/456?ignored=true";
  match UserRoute::parse(url_with_query) {
    Ok(route) => {
      println!("Parsed route with ignored query: {route:?}");
      println!("Formatted: {}", route.format());
    }
    Err(e) => {
      println!("Parse error: {e:?}");
    }
  }
}
