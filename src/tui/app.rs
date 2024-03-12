use crate::datatypes::recipe::Recipe;

use ratatui::widgets::ListState;

/// main application struct
#[derive(Debug, Default)]
pub struct App {
    /// the recipes contained in the application
    pub recipes: Vec<Recipe>,
    /// the current screen the application is on
    pub current_screen: CurrentScreen,
    /// state for recipe list
    pub recipe_list_state: ListState,
    /// length of recipe list
    pub recipe_list_len: usize,
    /// editing flag
    pub editing: bool,
    /// editing state
    pub editing_state: EditingState,
    /// running flag
    pub running: bool,
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
#[derive(Debug, Default)]
#[non_exhaustive]
pub enum EditingState {
    /// Idle
    #[default]
    Idle,
    /// Editing title
    Title(String),
    /// Editing ingredient
    Ingredient,
    /// Editing step
    Step,
}

impl App {
    /// `new` creates a new `App`
    #[must_use]
    pub fn new() -> Self {
        Self {
            recipes: Vec::new(),
            current_screen: CurrentScreen::default(),
            recipe_list_state: ListState::default(),
            recipe_list_len: usize::default(),
            running: false,
            editing: false,
            editing_state: EditingState::default(),
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
