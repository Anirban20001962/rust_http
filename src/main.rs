#![allow(dead_code)]

mod server;
mod http;
mod website_handler;

use server::Server;
use website_handler::WebsiteHandler;
use std::env;

fn main() {
    let public_path = format!("{}/public",env!("CARGO_MANIFEST_DIR"));
    let server = Server::new("127.0.0.1:8000".to_string());
    server.run(WebsiteHandler::new(public_path));
}
