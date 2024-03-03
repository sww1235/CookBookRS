/// `app` is the main application logic and structure
pub mod app;

/// `ui` contains the layout and formatting code for the TUI
pub mod ui;

use std::io::{self, stdout, Stdout};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{prelude::CrosstermBackend, Terminal};

/// A type alias for the terminal type used in this application
pub type Tui = Terminal<CrosstermBackend<Stdout>>;

/// initialize the terminal
///
/// # Errors
/// Will error if any of the underlying terminal manipulation commands fail
pub fn init() -> io::Result<Tui> {
    // execute a command on the terminal handle returned by stdout()
    execute!(stdout(), EnterAlternateScreen, EnableMouseCapture)?;
    // enable terminal raw mode
    enable_raw_mode()?;
    // create new terminal backend
    Terminal::new(CrosstermBackend::new(stdout()))
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
