use std::ops::{Add, AddAssign};

use dimensioned::ucum;
use num_derive::{FromPrimitive, ToPrimitive};
use ranged_wrapping::RangedWrapping;
use ratatui::{style::Stylize, widgets::Widget};
use serde::Serialize;
use uuid::Uuid;

use cookbook_macros::{StatefulWidgetRef, WidgetRef};

use super::filetypes;

//let unit_block = Block::default()
//    .borders(Borders::ALL)
//    .style(Style::default())
//    .title("Quantity and units");
//TODO: fix this input, and allow for proper unit/numeric entry and parsing
//let unit_paragraph = Paragraph::new(Text::styled(
//    self.time_needed.unwrap_or_default().to_string(),
//    Style::default().fg(Color::Red),
//))

/// `Ingredient` is a unique item that represents the quantity of a particular ingredient
#[derive(Default, Debug, Clone, PartialEq, StatefulWidgetRef, WidgetRef, Serialize)]
#[cookbook(state_struct = "State")]
pub struct Ingredient {
    /// database ID
    #[cookbook(skip)]
    pub id: Uuid,
    /// ingredient short name
    #[cookbook(display_order = 0)]
    #[cookbook(constraint_type = "Length")]
    #[cookbook(constraint_value = 3)]
    pub name: String,
    /// optional description
    #[cookbook(display_order = 1)]
    #[cookbook(constraint_type = "Min")]
    #[cookbook(constraint_value = 7)]
    pub description: Option<String>,
    /// Unit and quantity of ingredient
    #[cookbook(skip)] //TODO: unit quantity stuff
    pub unit_quantity: UnitType,
    //TODO: inventory reference
}

/// `UnitType` handles different unit types for an ingredient and allows flexibility rather than
/// needing to have 1 ingredient type per unit type
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum UnitType {
    /// Represents a count or physical quantity of an `Ingredient`:
    /// Ex: 30 chocolate chips, 5 bananas, 10 carrots etc.
    Quantity(f64),
    /// Mass of an `Ingredient`
    Mass(ucum::Gram<f64>),
    /// Volume of an `Ingredent`
    Volume(ucum::Meter3<f64>),
}

/// `State` contains the state of the Ingredient widget
#[derive(Debug)]
pub struct State {
    /// which field is selected in the Ingredient widget display
    pub selected_field: RangedWrapping<usize>,
    /// which field is being edited, if any
    pub editing_selected_field: Option<IngredientFields>,
}
impl Default for State {
    fn default() -> Self {
        Self {
            selected_field: RangedWrapping {
                value: 0,
                max: Ingredient::NUM_FIELDS,
                min: 0,
            },
            editing_selected_field: None,
        }
    }
}

impl Add for UnitType {
    type Output = Self;

    #[expect(clippy::arithmetic_side_effects)] //TODO: fix this
    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Self::Quantity(l), Self::Quantity(r)) => Self::Quantity(l + r),
            (Self::Mass(l), Self::Mass(r)) => Self::Mass(l + r),
            (Self::Volume(l), Self::Volume(r)) => Self::Volume(l + r),
            _ => panic!("Attempted to add different unit types together. This should not have happened"),
        }
    }
}
impl AddAssign for UnitType {
    #[expect(clippy::arithmetic_side_effects)] //TODO: fix this
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}
impl Default for UnitType {
    fn default() -> Self {
        Self::Quantity(0.0_f64)
    }
}

impl From<filetypes::Ingredient> for Ingredient {
    fn from(input: filetypes::Ingredient) -> Self {
        Self {
            id: input.id,
            name: input.name,
            description: input.description,
            unit_quantity: input.unit_quantity.into(),
        }
    }
}

impl From<filetypes::UnitType> for UnitType {
    fn from(input: filetypes::UnitType) -> Self {
        match input {
            filetypes::UnitType::Quantity(q) => Self::Quantity(q),
            filetypes::UnitType::Mass(m) => Self::Mass(m * ucum::G),
            filetypes::UnitType::Volume(v) => Self::Volume(v * ucum::M3),
        }
    }
}
