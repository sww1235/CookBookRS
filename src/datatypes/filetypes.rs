use serde::{Deserialize, Serialize};

/// `Recipe` represents one recipe from start to finish
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    /// database ID
    pub id: u64,
    /// short name of recipe
    pub name: String,
    /// optional description
    pub description: Option<String>,
    /// recipe comments
    pub comments: Option<String>,
    /// recipe source
    pub source: String,
    /// recipe author
    pub author: String,
    /// amount made
    pub amount_made: u64,
    /// units for amount made.
    ///
    /// Thse are not type checked at all and are treated as a base quantity internally.
    /// This is just a representation of the units to display.
    /// There may be a future addition that automatically calculates calories, or serving
    /// sizes based on calories.
    pub amount_made_units: String,
    /// list of steps in recipe
    pub steps: Vec<Step>,
    /// tags
    pub tags: Vec<String>,
    //TODO: versions
    //TODO: maybe make comments a bit more formal, want to be able to record when recipe was last
    //made
}
/// `Equipment` represents any implement you might use to prepare a recipe,
/// from a stove, to a microwave, to a stand mixer, to a potato peeler
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Equipment {
    /// database unique ID
    pub id: u64,
    /// short name of item
    pub name: String,
    /// longer description of item
    pub description: Option<String>,
    /// If item is owned. Allows filtering out recipes that require equipment you don't own so you
    /// don't get half way through a recipe and realize it needs some specialized piece of
    /// equipment like a melon baller or pineapple corer
    pub is_owned: bool,
}
/// `Ingredient` is a unique item that represents the quantity of a particular ingredient
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ingredient {
    /// database ID
    pub id: u64,
    /// ingredient short name
    pub name: String,
    /// optional description
    pub description: Option<String>,
    /// quantity of ingredient
    pub quantity: f64,
    //TODO: maybe change this to an enum?
    /// unit of ingredient as a text string
    pub unit: String,
    //TODO: inventory reference
}

/// `Step` represents a discrete step within a recipe
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Step {
    /// database ID
    pub id: u64,
    /// time needed to perform this step in the recipe
    /// Optional for informational steps, or steps that
    /// don't traditionally have durations associated
    pub time_needed: Option<f64>,
    /// cook temperature. Optional for steps that don't involve temperature or cooking
    pub temperature: Option<f64>,
    /// instructions for step
    pub instructions: String,
    /// ingredients used in this step
    pub ingredients: Vec<Ingredient>,
    /// equipment used in this step
    pub equipment: Vec<Equipment>,
    /// Step type
    #[allow(clippy::struct_field_names)]
    pub step_type: StepType,
}

/// `StepType` represents what type of step each step is in a recipe. It is used to bucket times
/// for recipe total duration
#[non_exhaustive]
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Copy, Serialize, Deserialize)]
pub enum StepType {
    /// Prep steps
    Prep,
    /// cook steps
    Cook,
    /// waiting steps
    Wait,
    /// Other steps
    #[default]
    Other,
}
