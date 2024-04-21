//! cookbook TODO: add more documentation

use cookbook_core::tui::{
    app::{App, AppState},
    event::{Event, EventHandler},
    key_handler, Error, Tui,
};

use std::path::PathBuf;
use std::time::Duration;

use clap::Parser;

//TODO: investigate crate-ci/typos, cargo-audit/cargo-deny, codecov, bacon, editorconfig.org

fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    let events = EventHandler::new(Duration::from_millis(250));
    let mut tui = Tui::init(events)?;
    let mut app = App::new();
    if let Some(input_dir) = cli.input_directory {
        app.load_recipes_from_directory(input_dir)?;
    }
    let mut app_state = AppState::default();
    app.running = true;
    while app.running {
        // render interface
        tui.draw(&app, &mut app_state)?;
        #[allow(clippy::match_same_arms)] //TODO: remove this eventually
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => {
                key_handler::handle_key_events(&mut app, &mut app_state, key_event);
            }
            Event::Mouse(_) => {
                //TODO
            }
            // redraw app on resize
            Event::Resize(_, _) => tui.draw(&app, &mut app_state)?,
            _ => {
                //TODO
            }
        }
    }
    Tui::restore()?;
    Ok(())
}
/// `Cli` holds the defintions for command line arguments used in this binary
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Directory that project lives in
    input_directory: Option<PathBuf>,
    /// Increase verbosity of program by adding more v
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
    // Enable PostGresSql features
    //#[arg(short, long)]
    //enable_post_gres: bool,
    // PostGres DSN (optional)
    //#[arg(short, long)]
    //post_gres_dsn: Option<String>,
    // Only shows log messages with <Error> level. Use twice to completely eliminate output. Takes precidence over verbose
    //#[arg(short, long, action = clap::ArgAction::Count)]
    //quiet: u8,
    // Do not use default libraries included with program
    //#[arg(short, long)]
    //no_default_libs: bool,
    // Export complete PDF
    //#[arg(short, long)]
    //export_pdf: bool,
}
