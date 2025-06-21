use std::boxed::Box;
use std::io::{Cursor, Read};

use tiny_http::{
    http::{
        header::{self, HeaderMap, HeaderValue},
        status::StatusCode,
    },
    Response,
};

use super::html_stubs::FOOTER;

/// `webroot` returns the `/` page for the web server.
///
/// It allows users to either view the CookBook or view the ingredient database.
pub fn webroot() -> anyhow::Result<Response<Box<dyn Read + Send>>> {
    let mut headers = HeaderMap::with_capacity(2);
    headers.append(header::CONTENT_TYPE, HeaderValue::try_from("text/html; charset=utf-8")?);
    //https://github.com/rust-lang/rust/issues/85846
    let data = format!(
        "{}",
        format_args!(
            include_str!("./root.html"),
            title = "Welcome to CookBookRS",
            footer = FOOTER,
            stylesheet = "",
            favicon = "/favicon.ico",
        )
    );
    // Don't fully understand why Box + Cursor, but thats what Rouille used and it seems to work.
    // Also not sure why the response data needs to implement Read but...
    Ok(Response::new(
        StatusCode::OK,
        headers,
        Box::new(Cursor::new(data.clone())),
        Some(data.len()),
        None,
    ))
}
