use dimensioned::ucum;

use std::ops::{Add, AddAssign};

use struct_field_names_as_array::FieldNamesAsSlice;

/// `Ingredient` is a unique item that represents the quantity of a particular ingredient
#[derive(Default, Debug, Clone, PartialEq, FieldNamesAsSlice)]
pub struct Ingredient {
    /// database ID
    #[field_names_as_slice(skip)]
    pub id: u64,
    /// ingredient short name
    pub name: String,
    /// optional description
    pub description: Option<String>,
    /// Unit and quantity of ingredient
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

impl Add for UnitType {
    type Output = Self;

    #[allow(clippy::arithmetic_side_effects)] //TODO: fix this
    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Self::Quantity(l), Self::Quantity(r)) => Self::Quantity(l + r),
            (Self::Mass(l), Self::Mass(r)) => Self::Mass(l + r),
            (Self::Volume(l), Self::Volume(r)) => Self::Volume(l + r),
            _ => panic!(
                "Attempted to add different unit types together. This should not have happened"
            ),
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
