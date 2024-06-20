//! cookbook TODO: add more documentation

use std::io::{stdin, stdout, Write};
use std::path::PathBuf;
use std::time::Duration;

use clap::{self, Parser};
use gix::{
    discover::{self, upwards},
    open,
};

use cookbook_core::tui::{
    app::{App, AppState},
    event::{Event, EventHandler},
    key_handler, Error, Tui,
};

//TODO: investigate crate-ci/typos, cargo-audit/cargo-deny, codecov, bacon, editorconfig.org

#[allow(clippy::result_large_err)] //TODO: fix this
fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    let events = EventHandler::new(Duration::from_millis(250));
    let mut app = App::new();

    // either use directory passed in or current directory
    let cwd = std::env::current_dir();
    let input_dir = match cli.input_directory {
        Some(ref i) => i.as_path(),
        None => match cwd {
            Ok(ref cwd) => cwd.as_path(),
            Err(e) => return Err(e.into()),
        },
    };

    //TODO: need to verify all recipe files are tracked in git repo
    //
    // first try to load git repo if present
    match gix::discover(input_dir) {
        Ok(repo) => app.git_repo = Some(repo),
        Err(e) => match e {
            discover::Error::Discover(e) => match e {
                upwards::Error::NoGitRepository { .. }
                | upwards::Error::NoGitRepositoryWithinCeiling { .. }
                | upwards::Error::NoGitRepositoryWithinFs { .. } => {
                    // if git repo is not detected, then prompt to create one.
                    // TODO: provide a command line argument to always create a git repo
                    let crate_name = clap::crate_name!();
                    let path_string = input_dir.display();
                    println!("Git repository not detected at path {path_string}.");
                    println!("This is required for the version tracking and orginization of the cookbook.");
                    println!("You can either automatically have {crate_name} initialize one for you (Y) or exit and manually initialize it yourself (N).");
                    print!("Do you want to automatically create one? ([Y]/N)");
                    // need to flush output to screen prior to prompting for response.
                    stdout().flush()?;
                    let mut input = String::new();
                    loop {
                        match stdin().read_line(&mut input) {
                            Ok(_) => match input.trim().to_uppercase().as_str() {
                                "Y" | "YES" => {
                                    match gix::init(input_dir) {
                                        Ok(repo) => app.git_repo = Some(repo),
                                        Err(e) => return Err(e.into()),
                                    }
                                    break;
                                }
                                "N" | "NO" => {
                                    println!("Exiting without creating git repo");
                                    // return always exits the function which in this case is main
                                    return Ok(());
                                }
                                _ => {
                                    if input.is_empty() {
                                        match gix::init(input_dir) {
                                            Ok(repo) => app.git_repo = Some(repo),
                                            Err(e) => return Err(e.into()),
                                        }
                                        break;
                                    } else {
                                        println!("Either enter [Y]es, [N]o or hit enter to accept the default of Yes");
                                    }
                                }
                            },
                            Err(e) => return Err(e.into()),
                        }
                    }
                }
                e => return Err(e.into()),
            },
            discover::Error::Open(e) => match e {
                open::Error::NotARepository { .. } => {
                    // if git repo is not detected, then prompt to create one.
                    // TODO: provide a command line argument to always create a git repo
                    let crate_name = clap::crate_name!();
                    let path_string = input_dir.display();
                    println!("Git repository not detected at path {path_string}.");
                    println!("This is required for the version tracking and orginization of the cookbook.");
                    println!("You can either automatically have {crate_name} initialize one for you (Y) or exit and manually initialize it yourself (N).");
                    print!("Do you want to automatically create one? ([Y]/N)");
                    // need to flush output to screen prior to prompting for response.
                    stdout().flush()?;
                    let mut input = String::new();
                    loop {
                        match stdin().read_line(&mut input) {
                            Ok(_) => match input.trim().to_uppercase().as_str() {
                                "Y" | "YES" => {
                                    match gix::init(input_dir) {
                                        Ok(repo) => app.git_repo = Some(repo),
                                        Err(e) => return Err(e.into()),
                                    }
                                    break;
                                }

                                "N" | "NO" => {
                                    println!("Exiting without creating git repo");
                                    // return always exits the function which in this case is main
                                    return Ok(());
                                }
                                _ => {
                                    if input.is_empty() {
                                        match gix::init(input_dir) {
                                            Ok(repo) => app.git_repo = Some(repo),
                                            Err(e) => return Err(e.into()),
                                        }
                                        break;
                                    } else {
                                        println!("Either enter [Y]es, [N]o or hit enter to accept the default of Yes");
                                    }
                                }
                            },
                            Err(e) => return Err(e.into()),
                        }
                    }
                }
                e => return Err(e.into()),
            },
        },
    };
    app.load_recipes_from_directory(input_dir)?;

    let mut tui = Tui::init(events)?;
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
    /// Directory that cookbook lives in
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
    // Export complete PDF
    //#[arg(short, long)]
    //export_pdf: bool,
}
