use std::num::Wrapping;
use std::{collections::HashMap, fmt};

use dimensioned::ucum;
use num_derive::{FromPrimitive, ToPrimitive};
use ratatui::{style::Stylize, widgets::Widget};

use cookbook_macros::{StatefulWidgetRef, WidgetRef};

use super::{
    equipment::Equipment,
    filetypes,
    ingredient::Ingredient,
    step::{Step, StepType},
    tag::Tag,
};

//TODO: associate equipment with recipe and steps, so you don't have to re-enter info for equipment
//that is used on multiple steps. Maybe do this with ingredients as well? May have to use ref_cell
//for this

/// `Recipe` represents one recipe from start to finish
#[derive(Default, Debug, Clone, PartialEq, StatefulWidgetRef, WidgetRef)]
#[cookbook(state_struct = "RecipeState")]
pub struct Recipe {
    /// database ID
    #[cookbook(skip)]
    pub id: Option<u64>,
    /// short name of recipe
    #[cookbook(display_order = 0)]
    #[cookbook(constraint_type = "Length")]
    #[cookbook(constraint_value = 3)]
    pub name: String,
    /// optional description
    #[cookbook(display_order = 1)]
    #[cookbook(constraint_type = "Min")]
    #[cookbook(constraint_value = 7)]
    pub description: Option<String>,
    //TODO: maybe make comments a bit more formal, want to be able to record when recipe was last
    //made
    /// recipe comments
    #[cookbook(display_order = 2)]
    #[cookbook(constraint_type = "Min")]
    #[cookbook(constraint_value = 7)]
    pub comments: Option<String>,
    /// recipe source
    #[cookbook(display_order = 3)]
    #[cookbook(constraint_type = "Length")]
    #[cookbook(constraint_value = 3)]
    pub source: String,
    /// recipe author
    #[cookbook(display_order = 4)]
    #[cookbook(constraint_type = "Length")]
    #[cookbook(constraint_value = 3)]
    pub author: String,
    /// amount made
    #[cookbook(display_order = 5)]
    #[cookbook(constraint_type = "Length")]
    #[cookbook(constraint_value = 3)]
    pub amount_made: AmountMade,
    /// list of steps in recipe
    #[cookbook(left_field)]
    #[cookbook(left_field_title = "Number Of Steps")]
    pub steps: Vec<Step>,
    /// list of tags on recipe
    #[cookbook(skip)]
    pub tags: Vec<Tag>,
    //TODO: versions
    /// if the recipe has unsaved changes or not
    //TODO: figure out a save system
    #[cookbook(skip)]
    pub saved: bool,
}

/// [`AmountMade`] represents the total finished quantity that the recipe makes, like 24 cookies,
/// 24 servings, 6 portions, etc.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AmountMade {
    /// amount made
    pub quantity: u64,
    /// units for amount made.
    ///
    /// Thse are not type checked at all and are treated as a base quantity internally.
    /// This is just a representation of the units to display.
    /// There may be a future addition that automatically calculates calories, or serving
    /// sizes based on calories.
    pub units: String,
}

impl fmt::Display for AmountMade {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Makes: {} {}", self.quantity, self.units)
    }
}

impl Recipe {
    /// `new` creates a new [`Recipe`]
    #[must_use]
    pub fn new() -> Self {
        Self {
            id: None,
            name: String::default(),
            description: None,
            comments: None,
            source: String::default(),
            author: String::default(),
            amount_made: AmountMade::default(),
            steps: Vec::new(),
            tags: Vec::new(),
            //TODO: versions
            saved: false,
        }
    }

    /// `step_time_totals` provides the time required for each type of step as a `HashMap`
    #[must_use]
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
        for step in &self.steps {
            for ingredient in &step.ingredients {
                if let Some(i) = out.get_mut(&ingredient.name) {
                    i.unit_quantity += ingredient.unit_quantity;
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
        self.steps.iter().all(|s| s.equipment.iter().all(|e| e.is_owned))
    }
}

/// [`RecipeState`]
#[derive(Default, Debug)]
#[allow(clippy::module_name_repetitions, missing_docs)]
pub struct RecipeState {
    pub selected_field: Wrapping<usize>,
    pub num_fields: usize,
    pub editing_selected_field: Option<RecipeFields>,
}

//https://www.reddit.com/r/learnrust/comments/1b1xwci/best_way_to_add_an_optiont_to_an_optiont/
/// helper function for `step_time_totals` to allow adding an option and an option togther
fn add(lhs: &mut Option<ucum::Second<f64>>, rhs: Option<ucum::Second<f64>>) -> Option<ucum::Second<f64>> {
    #[allow(clippy::arithmetic_side_effects)] //TODO: change this to saturating
    match (lhs, rhs) {
        (Some(l), Some(r)) => Some(*l + r),
        (Some(l), None) => Some(*l),
        (None, Some(r)) => Some(r),
        (None, None) => None,
    }
}

impl From<filetypes::Recipe> for Recipe {
    fn from(input: filetypes::Recipe) -> Self {
        Self {
            id: input.id,
            name: input.name,
            description: input.description,
            comments: input.comments,
            source: input.source,
            author: input.author,
            amount_made: AmountMade {
                quantity: input.amount_made,
                units: input.amount_made_units,
            },
            steps: input.steps.into_iter().map(Into::into).collect(),
            tags: input.tags,
            saved: false,
        }
    }
}
