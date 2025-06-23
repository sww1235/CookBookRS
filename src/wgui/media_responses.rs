use std::fs::File;
use std::path::Path;

use log::trace;
use tiny_http::{
    Response,
    http::{
        header::{self, HeaderMap, HeaderValue},
        status::StatusCode,
    },
};

// This is based on the Response<File>::from_file() function from tiny_http. I wanted more control
// and the ability to set the headers before returning the response
// so I reimplemented it here
/// `icon` returns a `[Response]` filled with an icon
pub fn icon(file_path: &Path) -> anyhow::Result<Response<File>> {
    trace!("{file_path:?}");
    let file = File::open(file_path)?;
    let file_size = file.metadata()?.len();
    let mut headers = HeaderMap::with_capacity(3);
    headers.append(header::CONTENT_TYPE, HeaderValue::try_from("image/x-icon")?);
    //TODO: parse file to make sure it is an ICO file.

    Ok(Response::new(
        StatusCode::OK,
        headers,
        file,
        Some(file_size.try_into()?),
        None,
    ))
}
