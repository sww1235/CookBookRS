use crate::{datatypes::recipe::Recipe, tui::ui};

use std::io;

use ratatui::{backend::Backend, terminal::Terminal};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

/// main application struct
#[derive(Debug, Default)]
pub struct App {
    /// the recipes contained in the application
    pub recipes: Vec<Recipe>,
    /// the current screen the application is on
    pub current_screen: CurrentScreen,
    /// exit flag
    pub exit: bool,
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
    RecipeEditing,
    /// `RecipeViewing` is the main way to view a recipe
    RecipeViewing,
}

impl App {
    /// `new` creates a new `App`
    #[must_use]
    pub fn new() -> Self {
        Self {
            recipes: Vec::new(),
            current_screen: CurrentScreen::default(),
            exit: false,
        }
    }

    /// `run` starts the main application loop that exists until the user quits
    ///
    /// # Errors
    /// Will error if any of the underlying terminal manipulation commands fail
    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        while !self.exit {
            // change the call to ui::ui_init to change what is displayed
            terminal.draw(|frame| ui::layout(frame, self))?;
            self.handle_events()?;
        }
        Ok(())
    }
    /// `handle_events` handles all [`crossterm::event`]s
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // only match key presses to avoid key release/repeat events on Windows
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_events(key_event);
            }
            _ => {}
        };
        Ok(())
    }
    /// `handle_key_event` handles all `KeyEvent`s
    fn handle_key_events(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }
    /// `exit` exits App
    fn exit(&mut self) {
        self.exit = true;
    }
}
