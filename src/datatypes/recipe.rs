use std::fs;
use std::io;
use std::path::Path;
use std::{
    collections::{HashMap, HashSet},
    fmt,
};

#[cfg(feature = "tui")]
use num_derive::{FromPrimitive, ToPrimitive};
#[cfg(feature = "tui")]
use ranged_wrapping::RangedWrapping;
#[cfg(feature = "tui")]
use ratatui::{style::Stylize, widgets::Widget};
use serde::Serialize;
use uom::si::rational64::Time;
use uuid::Uuid;

#[cfg(feature = "tui")]
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
#[cfg_attr(feature = "tui", derive(StatefulWidgetRef, WidgetRef), cookbook(state_struct = "State"))]
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct Recipe {
    /// database ID
    #[cfg_attr(feature = "tui", cookbook(skip))]
    pub id: Uuid,
    /// short name of recipe
    #[cfg_attr(feature = "tui", cookbook(display_order = 0))]
    #[cfg_attr(feature = "tui", cookbook(constraint_type = "Length"))]
    #[cfg_attr(feature = "tui", cookbook(constraint_value = 3))]
    pub name: String,
    /// optional description
    #[cfg_attr(feature = "tui", cookbook(display_order = 1))]
    #[cfg_attr(feature = "tui", cookbook(constraint_type = "Min"))]
    #[cfg_attr(feature = "tui", cookbook(constraint_value = 7))]
    pub description: Option<String>,
    //TODO: maybe make comments a bit more formal, want to be able to record when recipe was last
    //made
    /// recipe comments
    #[cfg_attr(feature = "tui", cookbook(display_order = 2))]
    #[cfg_attr(feature = "tui", cookbook(constraint_type = "Min"))]
    #[cfg_attr(feature = "tui", cookbook(constraint_value = 7))]
    pub comments: Option<String>,
    /// recipe source
    #[cfg_attr(feature = "tui", cookbook(display_order = 3))]
    #[cfg_attr(feature = "tui", cookbook(constraint_type = "Length"))]
    #[cfg_attr(feature = "tui", cookbook(constraint_value = 3))]
    pub source: String,
    /// recipe author
    #[cfg_attr(feature = "tui", cookbook(display_order = 4))]
    #[cfg_attr(feature = "tui", cookbook(constraint_type = "Length"))]
    #[cfg_attr(feature = "tui", cookbook(constraint_value = 3))]
    pub author: String,
    /// amount made
    #[cfg_attr(feature = "tui", cookbook(display_order = 5))]
    #[cfg_attr(feature = "tui", cookbook(constraint_type = "Length"))]
    #[cfg_attr(feature = "tui", cookbook(constraint_value = 3))]
    pub amount_made: AmountMade,
    /// list of steps in recipe
    #[cfg_attr(feature = "tui", cookbook(left_field))]
    #[cfg_attr(feature = "tui", cookbook(left_field_title = "Number Of Steps"))]
    pub steps: Vec<Step>,
    /// list of tags on recipe
    #[cfg_attr(feature = "tui", cookbook(skip))]
    pub tags: Vec<Tag>,
    //TODO: versions
    /// if the recipe has unsaved changes or not
    //TODO: figure out a save system
    #[cfg_attr(feature = "tui", cookbook(skip))]
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
            id: Uuid::nil(),
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
    pub fn step_time_totals(&self) -> HashMap<StepType, Option<Time>> {
        let mut out_map: HashMap<StepType, Option<Time>> = HashMap::new();
        for step in &self.steps {
            out_map
                .entry(step.step_type)
                .and_modify(|e: &mut Option<Time>| {
                    add(e, step.time_needed);
                })
                .or_insert(step.time_needed);
        }
        out_map
    }
    /// `total_time` returns the total time required for a recipe
    #[must_use]
    pub fn total_time(&self) -> Time {
        let mut time: Time = Time::default();
        for step in &self.steps {
            time += step.time_needed.unwrap_or(Time::default());
        }
        time
    }
    /// `ingredient_list` returns the total amount of ingredients required to make the recipe
    #[must_use]
    pub fn ingredient_list(&self) -> HashSet<Ingredient> {
        let mut out: HashSet<Ingredient> = HashSet::new();
        for step in &self.steps {
            for ingredient in &step.ingredients {
                if out.contains(ingredient) {
                    let mut new_ingredient = out.get(ingredient).unwrap().clone();
                    new_ingredient.unit_quantity += ingredient.unit_quantity.clone();
                    out.remove(ingredient);
                    out.insert(new_ingredient);
                } else {
                    //TODO: figure out if ingredients should be tracked using RC or not
                    out.insert(ingredient.clone());
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
    /// `*.toml` files found and return a `HashMap<Uuid, Recipe>` with the parsed `Recipe`s.
    ///
    /// # Errors
    ///
    /// Will error if:
    /// - reading any of the individual recipes fails
    /// - the specified path is not a directory
    /// - [`OsStr`](std::ffi::OsStr) failed to parse to UTF-8
    pub fn load_recipes_from_directory(dir: &Path) -> anyhow::Result<HashMap<Uuid, Self>> {
        if dir.is_dir() {
            let mut recipes: HashMap<Uuid, Self> = HashMap::new();
            Self::load_recipes_from_directory_inner(dir, &mut recipes)?;
            //recipes.sort_unstable_by_key(|r| r.id);
            Ok(recipes)
        } else {
            Err(anyhow::Error::new(io::Error::new(
                io::ErrorKind::NotADirectory,
                format! {"Provided filepath not a directory {}", dir.display()},
            )))
        }
    }

    fn load_recipes_from_directory_inner(inner_dir: &Path, recipes: &mut HashMap<Uuid, Self>) -> anyhow::Result<()> {
        let ext = match inner_dir.extension() {
            Some(ext) => match ext.to_str() {
                Some(ext) => ext,
                None => {
                    return Err(anyhow::Error::new(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "os_str failed to parse to valid utf-8",
                    )));
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
                    )));
                }
            };
            recipes.insert(recipe.id, recipe);
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
        if output.id.is_nil() {
            output.id = Uuid::new_v4();
        }
        Ok(output)
    }
    /// `write_recipe` writes an individual recipe to a toml file
    pub fn write_recipe(recipe: Recipe, out_path: &Path) -> anyhow::Result<()> {
        let output = toml::to_string_pretty(&filetypes::Recipe::from(recipe))?;
        fs::write(out_path, output)?;
        Ok(())
    }

    /// `compile_tag_list` scans through all tags on a `Vec<cookbook_core::datatypes::recipe::Recipe>`,
    /// and returns a `Vec<cookbook_core::datatypes::tag::Tag>` with all tags found.
    /// The resulting `Vec<cookbook_core::datatypes::tag::Tag>` is sorted and deduplicated before
    /// being returned
    pub fn compile_tag_list(recipes: HashMap<Uuid, Self>) -> Vec<Tag> {
        let mut tags: Vec<Tag> = Vec::new();
        for recipe in recipes.values() {
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
#[cfg(feature = "tui")]
#[derive(Debug)]
pub struct State {
    /// which field is selected in the Recipe widget display
    pub selected_field: RangedWrapping<usize>,
    /// which field is being edited, if any
    pub editing_selected_field: Option<RecipeFields>,
    // RecipeFields enum is automatically derived
    pub editing_field_cursor_position: Option<u16>,
}

#[cfg(feature = "tui")]
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
fn add(lhs: &mut Option<Time>, rhs: Option<Time>) -> Option<Time> {
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
            id: input.id.unwrap_or_default(),
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
