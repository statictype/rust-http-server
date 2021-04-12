use super::http::{Method, Request, Response, StatusCode};
use super::server::Handler;
pub struct WebsiteHandler;
impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(
                    StatusCode::Ok,
                    Some("<h1>Welcome to homepage</h1>".to_string()),
                ),
                "/hello" => Response::new(
                    StatusCode::Ok,
                    Some("<h1>Welcome to another page</h1>".to_string()),
                ),
                _ => Response::new(StatusCode::NotFound, None),
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
