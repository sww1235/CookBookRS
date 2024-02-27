use super::{equipment::Equipment, ingredient::Ingredient};
pub struct Step {
    /// database ID
    id: u64,
    //TimeNeeded: Option<TODO: units>
    //Temperature: Option<TODO: units>
    /// instructions for step
    instructions: String,
    /// ingredients used in this step
    ingredients: Vec<Ingredient>,
    /// equipment used in this step
    equipment: Vec<Equipment>,
}

/// `StepType` represents what type of step each step is in a recipe. It is used to bucket times
/// for recipe total duration
pub enum StepType {
    /// Prep steps
    Prep,
    /// cook steps
    Cook,
    /// waiting steps
    Wait,
    /// Other steps
    Other,
}
