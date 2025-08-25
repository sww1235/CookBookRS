/// cooking equipment
pub mod equipment;

/// recipes
pub mod recipe;

/// internal ingredient representation
pub mod ingredient;

/// steps in a recipe
pub mod step;

/// string tags for recipes
pub mod tag;

/// intermediate structs to help with serialization/deserialization of units
mod filetypes;

/// functions to help work around issues with uom crate
/// not easily supporting selectable input and output units
pub mod unit_helper;
