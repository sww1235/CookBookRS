use std::collections::HashMap;

use anyhow::anyhow;
use form_urlencoded::parse;
use tiny_http::{http::header, Request};

/// `get_content_type()` returns the value contained in the first CONTENT_TYPE http header in a ['Request']
pub fn get_content_type(request: &Request) -> Option<String> {
    request
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|x| x.to_str().ok())
        .map(|s| s.to_owned())
}

// Inspired by the fn raw_urlencoded_post_input from Rouille
// https://docs.rs/rouille/latest/src/rouille/input/post.rs.html#676
// This is probably vulnerable to buffer overflows, etc but I can't be arsed to fix that right now.
// TODO: check and see if request.as_reader() is vulnerable to buffer overflow
/// `parse_post_form_data()` parses form data from POST requests if present
pub fn parse_post_form_data(request: &mut Request) -> anyhow::Result<HashMap<String, String>> {
    if get_content_type(request) == Some("application/x-www-form-urlencoded".to_owned()) {
        let mut content = String::new();
        request.as_reader().read_to_string(&mut content)?;
        Ok(parse(content.as_bytes()).into_owned().collect())
    } else if get_content_type(request) == Some("multipart/form-data".to_owned()) {
        Err(anyhow!("Requests of type multipart/form-data are not supported currently"))
    } else {
        Err(anyhow!("No form data found in request"))
    }
}
