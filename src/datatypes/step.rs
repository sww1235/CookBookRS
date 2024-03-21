use super::{equipment::Equipment, ingredient::Ingredient};

use std::fmt;

use dimensioned::ucum;
use struct_field_names_as_array::FieldNamesAsSlice;

/// `Step` represents a discrete step within a recipe
#[derive(Default, Debug, Clone, PartialEq, FieldNamesAsSlice)]
pub struct Step {
    /// database ID
    #[field_names_as_slice(skip)]
    pub id: u64,
    /// time needed to perform this step in the recipe
    /// Optional for informational steps, or steps that
    /// don't traditionally have durations associated
    pub time_needed: Option<ucum::Second<f64>>,
    /// cook temperature. Optional for steps that don't involve temperature or cooking
    pub temperature: Option<ucum::Kelvin<f64>>,
    /// instructions for step
    pub instructions: String,
    /// ingredients used in this step
    #[field_names_as_slice(skip)]
    pub ingredients: Vec<Ingredient>,
    /// equipment used in this step
    #[field_names_as_slice(skip)]
    pub equipment: Vec<Equipment>,
    /// Step type
    pub step_type: StepType,
}

/// `StepType` represents what type of step each step is in a recipe. It is used to bucket times
/// for recipe total duration
#[non_exhaustive]
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Copy)]
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

impl fmt::Display for StepType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StepType::Prep => write!(f, "Prep"),
            StepType::Cook => write!(f, "Cook"),
            StepType::Wait => write!(f, "Wait"),
            StepType::Other => write!(f, "Other"),
        }
    }
}
