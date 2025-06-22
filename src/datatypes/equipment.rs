#[cfg(feature = "tui")]
use num_derive::{FromPrimitive, ToPrimitive};
#[cfg(feature = "tui")]
use ranged_wrapping::RangedWrapping;
#[cfg(feature = "tui")]
use ratatui::{style::Stylize, widgets::Widget};
use serde::Serialize;
use uuid::Uuid;

#[cfg(feature = "tui")]
use cookbook_macros::{StatefulWidgetRef, WidgetRef};

use super::filetypes;

/// `Equipment` represents any implement you might use to prepare a recipe,
/// from a stove, to a microwave, to a stand mixer, to a potato peeler
#[cfg_attr(feature = "tui", derive(StatefulWidgetRef, WidgetRef), cookbook(state_struct = "State"))]
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct Equipment {
    /// database unique ID
    #[cfg_attr(feature = "tui", cookbook(skip))]
    pub id: Uuid,
    /// short name of item
    #[cfg_attr(feature = "tui", cookbook(display_order = 0))]
    #[cfg_attr(feature = "tui", cookbook(constraint_type = "Length"))]
    #[cfg_attr(feature = "tui", cookbook(constraint_value = 3))]
    pub name: String,
    /// longer description of item
    #[cfg_attr(feature = "tui", cookbook(display_order = 1))]
    #[cfg_attr(feature = "tui", cookbook(constraint_type = "Min"))]
    #[cfg_attr(feature = "tui", cookbook(constraint_value = 7))]
    pub description: Option<String>,
    /// If item is owned. Allows filtering out recipes that require equipment you don't own so you
    /// don't get half way through a recipe and realize it needs some specialized piece of
    /// equipment like a melon baller or pineapple corer
    #[cfg_attr(feature = "tui", cookbook(display_order = 2))]
    #[cfg_attr(feature = "tui", cookbook(constraint_type = "Length"))]
    #[cfg_attr(feature = "tui", cookbook(constraint_value = 3))]
    pub is_owned: bool,
}

/// `State` contains the state of the Equipment widget
#[derive(Debug)]
#[cfg(feature = "tui")]
pub struct State {
    /// which field is selected in the Equipment widget display
    pub selected_field: RangedWrapping<usize>,
    /// which field is being edited, if any
    pub editing_selected_field: Option<EquipmentFields>,
}
#[cfg(feature = "tui")]
impl Default for State {
    fn default() -> Self {
        Self {
            selected_field: RangedWrapping {
                value: 0,
                max: Equipment::NUM_FIELDS,
                min: 0,
            },
            editing_selected_field: None,
        }
    }
}

impl From<filetypes::Equipment> for Equipment {
    fn from(input: filetypes::Equipment) -> Self {
        Self {
            id: input.id,
            name: input.name,
            description: input.description,
            is_owned: input.is_owned,
        }
    }
}
