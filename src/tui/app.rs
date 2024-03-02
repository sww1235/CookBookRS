use crate::{datatypes::recipe::Recipe, tui::ui};

use std::io;

use ratatui::{backend::Backend, buffer, layout::Rect, terminal::Terminal, widgets::Widget, Frame};

/// main application struct
#[derive(Debug, Default)]
pub struct App {
    recipes: Vec<Recipe>,
    current_screen: CurrentScreen,
    exit: bool,
}

/// `CurrentScreen` represents the screen the user is currently seeing
#[derive(Debug, Default)]
pub enum CurrentScreen {
    /// `RecipeBrowser` is the main screen that shows a list of recipes, and allows users to filter
    /// and search for recipes
    #[default]
    RecipeBrowser,
    /// `RecipeEditing` allows users to edit recipes
    RecipeEditing,
    /// `RecipeViewing` is the main way to view a recipe
    RecipeViewing,
}

impl App {
    /// `new` creates a new `App`
    pub fn new() -> Self {
        Self {
            recipes: Vec::new(),
            current_screen: CurrentScreen::default(),
            exit: false,
        }
    }

    /// `run` starts the main application loop that exists until the user quits
    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| ui::ui_init(frame, self))?;
            //self.handle_events()?;
        }
        Ok(())
    }
    fn handle_events(&mut self) -> io::Result<()> {
        todo!()
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut buffer::Buffer) {
        //
        todo!()
    }
}
