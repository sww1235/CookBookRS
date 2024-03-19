use crate::datatypes::recipe::Recipe;

use ratatui::widgets::{ListState, ScrollbarState};

/// main application struct
#[derive(Debug, Default)]
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
}

/// `CurrentScreen` represents the screen the user is currently seeing
#[derive(Debug, Default)]
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
    Save,
}

impl App {
    /// `new` creates a new `App`
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
        }
    }

    /// `tick` handles the tick event of the app
    pub fn tick(&self) {
        todo!()
    }

    /// `exit` exits App
    pub fn exit(&mut self) {
        self.running = false;
    }
}
