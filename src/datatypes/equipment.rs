use num_derive::{FromPrimitive, ToPrimitive};
use ranged_wrapping::RangedWrapping;
use ratatui::{style::Stylize, widgets::Widget};

use cookbook_macros::{StatefulWidgetRef, WidgetRef};

use super::filetypes;

/// `Equipment` represents any implement you might use to prepare a recipe,
/// from a stove, to a microwave, to a stand mixer, to a potato peeler
#[derive(Default, Debug, Clone, PartialEq, StatefulWidgetRef, WidgetRef)]
#[cookbook(state_struct = "EquipmentState")]
pub struct Equipment {
    /// database unique ID
    #[cookbook(skip)]
    pub id: u64,
    /// short name of item
    #[cookbook(display_order = 0)]
    #[cookbook(constraint_type = "Length")]
    #[cookbook(constraint_value = 3)]
    pub name: String,
    /// longer description of item
    #[cookbook(display_order = 1)]
    #[cookbook(constraint_type = "Min")]
    #[cookbook(constraint_value = 7)]
    pub description: Option<String>,
    /// If item is owned. Allows filtering out recipes that require equipment you don't own so you
    /// don't get half way through a recipe and realize it needs some specialized piece of
    /// equipment like a melon baller or pineapple corer
    #[cookbook(display_order = 2)]
    #[cookbook(constraint_type = "Length")]
    #[cookbook(constraint_value = 3)]
    pub is_owned: bool,
}

/// [`EquipmentState`]
#[derive(Debug, Default)]
#[allow(clippy::module_name_repetitions, missing_docs)]
pub struct EquipmentState {
    pub selected_field: RangedWrapping<usize, usize>,
    pub num_fields: usize,
    pub editing_selected_field: Option<EquipmentFields>,
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
