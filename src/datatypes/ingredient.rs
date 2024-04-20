use dimensioned::ucum;
use ratatui::{style::Stylize, widgets::Widget};

use cookbook_macros::{StatefulWidgetRef, WidgetRef};

use std::num::Wrapping;
use std::ops::{Add, AddAssign};

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
#[derive(Default, Debug, Clone, PartialEq, StatefulWidgetRef, WidgetRef)]
#[cookbook(state_struct = "IngredientState")]
pub struct Ingredient {
    /// database ID
    #[cookbook(skip)]
    pub id: u64,
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
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnitType {
    /// Represents a count or physical quantity of an `Ingredient`:
    /// Ex: 30 chocolate chips, 5 bananas, 10 carrots etc.
    Quantity(f64),
    /// Mass of an `Ingredient`
    Mass(ucum::Gram<f64>),
    /// Volume of an `Ingredent`
    Volume(ucum::Meter3<f64>),
}

/// [`IngredientState`]
#[derive(Debug, Default)]
#[allow(clippy::module_name_repetitions, missing_docs)]
pub struct IngredientState {
    //TODO: selected field, etc
    pub selected_field: Wrapping<usize>,
    pub num_fields: usize,
}

impl Add for UnitType {
    type Output = Self;

    #[allow(clippy::arithmetic_side_effects)] //TODO: fix this
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
    #[allow(clippy::arithmetic_side_effects)] //TODO: fix this
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}
impl Default for UnitType {
    fn default() -> Self {
        Self::Quantity(0.0_f64)
    }
}
