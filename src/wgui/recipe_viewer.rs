use std::boxed::Box;
use std::io::{Cursor, Read};

use tiny_http::{
    Response,
    http::{
        header::{self, HeaderMap, HeaderValue},
        status::StatusCode,
    },
};
use uom::{
    fmt::DisplayStyle::{Abbreviation, Description},
    si::rational64::Time,
};

use crate::datatypes::{ingredient::UnitType, recipe::Recipe, step::StepType, unit_helper};

use super::{html_stubs::FOOTER, http_helper};

/// `recipe_viewer` returns the recipe browser page for the web server.
///
/// This is the main page for the Cookbook. This page allows users to select a specific recipe
/// or filter the recipe list via selecting tags.
pub fn recipe_viewer(recipe: Recipe) -> anyhow::Result<Response<Box<dyn Read + Send>>> {
    //let page_len = 25;
    let mut headers = HeaderMap::with_capacity(2);
    headers.append(header::CONTENT_TYPE, HeaderValue::try_from("text/html; charset=utf-8")?);

    //TODO: want to be able to change unit based on configuration options and sigfigs
    let is_new_recipe = recipe == Recipe::new();

    let recipe_name = if is_new_recipe { "New Recipe" } else { recipe.name.as_str() };

    // Create step list
    // TODO: want to be able to change the order of the steps/sort
    let mut step_list = String::new();
    if recipe.steps.is_empty() {
        step_list.push_str("<strong>No Steps in Recipe</strong>\n");
    } else {
        step_list.push_str("<ol>\n");
        for step in &recipe.steps {
            // ingredients
            // equipment
            step_list.push_str("<li>\n");
            step_list.push_str("<section>\n");
            step_list.push_str(&format!("<h3>{}</h3>\n", step.step_type));
            if let Some(time) = step.time_needed {
                //TODO: fix units
                //TODO: print this using approximate_float method
                step_list.push_str(&format!(
                    "<p>Takes: {}</p>\n",
                    unit_helper::time_unit_format_output(time, "min", Abbreviation)
                ));
            }
            if let Some(temp) = step.temperature {
                //TODO: fix units
                //TODO: print this using approximate_float method
                step_list.push_str(&format!(
                    "<p>Cook at: {}</p>\n",
                    unit_helper::temp_interval_unit_format_output(temp, "°C", Abbreviation)
                ));
            }
            if !step.ingredients.is_empty() {
                step_list.push_str("<ul>");
                for ingredient in &step.ingredients {
                    let unit_string = match ingredient.unit_quantity {
                        UnitType::Quantity(q) => q.to_string(),
                        //TODO: need to be able to specify which units to use for mass and volume
                        //TODO: print this using approximate_float method
                        UnitType::Mass { value: m, unit: _ } => unit_helper::mass_unit_format_output(m, "g", Abbreviation),
                        UnitType::Volume { value: v, unit: _ } => unit_helper::volume_unit_format_output(v, "m³", Abbreviation),
                    };
                    step_list.push_str(format!("<li>{}: {}</li>", ingredient.name, unit_string).as_str());
                }
                step_list.push_str("</ul>");
            }
            if !step.equipment.is_empty() {
                step_list.push_str("<ul>");
                for equipment in &step.equipment {
                    step_list.push_str(format!("<li>{}</li>", equipment.name).as_str());
                }
                step_list.push_str("</ul>");
            }
            step_list.push_str(format!("<p>{}</p>", step.instructions).as_str());
            step_list.push_str("</section>");
            step_list.push_str("</li>");
        }
        step_list.push_str("</ol>\n");
    }

    // Create ingredient list
    let mut ingredient_list = String::new();
    if recipe.ingredient_list().is_empty() {
        ingredient_list.push_str("<strong>No Ingredients in Recipe</strong>\n");
    } else {
        ingredient_list.push_str("<ul>\n");
        for ingredient in recipe.ingredient_list() {
            // TODO: description
            let unit_string = match ingredient.unit_quantity {
                UnitType::Quantity(q) => q.to_string(),
                //TODO: need to be able to specify which units to use for mass and volume
                //TODO: print this using approximate_float method
                UnitType::Mass { value: m, unit: _ } => unit_helper::mass_unit_format_output(m, "g", Abbreviation),
                UnitType::Volume { value: v, unit: _ } => unit_helper::volume_unit_format_output(v, "m³", Abbreviation),
            };
            ingredient_list.push_str(format!("<li>{}: {}</li>", ingredient.name, unit_string).as_str());
        }
        ingredient_list.push_str("</ul>\n");
    }

    // Create equipment list
    let mut equipment_list = String::new();
    if recipe.equipment_list().is_empty() {
        equipment_list.push_str("<strong>No Special Equipment needed for Recipe</strong>\n");
    } else {
        equipment_list.push_str("<ul>\n");
        for equipment in recipe.equipment_list() {
            // name
            // description
            // is_owned
            equipment_list.push_str(format!("<li>{}. Owned: {}</li>\n", equipment.name, equipment.is_owned).as_str());
        }
        equipment_list.push_str("</ul>\n");
    }
    let step_type_time_totals = recipe.step_time_totals();
    let prep_time_unit = "min";
    let cook_time_unit = "min";
    let wait_time_unit = "min";
    let other_time_unit = "min";
    let total_time_unit = "min";
    //https://github.com/rust-lang/rust/issues/85846
    let data = format!(
        "{}",
        format_args!(
            include_str!("./recipe_viewer.html"),
            title = "Welcome to CookBookRS",
            footer = FOOTER,
            stylesheet = "",
            favicon = "/favicon.ico",
            recipe_name = http_helper::html_escape(recipe_name),
            description = http_helper::html_escape(&recipe.description.clone().unwrap_or_default()),
            comments = http_helper::html_escape(&recipe.comments.clone().unwrap_or_default()),
            //TODO see if we can detect URLs here and properly format them in html
            source = http_helper::html_escape(&recipe.source),
            author = http_helper::html_escape(&recipe.author),
            amount_made_number = recipe.amount_made.quantity,
            amount_made_units = recipe.amount_made.units,
            //TODO: adjust these units depending on total time
            //TODO: print this using approximate_float method
            prep_time = unit_helper::time_unit_format_output(
                {
                    if let Some(prep_time_total) = step_type_time_totals.get(&StepType::Prep)
                        && let Some(prep_time_value) = *prep_time_total
                    {
                        prep_time_value
                    } else {
                        Time::default()
                    }
                },
                prep_time_unit,
                Description
            ),
            cook_time = unit_helper::time_unit_format_output(
                {
                    if let Some(cook_time_total) = step_type_time_totals.get(&StepType::Cook)
                        && let Some(cook_time_value) = *cook_time_total
                    {
                        cook_time_value
                    } else {
                        Time::default()
                    }
                },
                cook_time_unit,
                Description
            ),
            wait_time = unit_helper::time_unit_format_output(
                {
                    if let Some(wait_time_total) = step_type_time_totals.get(&StepType::Wait)
                        && let Some(wait_time_value) = *wait_time_total
                    {
                        wait_time_value
                    } else {
                        Time::default()
                    }
                },
                wait_time_unit,
                Description
            ),
            other_time = unit_helper::time_unit_format_output(
                {
                    if let Some(other_time_total) = step_type_time_totals.get(&StepType::Other)
                        && let Some(other_time_value) = *other_time_total
                    {
                        other_time_value
                    } else {
                        Time::default()
                    }
                },
                other_time_unit,
                Description
            ),
            total_time = unit_helper::time_unit_format_output(recipe.total_time(), total_time_unit, Description),
            step_list = step_list,
            equipment_list = equipment_list,
            ingredient_list = ingredient_list,
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
