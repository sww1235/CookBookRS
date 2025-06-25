use std::boxed::Box;
use std::io::{Cursor, Read};

use tiny_http::{
    Response,
    http::{
        header::{self, HeaderMap, HeaderValue},
        status::StatusCode,
    },
};

use crate::datatypes::recipe::Recipe;

use super::{html_stubs::FOOTER, http_helper};

/// `recipe_editor` returns the recipe browser page for the web server.
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
    let mut step_list = String::new();
    if recipe.steps.is_empty() {
        step_list.push_str("<option value=\"-1\">No Steps in Recipe</option>\n");
    } else {
        for (i, step) in recipe.steps.iter().enumerate() {
            let step_type = step.step_type;
            step_list.push_str(format!("<option value=\"{i}\">{i}: {step_type}</option>\n").as_str());
        }
    }

    //https://github.com/rust-lang/rust/issues/85846
    let data = format!(
        "{}",
        format_args!(
            include_str!("./recipe_editor.html"),
            title = "Welcome to CookBookRS",
            footer = FOOTER,
            stylesheet = "",
            favicon = "/favicon.ico",
            recipe_id = recipe.id,
            recipe_name = http_helper::html_escape(recipe_name),
            edit_name = http_helper::html_escape(edit_name),
            description = http_helper::html_escape(&recipe.description.unwrap_or_default()),
            comments = http_helper::html_escape(&recipe.comments.unwrap_or_default()),
            source = http_helper::html_escape(&recipe.source),
            author = http_helper::html_escape(&recipe.author),
            amount_made_number = recipe.amount_made.quantity,
            amount_made_units = recipe.amount_made.units,
            num_steps = recipe.steps.len(),
            step_list = step_list,
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
