use super::{equipment::Equipment, filetypes, ingredient::Ingredient};

use cookbook_macros::{StatefulWidgetRef, WidgetRef};

use std::fmt;
use std::num::Wrapping;

use dimensioned::ucum;
use num_derive::{FromPrimitive, ToPrimitive};
use ratatui::{style::Stylize, widgets::Widget};

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
    pub editing_selected_field: Option<StepFields>,
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

impl From<filetypes::Step> for Step {
    fn from(input: filetypes::Step) -> Self {
        Self {
            id: input.id,
            time_needed: input.time_needed.map(|tn| tn * ucum::S),
            temperature: input.temperature.map(|t| t * ucum::K),
            instructions: input.instructions,
            ingredients: input.ingredients.into_iter().map(Into::into).collect(),
            equipment: input.equipment.into_iter().map(Into::into).collect(),
            step_type: input.step_type.into(),
        }
    }
}

impl From<filetypes::StepType> for StepType {
    fn from(input: filetypes::StepType) -> Self {
        match input {
            filetypes::StepType::Prep => Self::Prep,
            filetypes::StepType::Cook => Self::Cook,
            filetypes::StepType::Wait => Self::Wait,
            filetypes::StepType::Other => Self::Other,
        }
    }
}
