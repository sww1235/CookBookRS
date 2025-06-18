use std::io::Empty;

use tiny_http::{
    http::{header, method, status::StatusCode},
    Response,
};

pub fn not_found() -> Response<Empty> {
    Response::empty(StatusCode::NOT_FOUND)
}

pub fn method_not_allowed(allowed_methods: Vec<method::Method>) -> Response<Empty> {
    let mut response = Response::empty(StatusCode::METHOD_NOT_ALLOWED);
    response.add_header(
        header::ALLOW,
        header::HeaderValue::try_from(allowed_methods.iter().map(|i| i.to_string()).collect::<String>())
            .expect("converting HTTP methods to strings failed"),
    );
    response
}

pub fn bad_request() -> Response<Empty> {
    Response::empty(StatusCode::BAD_REQUEST)
}
