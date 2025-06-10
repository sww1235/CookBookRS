use std::boxed::Box;
use std::collections::HashMap;
use std::io::{Cursor, Read};

use tiny_http::{
    http::{
        header::{self, HeaderMap, HeaderValue},
        status::StatusCode,
    },
    Response,
};
use uuid::Uuid;

use crate::datatypes::{recipe::Recipe, tag::Tag};

use super::html_stubs::FOOTER;

/// `browser` returns the recipe browser page for the web server.
///
/// This is the main page for the Cookbook. This page allows users to select a specific recipe
/// or filter the recipe list via selecting tags.
pub fn browser(recipes: HashMap<Uuid, Recipe>, tags: &[Tag]) -> anyhow::Result<Response<Box<dyn Read + Send>>> {
    let page_len = 25;
    let mut headers = HeaderMap::with_capacity(2);
    headers.append(header::CONTENT_TYPE, HeaderValue::try_from("text/html; charset=utf-8")?);

    let mut recipe_list = String::new();
    let mut tag_list = String::new();

    if recipes.is_empty() {
        recipe_list.push_str("<option value=\"-1\">No Recipes Loaded</option>\n");
    } else {
        let mut recipes_sorted = recipes.values().collect::<Vec<_>>();
        recipes_sorted.sort_unstable_by_key(|k| k.name.clone());
        for recipe in recipes_sorted {
            let recipe_name = recipe.name.clone();
            let recipe_id = if !recipe.id.is_nil() {
                recipe.id
            } else {
                // don't want to list recipes without IDs
                continue;
            };
            recipe_list.push_str(format!("<option value=\"{recipe_id}\">{recipe_name}</option>\n").as_str());
        }
    }

    if tags.is_empty() {
        tag_list.push_str("<option value=\"-1\">No Tags Loaded</option>");
    } else {
        for (i, tag) in tags.iter().enumerate() {
            tag_list.push_str(format!("<option value=\"{i}\">{tag}</option>").as_str());
        }
    }

    let tag_list_size = if tags.len() > page_len {
        page_len
    } else if tags.len() < 2 {
        2
    } else {
        tags.len()
    };
    let recipe_list_size = if recipes.len() > page_len {
        page_len
    } else if tags.len() < 2 {
        2
    } else {
        recipes.len()
    };

    //https://github.com/rust-lang/rust/issues/85846
    let data = format!(
        "{}",
        format_args!(
            include_str!("./browser.html"),
            title = "Welcome to CookBookRS",
            footer = FOOTER,
            stylesheet = "",
            tag_list = tag_list,
            recipe_list = recipe_list,
            tag_list_size = tag_list_size,
            recipe_list_size = recipe_list_size
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
