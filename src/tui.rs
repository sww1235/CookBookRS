/// `app` is the main application logic and structure
pub mod app;

/// `ui` contains the layout and formatting code for the TUI
pub mod ui;

/// `event` contains the event handling logic for the application
pub mod event;

/// `key_handler` handles all keyboard input
pub mod key_handler;

use std::io::{self, stdout, Stdout};
use std::sync::mpsc;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{backend::Backend, prelude::CrosstermBackend, Terminal};

// based on the ratatui [simple
// example](https://github.com/ratatui-org/templates/blob/main/simple/src/tui.rs)

/// A type alias for the terminal type used in this application
pub struct Tui<B: Backend> {
    /// ratatui `Terminal` representation
    terminal: Terminal<B>,
    /// event handler for App
    pub events: event::EventHandler,
}

impl Tui<CrosstermBackend<Stdout>> {
    /// initialize the terminal
    ///
    /// # Errors
    /// Will error if any of the underlying terminal manipulation commands fail
    ///
    /// # Panics
    ///
    /// May panic if terminal setup/teardown code fails. Panic handler should take care of
    /// resetting terminal back to normal state
    pub fn init(events: event::EventHandler) -> io::Result<Self> {
        // enable terminal raw mode
        enable_raw_mode()?;
        // execute a command on the terminal handle returned by stdout()
        execute!(stdout(), EnterAlternateScreen, EnableMouseCapture)?;

        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

        terminal.hide_cursor()?;
        terminal.clear()?;
        let tui = Self { terminal, events };

        // set up panic restore hook
        let panic_hook = std::panic::take_hook();
        // allowing expect since it is happening in a panic handler
        #[allow(clippy::expect_used)]
        std::panic::set_hook(Box::new(move |panic| {
            Self::restore().expect("failed to reset the terminal");
            panic_hook(panic);
        }));

        // create new terminal backend
        Ok(tui)
    }

    /// renders ui of TUI
    ///
    /// # Errors
    /// Will error if any of the underlying terminal manipulation commands fail
    pub fn draw(&mut self, app: &mut app::App) -> io::Result<()> {
        self.terminal.draw(|frame| ui::layout(frame, app))?;
        Ok(())
    }

    /// restore terminal to original state
    ///
    /// # Errors
    /// Will error if any of the underlying terminal manipulation commands fail
    pub fn restore() -> io::Result<()> {
        execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
        disable_raw_mode()?;
        Ok(())
    }

    /// exits TUI and restores terminal
    ///
    /// # Errors
    /// Will error if any of the underlying terminal manipulation commands fail
    pub fn exit(&mut self) -> io::Result<()> {
        Self::restore()?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}

/// Error type for TUI
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// std::io::Error
    IOError(io::Error),
    /// std::sync::mpsc::RecvError
    RecvError(mpsc::RecvError),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Self::IOError(ref e) => write!(f, "{e}"),
            Self::RecvError(ref e) => write!(f, "{e}"),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::IOError(e)
    }
}

impl From<mpsc::RecvError> for Error {
    fn from(e: mpsc::RecvError) -> Self {
        Self::RecvError(e)
    }
}
