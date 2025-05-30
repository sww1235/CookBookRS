use std::io::Empty;

use tiny_http::{http::status::StatusCode, Response};

pub fn not_found() -> Response<Empty> {
    Response::empty(StatusCode::NOT_FOUND)
}
