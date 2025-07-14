use std::ops::{Add, AddAssign};

#[cfg(feature = "tui")]
use num_derive::{FromPrimitive, ToPrimitive};
use num_rational::Rational64;
#[cfg(feature = "tui")]
use ranged_wrapping::RangedWrapping;
#[cfg(feature = "tui")]
use ratatui::{style::Stylize, widgets::Widget};
use serde::Serialize;
use uom::si::{
    mass::{
        centigram, decagram, decigram, gigagram, gram, hectogram, kilogram, megagram, microgram, milligram, nanogram, ounce,
        picogram, pound, teragram,
    },
    rational64::{Mass, Volume},
    volume::{
        acre_foot, barrel, bushel, centiliter, cord, cubic_centimeter, cubic_decameter, cubic_decimeter, cubic_foot,
        cubic_gigameter, cubic_hectometer, cubic_inch, cubic_kilometer, cubic_megameter, cubic_meter, cubic_micrometer,
        cubic_mile, cubic_millimeter, cubic_nanometer, cubic_picometer, cubic_terameter, cubic_yard, cup, decaliter, deciliter,
        fluid_ounce, fluid_ounce_imperial, gallon, gallon_imperial, gigaliter, gill, gill_imperial, hectoliter, kiloliter, liter,
        megaliter, microliter, milliliter, nanoliter, peck, picoliter, pint_dry, pint_liquid, quart_dry, quart_liquid,
        tablespoon, teaspoon, teraliter,
    },
};
use uuid::Uuid;

#[cfg(feature = "tui")]
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
                value: mass_unit_parser(m, u.as_str()),
                unit: u,
            },
            filetypes::UnitType::Volume { value: v, unit: u } => Self::Volume {
                value: volume_unit_parser(v, u.as_str()),
                unit: u,
            },
        }
    }
}

fn mass_unit_parser(value: Rational64, unit_string: &str) -> Mass {
    match unit_string {
        "Tg" => Mass::new::<teragram>(value),
        "Gg" => Mass::new::<gigagram>(value),
        "Mg" => Mass::new::<megagram>(value),
        "kg" => Mass::new::<kilogram>(value),
        "hg" => Mass::new::<hectogram>(value),
        "dag" => Mass::new::<decagram>(value),
        "g" => Mass::new::<gram>(value),
        "dg" => Mass::new::<decigram>(value),
        "cg" => Mass::new::<centigram>(value),
        "mg" => Mass::new::<milligram>(value),
        "µg" => Mass::new::<microgram>(value),
        "ng" => Mass::new::<nanogram>(value),
        "pg" => Mass::new::<picogram>(value),
        "oz" => Mass::new::<ounce>(value),
        "lb" => Mass::new::<pound>(value),
        "placeholder" => panic!("Unit not specified for ingredient mass"),
        x => panic!("{x} not recognized as a supported mass unit abbreviation"),
    }
}

fn volume_unit_parser(value: Rational64, unit_string: &str) -> Volume {
    match unit_string {
        "Tm³" => Volume::new::<cubic_terameter>(value),
        "Gm³" => Volume::new::<cubic_gigameter>(value),
        "Mm³" => Volume::new::<cubic_megameter>(value),
        "km³" => Volume::new::<cubic_kilometer>(value),
        "hm³" => Volume::new::<cubic_hectometer>(value),
        "dam³" => Volume::new::<cubic_decameter>(value),
        "m³" => Volume::new::<cubic_meter>(value),
        "dm³" => Volume::new::<cubic_decimeter>(value),
        "cm³" => Volume::new::<cubic_centimeter>(value),
        "mm³" => Volume::new::<cubic_millimeter>(value),
        "µm³" => Volume::new::<cubic_micrometer>(value),
        "nm³" => Volume::new::<cubic_nanometer>(value),
        "pm³" => Volume::new::<cubic_picometer>(value),
        "ac · ft" => Volume::new::<acre_foot>(value),
        "bbl" => Volume::new::<barrel>(value),
        "bu" => Volume::new::<bushel>(value),
        "cords" => Volume::new::<cord>(value),
        "ft³" => Volume::new::<cubic_foot>(value),
        "in³" => Volume::new::<cubic_inch>(value),
        "mi³" => Volume::new::<cubic_mile>(value),
        "yd³" => Volume::new::<cubic_yard>(value),
        "cup" => Volume::new::<cup>(value),
        "fl oz" => Volume::new::<fluid_ounce>(value),
        "fl oz (UK)" => Volume::new::<fluid_ounce_imperial>(value),
        "gal (UK)" => Volume::new::<gallon_imperial>(value),
        "gal" => Volume::new::<gallon>(value),
        "gi (UK)" => Volume::new::<gill_imperial>(value),
        "gi" => Volume::new::<gill>(value),
        "TL" => Volume::new::<teraliter>(value),
        "GL" => Volume::new::<gigaliter>(value),
        "ML" => Volume::new::<megaliter>(value),
        "kL" => Volume::new::<kiloliter>(value),
        "hL" => Volume::new::<hectoliter>(value),
        "daL" => Volume::new::<decaliter>(value),
        "L" => Volume::new::<liter>(value),
        "dL" => Volume::new::<deciliter>(value),
        "cL" => Volume::new::<centiliter>(value),
        "mL" => Volume::new::<milliliter>(value),
        "µL" => Volume::new::<microliter>(value),
        "nL" => Volume::new::<nanoliter>(value),
        "pL" => Volume::new::<picoliter>(value),
        "pk" => Volume::new::<peck>(value),
        "dry pt" => Volume::new::<pint_dry>(value),
        "liq pt" => Volume::new::<pint_liquid>(value),
        "dry qt" => Volume::new::<quart_dry>(value),
        "liq qt" => Volume::new::<quart_liquid>(value),
        "tbsp" => Volume::new::<tablespoon>(value),
        "tsp" => Volume::new::<teaspoon>(value),
        "placeholder" => panic!("Unit not specified for ingredient mass"),
        x => panic!("{x} not recognized as a supported mass unit abbreviation"),
    }
}
