use std::io::Empty;

use tiny_http::{http::status::StatusCode, Response};

pub fn not_found() -> Response<Empty> {
    Response::empty(StatusCode::NOT_FOUND)
}

pub fn method_not_allowed() -> Response<Empty> {
    Response::empty(StatusCode::METHOD_NOT_ALLOWED)
}

pub fn bad_request() -> Response<Empty> {
    Response::empty(StatusCode::BAD_REQUEST)
}
