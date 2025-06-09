use std::fs;
use std::io;
use std::path::Path;
use std::{collections::HashMap, fmt};

use dimensioned::ucum;
use num_derive::{FromPrimitive, ToPrimitive};
use ranged_wrapping::RangedWrapping;
use ratatui::{style::Stylize, widgets::Widget};
use serde::Serialize;
use uuid::Uuid;

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
//
//TODO: change the macro generating the rendering to print list of steps with ingredients/equipment at
//the top for display only
//

/// `Recipe` represents one recipe from start to finish
#[derive(Default, Debug, Clone, PartialEq, StatefulWidgetRef, WidgetRef, Serialize)]
#[cookbook(state_struct = "State")]
pub struct Recipe {
    /// database ID
    #[cookbook(skip)]
    pub id: Option<Uuid>,
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
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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

    /// `load_recipes_from_directory` recursively parses the provided directory path to parse all
    /// `*.toml` files found and return a `Vec<Recipe>` with the parsed `Recipe`s.
    ///
    /// # Errors
    ///
    /// Will error if:
    /// - reading any of the individual recipes fails
    /// - the specified path is not a directory
    /// - [`OsStr`](std::ffi::OsStr) failed to parse to UTF-8
    pub fn load_recipes_from_directory(dir: &Path) -> anyhow::Result<Vec<Self>> {
        if dir.is_dir() {
            let mut recipes: Vec<Self> = Vec::new();
            Self::load_recipes_from_directory_inner(dir, &mut recipes)?;
            recipes.sort_unstable_by_key(|r| r.id);
            Ok(recipes)
        } else {
            Err(anyhow::Error::new(io::Error::new(
                io::ErrorKind::NotADirectory,
                format! {"Provided filepath not a directory {}", dir.display()},
            )))
        }
    }

    fn load_recipes_from_directory_inner(inner_dir: &Path, recipes: &mut Vec<Self>) -> anyhow::Result<()> {
        let ext = match inner_dir.extension() {
            Some(ext) => match ext.to_str() {
                Some(ext) => ext,
                None => {
                    return Err(anyhow::Error::new(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "os_str failed to parse to valid utf-8",
                    )))
                }
            },
            None => "",
        };
        if inner_dir.is_file() && ext == "toml" {
            let recipe = match Self::parse_recipe(inner_dir) {
                Ok(r) => r,
                Err(error) => {
                    return Err(anyhow::Error::new(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format! {"Parsing TOML file {} failed: {}", &inner_dir.display(), error},
                    )))
                }
            };
            recipes.push(recipe);
            Ok(())
        } else if inner_dir.is_dir() {
            for entry in fs::read_dir(inner_dir)? {
                let entry = entry?; // read_dir returns result
                let path = entry.path();
                Self::load_recipes_from_directory_inner(&path, recipes)?;
            }
            Ok(())
        } else {
            // not a directory or file (maybe a symlink or something?
            Ok(())
        }
    }

    fn parse_recipe(recipe_file: &Path) -> anyhow::Result<Self> {
        let contents = fs::read_to_string(recipe_file)?;
        let output: filetypes::Recipe = toml::from_str(contents.as_str())?;
        let mut output: Self = output.into();
        if output.id.is_none() {
            output.id = Some(Uuid::new_v4());
        }
        Ok(output)
    }

    /// `compile_tag_list` scans through all tags on a `Vec<cookbook_core::datatypes::recipe::Recipe>`,
    /// and returns a `Vec<cookbook_core::datatypes::tag::Tag>` with all tags found.
    /// The resulting `Vec<cookbook_core::datatypes::tag::Tag>` is sorted and deduplicated before
    /// being returned
    pub fn compile_tag_list(recipes: &[Self]) -> Vec<Tag> {
        let mut tags: Vec<Tag> = Vec::new();
        for recipe in recipes {
            //TODO: maybe switch to using try_reserve instead
            tags.reserve(recipe.tags.len());
            tags.extend(recipe.tags.clone());
        }
        // don't care about order of duplicate elements since we are removing them
        tags.sort_unstable();
        tags.dedup();
        tags.shrink_to_fit();
        tags
    }
}

/// `State` contains the state of the Recipe widget
#[derive(Debug)]
pub struct State {
    /// which field is selected in the Recipe widget display
    pub selected_field: RangedWrapping<usize>,
    /// which field is being edited, if any
    pub editing_selected_field: Option<RecipeFields>,
    // RecipeFields enum is automatically derived
    pub editing_field_cursor_position: Option<u16>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            selected_field: RangedWrapping {
                value: 0,
                max: Recipe::NUM_FIELDS,
                min: 0,
            },
            editing_selected_field: None,
            editing_field_cursor_position: None,
        }
    }
}

//https://www.reddit.com/r/learnrust/comments/1b1xwci/best_way_to_add_an_optiont_to_an_optiont/
/// helper function for `step_time_totals` to allow adding an option and an option togther
fn add(lhs: &mut Option<ucum::Second<f64>>, rhs: Option<ucum::Second<f64>>) -> Option<ucum::Second<f64>> {
    #[expect(clippy::arithmetic_side_effects)] //TODO: change this to saturating
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
