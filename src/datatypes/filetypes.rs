use dimensioned::{ucum, Dimensionless};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{equipment, ingredient, recipe, step};

/// `Recipe` represents one recipe from start to finish
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    /// database ID
    pub id: Option<Uuid>,
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
    pub id: Uuid,
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
    pub id: Uuid,
    /// ingredient short name
    pub name: String,
    /// optional description
    pub description: Option<String>,
    /// quantity of ingredient
    pub unit_quantity: UnitType,
    //TODO: inventory reference
}

/// `UnitType` handles different unit types for an ingredient and allows flexibility rather than
/// needing to have 1 ingredient type per unit type
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum UnitType {
    /// Represents a count or physical quantity of an `Ingredient`:
    /// Ex: 30 chocolate chips, 5 bananas, 10 carrots etc.
    Quantity(f64),
    /// Mass of an `Ingredient`, specified in grams
    Mass(f64),
    /// Volume of an `Ingredent`, specified in m^3
    Volume(f64),
}

impl Default for UnitType {
    fn default() -> Self {
        Self::Quantity(0.0_f64)
    }
}
/// `Step` represents a discrete step within a recipe
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Step {
    /// database ID
    pub id: Uuid,
    /// time needed to perform this step in the recipe
    /// Optional for informational steps, or steps that
    /// don't traditionally have durations associated
    /// Specified in seconds
    pub time_needed: Option<f64>,
    /// cook temperature. Optional for steps that don't involve temperature or cooking
    /// Specified in K
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

impl From<recipe::Recipe> for Recipe {
    fn from(input: recipe::Recipe) -> Self {
        Self {
            id: input.id,
            name: input.name,
            description: input.description,
            comments: input.comments,
            source: input.source,
            author: input.author,
            amount_made: input.amount_made.quantity,
            amount_made_units: input.amount_made.units,
            steps: input.steps.into_iter().map(Into::into).collect(),
            tags: input.tags,
        }
    }
}

impl From<step::Step> for Step {
    fn from(input: step::Step) -> Self {
        Self {
            id: input.id,
            time_needed: input.time_needed.map(|tn| *(tn / ucum::S).value()),
            temperature: input.temperature.map(|t| *(t / ucum::K).value()),
            instructions: input.instructions,
            ingredients: input.ingredients.into_iter().map(Into::into).collect(),
            equipment: input.equipment.into_iter().map(Into::into).collect(),
            step_type: input.step_type.into(),
        }
    }
}

impl From<step::StepType> for StepType {
    fn from(input: step::StepType) -> Self {
        match input {
            step::StepType::Prep => Self::Prep,
            step::StepType::Cook => Self::Cook,
            step::StepType::Wait => Self::Wait,
            step::StepType::Other => Self::Other,
        }
    }
}

impl From<equipment::Equipment> for Equipment {
    fn from(input: equipment::Equipment) -> Self {
        Self {
            id: input.id,
            name: input.name,
            description: input.description,
            is_owned: input.is_owned,
        }
    }
}

impl From<ingredient::Ingredient> for Ingredient {
    fn from(input: ingredient::Ingredient) -> Self {
        Self {
            id: input.id,
            name: input.name,
            description: input.description,
            unit_quantity: input.unit_quantity.into(),
        }
    }
}

impl From<ingredient::UnitType> for UnitType {
    fn from(input: ingredient::UnitType) -> Self {
        match input {
            ingredient::UnitType::Quantity(q) => Self::Quantity(q),
            ingredient::UnitType::Mass(m) => Self::Mass(*(m / ucum::G).value()),
            ingredient::UnitType::Volume(v) => Self::Volume(*(v / ucum::M3).value()),
        }
    }
}
