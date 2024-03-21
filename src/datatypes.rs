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
//TODO: fix this visibility
pub(crate) mod filetypes;
