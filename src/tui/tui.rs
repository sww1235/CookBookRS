use std::io::{self, stdout, Stdout};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{prelude::CrosstermBackend, Terminal};

/// A type alias for the terminal type used in this application
pub type Tui = Terminal<CrosstermBackend<Stdout>>;

//impl Tui {
/// initialize the terminal
pub fn init() -> io::Result<Tui> {
    // execute a command on the terminal handle returned by stdout()
    execute!(stdout(), EnterAlternateScreen, EnableMouseCapture)?;
    // enable terminal raw mode
    enable_raw_mode()?;
    // create new terminal backend
    Terminal::new(CrosstermBackend::new(stdout()))
}

/// restore terminal to original state
pub fn restore() -> io::Result<()> {
    execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
    disable_raw_mode()?;
    Ok(())
}
//}
