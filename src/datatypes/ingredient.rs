use dimensioned::ucum;

use std::ops::{Add, AddAssign};

/// `Ingredient` is a unique item that represents the quantity of a particular ingredient
#[derive(Debug, Clone, Default)]
pub struct Ingredient {
    /// database ID
    pub id: u64,
    /// ingredient short name
    pub name: String,
    /// optional description
    pub description: Option<String>,
    /// Unit of ingredient
    pub unit: UnitType,
    //TODO: inventory reference
}

/// `UnitType` handles different unit types for an ingredient and allows flexibility rather than
/// needing to have 1 ingredient type per unit type
#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
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
            (UnitType::Quantity(l), UnitType::Quantity(r)) => UnitType::Quantity(l + r),
            (UnitType::Mass(l), UnitType::Mass(r)) => UnitType::Mass(l + r),
            (UnitType::Volume(l), UnitType::Volume(r)) => UnitType::Volume(l + r),
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
        UnitType::Quantity(0.0_f64)
    }
}
