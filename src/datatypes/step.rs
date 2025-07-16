use std::fmt;

#[cfg(feature = "tui")]
use num_derive::{FromPrimitive, ToPrimitive};
#[cfg(feature = "tui")]
use ratatui::{style::Stylize, widgets::Widget};
use serde::Serialize;
use uom::si::rational64::{TemperatureInterval, Time};
use uuid::Uuid;

#[cfg(feature = "tui")]
use ranged_wrapping::RangedWrapping;

#[cfg(feature = "tui")]
use cookbook_macros::{StatefulWidgetRef, WidgetRef};

use super::{equipment::Equipment, filetypes, ingredient::Ingredient, unit_helper};
#[cfg(feature = "tui")]
use crate::tui::dropdown::{Dropdown, DropdownState};

/// `Step` represents a discrete step within a recipe
#[cfg_attr(feature = "tui", derive(StatefulWidgetRef, WidgetRef), cookbook(state_struct = "State"))]
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct Step {
    /// database ID
    #[cfg_attr(feature = "tui", cookbook(skip))]
    pub id: Option<Uuid>,
    /// time needed to perform this step in the recipe
    /// Optional for informational steps, or steps that
    /// don't traditionally have durations associated
    #[cfg_attr(feature = "tui", cookbook(display_order = 0))]
    #[cfg_attr(feature = "tui", cookbook(constraint_type = "Length"))]
    #[cfg_attr(feature = "tui", cookbook(constraint_value = 3))]
    pub time_needed: Option<Time>,
    /// Units for time_needed.
    #[cfg_attr(feature = "tui", cookbook(skip))]
    pub time_needed_unit: Option<String>,
    /// cook temperature. Optional for steps that don't involve temperature or cooking
    #[cfg_attr(feature = "tui", cookbook(display_order = 1))]
    #[cfg_attr(feature = "tui", cookbook(constraint_type = "Length"))]
    #[cfg_attr(feature = "tui", cookbook(constraint_value = 3))]
    pub temperature: Option<TemperatureInterval>,
    /// Units for temperature.
    #[cfg_attr(feature = "tui", cookbook(skip))]
    pub temperature_unit: Option<String>,
    /// instructions for step
    #[cfg_attr(feature = "tui", cookbook(display_order = 2))]
    #[cfg_attr(feature = "tui", cookbook(constraint_type = "Min"))]
    #[cfg_attr(feature = "tui", cookbook(constraint_value = 3))]
    pub instructions: String,
    /// ingredients used in this step
    #[cfg_attr(feature = "tui", cookbook(left_field))]
    #[cfg_attr(feature = "tui", cookbook(left_field_title = "Number Of Ingredients"))]
    pub ingredients: Vec<Ingredient>,
    /// equipment used in this step
    #[cfg_attr(feature = "tui", cookbook(right_field))]
    #[cfg_attr(feature = "tui", cookbook(right_field_title = "Equipment count"))]
    pub equipment: Vec<Equipment>,
    /// Step type
    #[cfg_attr(feature = "tui", cookbook(display_order = 3))]
    #[cfg_attr(feature = "tui", cookbook(constraint_type = "Length"))]
    #[cfg_attr(feature = "tui", cookbook(constraint_value = 3))]
    #[cfg_attr(feature = "tui", cookbook(display_widget = "Dropdown"))]
    #[cfg_attr(feature = "tui", cookbook(display_widget_state = "dropdown_state"))]
    #[cfg_attr(
        feature = "tui",
        cookbook(display_widget_options(StepType::Prep, StepType::Cook, StepType::Wait, StepType::Other))
    )]
    pub step_type: StepType,
}

/// `State` contains the state of the Step widget
#[derive(Debug)]
#[cfg(feature = "tui")]
pub struct State {
    /// which field is selected in the Step widget display
    pub selected_field: RangedWrapping<usize>,
    /// which field is being edited, if any
    pub editing_selected_field: Option<StepFields>,
    //TODO: may need to change the name of this if adding more dropdowns to Step
    /// State of step_type dropdown
    pub dropdown_state: DropdownState,
}

#[cfg(feature = "tui")]
impl Default for State {
    fn default() -> Self {
        Self {
            selected_field: RangedWrapping {
                value: 0,
                max: Step::NUM_FIELDS,
                min: 0,
            },
            editing_selected_field: None,
            dropdown_state: DropdownState::default(),
        }
    }
}
/// `StepType` represents what type of step each step is in a recipe. It is used to bucket times
/// for recipe total duration
#[non_exhaustive]
#[expect(clippy::module_name_repetitions)]
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Copy, Serialize)]
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
            time_needed: input
                .time_needed
                .map(|x| unit_helper::time_unit_parser(x, &input.time_needed_unit.clone().unwrap_or("placeholder".to_string()))),
            time_needed_unit: input.time_needed_unit,
            temperature: input.temperature.map(|x| {
                unit_helper::temp_interval_unit_parser(x, &input.temperature_unit.clone().unwrap_or("placeholder".to_string()))
            }),
            temperature_unit: input.temperature_unit,
            instructions: input.instructions,
            ingredients: if input.ingredients.is_some() {
                input.ingredients.unwrap().into_iter().map(Into::into).collect()
            } else {
                Vec::new()
            },
            equipment: if input.equipment.is_some() {
                input.equipment.unwrap().into_iter().map(Into::into).collect()
            } else {
                Vec::new()
            },
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
