use std::ops::{Add, AddAssign};

#[cfg(feature = "tui")]
use num_derive::{FromPrimitive, ToPrimitive};
use num_rational::Rational64;
#[cfg(feature = "tui")]
use ranged_wrapping::RangedWrapping;
#[cfg(feature = "tui")]
use ratatui::{style::Stylize, widgets::Widget};
use serde::Serialize;
use uom::si::rational64::{Mass, Volume};
use uuid::Uuid;

#[cfg(feature = "tui")]
use cookbook_macros::{StatefulWidgetRef, WidgetRef};

use super::{filetypes, unit_helper};

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
#[cfg_attr(feature = "tui", derive(StatefulWidgetRef, WidgetRef), cookbook(state_struct = "State"))]
#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Hash)]
pub struct Ingredient {
    /// database ID
    #[cfg_attr(feature = "tui", cookbook(skip))]
    pub id: Uuid,
    /// ingredient short name
    #[cfg_attr(feature = "tui", cookbook(display_order = 0))]
    #[cfg_attr(feature = "tui", cookbook(constraint_type = "Length"))]
    #[cfg_attr(feature = "tui", cookbook(constraint_value = 3))]
    pub name: String,
    /// optional description
    #[cfg_attr(feature = "tui", cookbook(display_order = 1))]
    #[cfg_attr(feature = "tui", cookbook(constraint_type = "Min"))]
    #[cfg_attr(feature = "tui", cookbook(constraint_value = 7))]
    pub description: Option<String>,
    /// Unit and quantity of ingredient
    #[cfg_attr(feature = "tui", cookbook(skip))] //TODO: unit quantity stuff
    pub unit_quantity: UnitType,
    //TODO: inventory reference
}

/// `UnitType` handles different unit types for an ingredient and allows flexibility rather than
/// needing to have 1 ingredient type per unit type
#[non_exhaustive]
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Hash)]
pub enum UnitType {
    /// Represents a count or physical quantity of an `Ingredient`:
    /// Ex: 30 chocolate chips, 5 bananas, 10 carrots etc.
    Quantity(Rational64),
    /// Mass of an `Ingredient`
    Mass { value: Mass, unit: String },
    /// Volume of an `Ingredent`
    Volume { value: Volume, unit: String },
}

/// `State` contains the state of the Ingredient widget
#[cfg(feature = "tui")]
#[derive(Debug)]
pub struct State {
    /// which field is selected in the Ingredient widget display
    pub selected_field: RangedWrapping<usize>,
    /// which field is being edited, if any
    pub editing_selected_field: Option<IngredientFields>,
}
#[cfg(feature = "tui")]
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

    //TODO: decide if adding two UnitTypes with different unit's is acceptable
    #[expect(clippy::arithmetic_side_effects)] //TODO: fix this
    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Self::Quantity(l), Self::Quantity(r)) => Self::Quantity(l + r),
            (Self::Mass { value: l, unit: lu }, Self::Mass { value: r, unit: ru }) => {
                let value = l + r;

                if lu != ru {
                    panic!("attempted to add two unit types together with different file units")
                }

                Self::Mass { value, unit: lu }
            }

            (Self::Volume { value: l, unit: lu }, Self::Volume { value: r, unit: ru }) => {
                let value = l + r;

                if lu != ru {
                    panic!("attempted to add two unit types together with different file units")
                }

                Self::Volume { value, unit: lu }
            }

            _ => panic!("Attempted to add different unit types together. This should not have happened"),
        }
    }
}
impl AddAssign for UnitType {
    #[expect(clippy::arithmetic_side_effects)] //TODO: fix this
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}
impl Default for UnitType {
    fn default() -> Self {
        Self::Quantity(Rational64::default())
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
            filetypes::UnitType::Mass { value: m, unit: u } => Self::Mass {
                value: unit_helper::mass_unit_parser(m, u.as_str()),
                unit: u,
            },
            filetypes::UnitType::Volume { value: v, unit: u } => Self::Volume {
                value: unit_helper::volume_unit_parser(v, u.as_str()),
                unit: u,
            },
        }
    }
}
