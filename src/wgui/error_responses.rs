use std::io::Empty;

use tiny_http::{
    http::{header, method, status::StatusCode},
    Response,
};

pub fn not_found() -> Response<Empty> {
    //TODO: change to custom 404 page
    Response::empty(StatusCode::NOT_FOUND)
}

pub fn method_not_allowed<I: IntoIterator<Item = method::Method>>(allowed_methods: I) -> Response<Empty> {
    let mut response = Response::empty(StatusCode::METHOD_NOT_ALLOWED);
    response.add_header(
        header::ALLOW,
        header::HeaderValue::try_from(allowed_methods.into_iter().map(|i| i.to_string()).collect::<String>())
            .expect("converting HTTP methods to strings failed"),
    );
    response
}

pub fn bad_request() -> Response<Empty> {
    Response::empty(StatusCode::BAD_REQUEST)
}

pub fn locked() -> Response<Empty> {
    Response::empty(StatusCode::LOCKED)
}
