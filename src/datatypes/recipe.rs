use super::{
    equipment::Equipment,
    ingredient::Ingredient,
    step::{Step, StepType},
};

use std::collections::HashMap;

use dimensioned::ucum;

/// `Recipe` represents one recipe from start to finish
#[derive(Default, Debug, Clone)]
pub struct Recipe {
    /// database ID
    pub id: u64,
    /// short name of recipe
    pub name: String,
    /// optional description
    pub description: Option<String>,
    /// recipe comments
    pub comments: Option<String>,
    /// recipe source
    pub source: String,
    /// recipe author
    pub author: String,
    //pub amount_made: TODO: units,
    /// list of steps in recipe
    pub steps: Vec<Step>,
    //TODO: tags, versions
    //TODO: maybe make comments a bit more formal, want to be able to record when recipe was last
    //made
}

impl Recipe {
    /// `step_time_totals` provides the time required for each type of step as a `HashMap`
    #[must_use]
    #[allow(clippy::arithmetic_side_effects)] //TODO: fix this
    pub fn step_time_totals(&self) -> HashMap<StepType, Option<ucum::Second<f64>>> {
        let mut out_map: HashMap<StepType, Option<ucum::Second<f64>>> = HashMap::new();
        for step in &self.steps {
            out_map
                .entry(step.step_type)
                .and_modify(|e: &mut Option<ucum::Second<f64>>| {
                    add(e, step.time_needed);
                })
                .or_insert(step.time_needed);
        }
        out_map
    }
    /// `total_time` returns the total time required for a recipe
    #[must_use]
    #[allow(clippy::arithmetic_side_effects)] //TODO: fix this
    pub fn total_time(&self) -> ucum::Second<f64> {
        let mut time = 0.0_f64 * ucum::S;
        for step in &self.steps {
            time += step.time_needed.unwrap_or(0.0_f64 * ucum::S);
        }
        time
    }
    /// `ingredient_list` returns the total amount of ingredients required to make the recipe
    #[must_use]
    pub fn ingredient_list(&self) -> HashMap<String, Ingredient> {
        let mut out: HashMap<String, Ingredient> = HashMap::new();
        #[allow(clippy::arithmetic_side_effects)] //TODO: fix this
        for step in &self.steps {
            for ingredient in &step.ingredients {
                if let Some(i) = out.get_mut(&ingredient.name) {
                    i.unit += ingredient.unit;
                } else {
                    //TODO: figure out if ingredients should be tracked using RC or not
                    out.insert(ingredient.name.clone(), ingredient.clone());
                }
            }
        }
        out
    }
    /// `equipment_list` returns the overall list of equipment needed to make the recipe
    #[must_use]
    pub fn equipment_list(&self) -> Vec<Equipment> {
        let mut out = Vec::new();
        for step in &self.steps {
            for equipment in &step.equipment {
                // all short circuits if the closure returns false, and then returns false. We
                // invert that false to true to see if a value is not contained in the vector
                if !out.iter().all(|e| e == equipment) {
                    out.push(equipment.clone());
                }
            }
        }
        out
    }
    /// `all_equipment_owned` returns the overall list of equipment needed to make the recipe
    #[must_use]
    pub fn all_equipment_owned(&self) -> bool {
        // iterate through all equipment in all steps, short circuiting if e.is_owned is false
        self.steps
            .iter()
            .all(|s| s.equipment.iter().all(|e| e.is_owned))
    }
}
//https://www.reddit.com/r/learnrust/comments/1b1xwci/best_way_to_add_an_optiont_to_an_optiont/
/// helper function for `step_time_totals` to allow adding an option and an option togther
fn add(
    lhs: &mut Option<ucum::Second<f64>>,
    rhs: Option<ucum::Second<f64>>,
) -> Option<ucum::Second<f64>> {
    #[allow(clippy::arithmetic_side_effects)] //TODO: change this to saturating
    match (lhs, rhs) {
        (Some(l), Some(r)) => Some(*l + r),
        (Some(l), None) => Some(*l),
        (None, Some(r)) => Some(r),
        (None, None) => None,
    }
}
