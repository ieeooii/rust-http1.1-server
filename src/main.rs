#![allow(dead_code)]
// use http::Method;
use server::Server;
use website_hanlder::WebsiteHanlder;

mod http;
mod server;
mod website_hanlder;

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHanlder);
}
