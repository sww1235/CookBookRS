//! cookbook TODO: add more documentation

use cookbook_core::tui::{app::App, tui};

use std::io;

fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;
    let app_result = App::new().run(&mut terminal);
    tui::restore()?;
    app_result
}
