use super::http::{Request, Response, StatusCode};
use super::server::Handler;

pub struct WebsiteHanlder;

impl Handler for WebsiteHanlder {
    fn handle_request(&mut self, request: &Request) -> Response {
        Response::new(StatusCode::Ok, Some("<h1>TEST</h1>".to_string()))
    }
}
