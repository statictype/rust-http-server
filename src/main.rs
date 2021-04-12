// ! means the attribute will be apply within module
// without ! it applies to the expression that follows
#![allow(dead_code)]
use server::Server;
use std::env;
use website_handler::WebsiteHandler;

mod http;
mod server;
mod website_handler;

fn main() {
    // cargo sets env vars at compile time, like the path of cargo.toml
    let defaul_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    // fallback on default path if no env var was passed
    let public_path = env::var("PUBLIC_PATH").unwrap_or(defaul_path);
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler::new(public_path));
}
