use spin_sdk::http::Request;
use spin_sdk::http_service;

spin_sdk::dependencies!();

#[http_service]
async fn handle(_req: Request) -> &'static str {
    "hello, world!"
}
