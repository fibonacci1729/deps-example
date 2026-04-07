use spin_sdk::http::Request;
use spin_sdk::http_service;

use deps::{Person, When, f};
mod deps {
    spin_sdk::dependencies!();
}

#[http_service]
async fn handle(_req: Request) -> String {
    f(
        &[Person { name: "brian".to_owned(), age: 42 }], 
        Some(When { time: 99 }),
    )
}
