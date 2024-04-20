use super::{equipment::Equipment, ingredient::Ingredient};

use std::fmt;
use std::num::Wrapping;

use dimensioned::ucum;
use ratatui::{style::Stylize, widgets::Widget};

use cookbook_macros::{StatefulWidgetRef, WidgetRef};

/// `Step` represents a discrete step within a recipe
#[derive(Default, Debug, Clone, PartialEq, StatefulWidgetRef, WidgetRef)]
#[cookbook(state_struct = "StepState")]
pub struct Step {
    /// database ID
    #[cookbook(skip)]
    pub id: u64,
    /// time needed to perform this step in the recipe
    /// Optional for informational steps, or steps that
    /// don't traditionally have durations associated
    #[cookbook(display_order = 0)]
    #[cookbook(constraint_type = "Length")]
    #[cookbook(constraint_value = 3)]
    pub time_needed: Option<ucum::Second<f64>>,
    /// cook temperature. Optional for steps that don't involve temperature or cooking
    #[cookbook(display_order = 1)]
    #[cookbook(constraint_type = "Length")]
    #[cookbook(constraint_value = 3)]
    pub temperature: Option<ucum::Kelvin<f64>>,
    /// instructions for step
    #[cookbook(display_order = 2)]
    #[cookbook(constraint_type = "Min")]
    #[cookbook(constraint_value = 3)]
    pub instructions: String,
    /// ingredients used in this step
    #[cookbook(left_field)]
    #[cookbook(field_title = "Number Of Ingredients")]
    pub ingredients: Vec<Ingredient>,
    /// equipment used in this step
    #[cookbook(right_field)]
    #[cookbook(field_title = "Equipment count")]
    pub equipment: Vec<Equipment>,
    /// Step type
    #[cookbook(display_order = 3)]
    #[cookbook(constraint_type = "Length")]
    #[cookbook(constraint_value = 3)]
    pub step_type: StepType,
}

/// [`StepState`]
#[derive(Default, Debug)]
#[allow(clippy::module_name_repetitions, missing_docs)]
pub struct StepState {
    pub selected_field: Wrapping<usize>,
    pub num_fields: usize,
}

/// `StepType` represents what type of step each step is in a recipe. It is used to bucket times
/// for recipe total duration
#[non_exhaustive]
#[allow(clippy::module_name_repetitions)]
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
            Self::Prep => write!(f, "Prep"),
            Self::Cook => write!(f, "Cook"),
            Self::Wait => write!(f, "Wait"),
            Self::Other => write!(f, "Other"),
        }
    }
}
