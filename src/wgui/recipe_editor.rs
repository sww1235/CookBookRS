use std::boxed::Box;
use std::io::{Cursor, Read};

use tiny_http::{
    http::{
        header::{self, HeaderMap, HeaderValue},
        status::StatusCode,
    },
    Response,
};

use crate::datatypes::recipe::Recipe;

use super::html_stubs::FOOTER;

/// `browser` returns the recipe browser page for the web server.
///
/// This is the main page for the Cookbook. This page allows users to select a specific recipe
/// or filter the recipe list via selecting tags.
pub fn recipe_editor(recipe: Recipe) -> anyhow::Result<Response<Box<dyn Read + Send>>> {
    //let page_len = 25;
    let mut headers = HeaderMap::with_capacity(2);
    headers.append(header::CONTENT_TYPE, HeaderValue::try_from("text/html; charset=utf-8")?);

    let is_new_recipe = recipe == Recipe::new();

    let recipe_name = if is_new_recipe { "New Recipe" } else { recipe.name.as_str() };
    let edit_name = if is_new_recipe { "" } else { recipe.name.as_str() };

    //https://github.com/rust-lang/rust/issues/85846
    let data = format!(
        "{}",
        format_args!(
            include_str!("./recipe_editor.html"),
            title = "Welcome to CookBookRS",
            footer = FOOTER,
            stylesheet = "",
            recipe_name = recipe_name,
            edit_name = edit_name,
            description = recipe.description.unwrap_or_default(),
            comments = recipe.comments.unwrap_or_default(),
            source = recipe.source,
            author = recipe.author,
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
