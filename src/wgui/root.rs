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

/// `webroot` displays the `/` page for the web server.
///
/// It allows users to either view the CookBook or view the ingredient database.
pub fn webroot() -> anyhow::Result<Response<Box<dyn Read + Send>>> {
    let mut headers = HeaderMap::with_capacity(2);
    headers.append(header::CONTENT_TYPE, HeaderValue::try_from("text/html; charset=utf-8")?);
    let mut data = String::new();
    data.push_str(r#"<!doctype html>\n"#);
    data.push_str(r#"<html lang="en-US">"#);
    data.push_str(ROOT_HEADER);
    data.push_str("<body>\n");
    data.push_str(ROOT_FORM);
    data.push_str(FOOTER);
    data.push_str("</body>");
    data.push_str("/html>");
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

pub(crate) const ROOT_HEADER: &str = r#"
    <head>
    <meta charset="utf-8" />
    <title>Welcome to CookBookRS</title>
    </head>
    "#;

pub(crate) const ROOT_FORM: &str = r#"
    <form method="post">
    <button formaction="/database" type="submit">Ingredient Database</button>
    <button autofocus formaction="/cookbook" type="submit">CookbookRS</button>
    </form>
    "#;
