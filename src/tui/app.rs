use crate::datatypes::{recipe::Recipe, tag::Tag};

use ratatui::widgets::{ListState, ScrollbarState};

/// main application struct
#[derive(Debug, Default, PartialEq)]
pub struct App {
    /// the recipes contained in the application
    pub recipes: Vec<Recipe>,
    /// either a new recipe, or a clone of the recipe that is currently being edited
    pub edit_recipe: Option<Recipe>,
    /// the current screen the application is on
    pub current_screen: CurrentScreen,
    /// state for recipe list
    pub recipe_list_state: ListState,
    /// length of recipe list
    pub recipe_list_len: usize,
    /// editing flag, indicating which recipe you are editing. Not used for creating new recipes
    pub editing: Option<usize>,
    /// editing state
    pub editing_state: EditingState,
    /// running flag
    pub running: bool,
    /// recipe list scrollbar state
    pub recipe_scroll_state: ScrollbarState,
    /// scrollbar state for viewer/editor
    pub middle_scrollbar_state: ScrollbarState,
    /// tag list
    pub tags: Vec<Tag>,
    /// tag list state
    pub tag_list_state: ListState,
    /// tag list length
    pub tag_list_len: usize,
}

/// `CurrentScreen` represents the screen the user is currently seeing
#[derive(Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum CurrentScreen {
    /// `RecipeBrowser` is the main screen that shows a list of recipes, and allows users to filter
    /// and search for recipes
    #[default]
    RecipeBrowser,
    /// `RecipeEditing` allows users to edit recipes
    RecipeEditor,
    /// `RecipeViewing` is the main way to view a recipe
    RecipeViewer,
    /// `RecipeCreator` is used for entry of new recipes
    RecipeCreator,
}

/// `EditingState` represents the current state of the editing/creation workflow
#[derive(Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum EditingState {
    /// Idle
    #[default]
    Idle,
    /// Editing recipe
    Recipe,
    /// Editing step, first index is step index
    Step(usize),
    /// Editing ingredient, first index is step index, second index is ingredient index within step
    Ingredient(usize, usize),
    /// Editing equipment, first index is step index, second index is equipment index within step
    Equipment(usize, usize),
    ///Save Prompt
    SavePrompt,
}

impl App {
    /// [`new`] creates a new [`App`]
    #[must_use]
    pub fn new() -> Self {
        Self {
            recipes: Vec::new(),
            edit_recipe: None,
            current_screen: CurrentScreen::default(),
            recipe_list_state: ListState::default(),
            recipe_list_len: usize::default(),
            running: false,
            editing: None,
            editing_state: EditingState::default(),
            recipe_scroll_state: ScrollbarState::default(),
            middle_scrollbar_state: ScrollbarState::default(),
            tags: Vec::new(),
            tag_list_state: ListState::default(),
            tag_list_len: usize::default(),
        }
    }

    /// [`compile_tag_list`] scans through all tags on all recipes, compiles them into the main app
    /// tag list, then sorts and deduplicates the list
    pub fn compile_tag_list(&mut self) {
        for recipe in &self.recipes {
            //TODO: maybe switch to using try_reserve instead
            self.tags.reserve(recipe.tags.len());
            self.tags.extend(recipe.tags.clone());
        }
        // don't care about order of duplicate elements since we are removing them
        self.tags.sort_unstable();
        self.tags.dedup();
        self.tags.shrink_to_fit();
    }

    /// [`tick`] handles the tick event of the app
    pub fn tick(&self) {
        todo!()
    }

    /// [`exit`] exits App
    pub fn exit(&mut self) {
        self.running = false;
    }
}
