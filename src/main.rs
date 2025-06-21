//! cookbook TODO: add more documentation

use std::error::Error;
use std::io::{stdin, stdout, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::panic;
use std::panic::{set_hook, take_hook};
use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::Context;
use clap::{self, Parser};
use flexi_logger::{FileSpec, LogSpecification, Logger};
use gix::{
    discover::{self, upwards},
    open,
};
use log::{info, trace, warn};

use cookbook_core::datatypes::recipe::Recipe;

//TODO: allow specification of alternate ingredients

//TODO: investigate crate-ci/typos, cargo-audit/cargo-deny, codecov, bacon, editorconfig.org
//

fn main() -> anyhow::Result<()> {
    // parse command line flags
    let cli = Cli::parse();

    // init logger
    #[allow(clippy::unwrap_used)]
    //TODO: investigate to see if it is worth trying to handle these
    //errors manually
    let logger = Logger::with(LogSpecification::trace())
        .log_to_file(FileSpec::default().suppress_timestamp())
        .format_for_files(flexi_logger::opt_format)
        .start()
        .unwrap();
    // match on how many times verbose flag is present in commandline
    match cli.verbose {
        0 => logger.set_new_spec(LogSpecification::info()),
        1 => logger.set_new_spec(LogSpecification::debug()),
        _ => logger.set_new_spec(LogSpecification::trace()),
    };

    // match on how many times quiet flag is present in commandline
    match cli.quiet {
        0 => {} // do nothing
        1 => logger.set_new_spec(LogSpecification::error()),
        _ => logger.set_new_spec(LogSpecification::off()),
    };

    // TODO: parse config file

    // either use directory passed in or current directory
    let cwd = std::env::current_dir();
    let input_dir = match cli.input_directory {
        Some(ref i) => i.as_path(),
        None => match cwd {
            Ok(ref cwd) => cwd.as_path(),

            Err(e) => return Err(e.into()),
        },
    };

    let recipe_repo = load_git_repo(input_dir)?;

    if cli.check_recipe_files {
        _ = Recipe::load_recipes_from_directory(input_dir)?;
    } else if cli.print_recipe_files {
        for recipe in Recipe::load_recipes_from_directory(input_dir)? {
            let output_string = toml::to_string_pretty(&recipe)?;
            println!("{output_string}");
        }
    } else if cli.run_web_server {
        //TODO: either parse this from commandline or config file
        let ip_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        info!("running web server");
        run_web_server(input_dir, ip_addr, None)?;
    } else {
        run_tui(input_dir, recipe_repo)?;
    }

    Ok(())
}
//TODO: webpage ideas
//
// Need a main page that replicates the layout of the RecipeBrowser layout
// Need a page for editing the main recipe + each of Ingredient, Step and Equipment
// Can probably use the above for creation of recipies as well
// Need a page for viewing recipes
//
// Also need a page for populating and viewing Ingredient database

fn run_web_server(input_dir: &Path, addrs: SocketAddr, ssl_conf: Option<tiny_http::SslConfig>) -> anyhow::Result<()> {
    // A lot of this borrowed from https://github.com/tomaka/example-tiny-http/blob/master/src/lib.rs
    // as the official multi-thread example is borked
    use std::collections::{HashMap, HashSet};
    use std::sync::{mpsc, Arc};
    use std::thread;

    use anyhow::anyhow;
    use tiny_http::{http::method::Method, ConfigListenAddr, Server, ServerConfig};
    use uuid::Uuid;

    use cookbook_core::wgui::{browser, error_responses, http_helper, recipe_editor, root};

    /// `ThreadMessage` are messages that worker threads can send back to the processing
    /// thread.
    #[derive(Debug)]
    enum ThreadMessage {
        /// `AllRecipes` is a request from the worker thread to send all recipes for presentation
        AllRecipes,
        /// `RecipeRO` is a request from the worker thread for a specific recipe to be viewed and
        /// not edited
        RecipeRO(Uuid),
        /// `RecipeRW` is a request from the worker thread for a specific recipe to be edited.
        RecipeRW(Uuid),
        /// `EditedRecipe` contains the resulting edited recipe from a worker thread.
        EditedRecipe(Recipe),
        /// `NewRecipe` contains a newly created recipe from a worker thread.
        NewRecipe(Recipe),
    }
    /// `ThreadResponse` contains responses from processing thread to worker threads
    #[derive(Debug)]
    enum ThreadResponse {
        AllRecipes(HashMap<Uuid, Recipe>),
        Recipe(Recipe),
        RecipeSaved(Uuid),
        EditingError(Uuid),
    }

    let mut recipes = Recipe::load_recipes_from_directory(input_dir)?;
    let tags = Recipe::compile_tag_list(recipes.clone());

    let server_config = ServerConfig {
        addr: ConfigListenAddr::from_socket_addrs(addrs)?,
        ssl: ssl_conf,
    };
    let server = Arc::new(Server::new(server_config).unwrap());
    info!("starting web server on {addrs}");

    // TODO: add this into the config file
    let num_threads: usize = 4;
    let mut join_guards = Vec::with_capacity(num_threads + 1);
    let mut tx_channels: Vec<mpsc::Sender<ThreadResponse>> = Vec::with_capacity(num_threads);
    let mut rx_channels: Vec<mpsc::Receiver<ThreadResponse>> = Vec::with_capacity(num_threads);

    let (tx, rx) = mpsc::channel::<(usize, ThreadMessage)>();

    // create channels
    for _ in 0..num_threads {
        let (thread_tx, thread_rx) = mpsc::channel::<ThreadResponse>();
        tx_channels.push(thread_tx);
        rx_channels.push(thread_rx);
    }
    // reverse order of elements so that popping works out properly.
    rx_channels.reverse();

    // spawn data owner thread
    join_guards.push(thread::spawn(move || {
        //let mut recipes = recipes.clone();
        let mut locked_recipes: HashSet<Uuid> = HashSet::new();
        loop {
            trace!("starting data owner thread");
            // TODO: fix usage of unwrap here
            let (thread_id, message): (usize, ThreadMessage) = rx.recv().unwrap();
            match message {
                // TODO: fix usage of unwrap on send
                ThreadMessage::AllRecipes => {
                    trace!("sending an AllRecipes response to thread id {thread_id}");
                    tx_channels[thread_id]
                        .clone()
                        .send(ThreadResponse::AllRecipes(recipes.clone()))
                        .unwrap()},
                        // TODO: properly handle the Option of HashMap.get() rather than unwrapping
                        // TODO: fix usage of unwrap on send
                ThreadMessage::RecipeRO(recipe_id) => {
                    trace!("sending a Recipe response with recipe_id {recipe_id} to thread id {thread_id}. From a RecipeRO request.");
                    tx_channels[thread_id]
                        .clone()
                        .send(ThreadResponse::Recipe(recipes.get(&recipe_id).unwrap().clone()))
                        .unwrap()},
                ThreadMessage::RecipeRW(recipe_id) => {
                    // this is a hashset. HashSet::insert() returns true if the value did not
                    // previously exist.
                    let not_locked = locked_recipes.insert(recipe_id);
                    let already_locked = !not_locked;
                    if already_locked {
                        trace!("Request from thread {thread_id} to edit recipe {recipe_id}. Recipe already locked. Sending EditingError response. From a RecipeRW request.");
                        // TODO: fix usage of unwrap on send
                        tx_channels[thread_id]
                            .clone()
                            .send(ThreadResponse::EditingError(recipe_id))
                            .unwrap();
                        } else {
                            trace!("sending a Recipe response with recipe_id {recipe_id} to thread id {thread_id}. From a RecipeRW request.");
                            // TODO: fix usage of unwrap on send
                            tx_channels[thread_id]
                                .clone()
                                .send(ThreadResponse::Recipe(recipes.get(&recipe_id).unwrap().clone()))
                                .unwrap();
                    }
                }
                ThreadMessage::EditedRecipe(recipe) => {
                    // update old recipe with edited copy
                    let recipe_present = recipes.insert(recipe.id, recipe.clone());
                    if recipe_present.is_none() {
                        //TODO: handle this better
                        panic!("Edited recipe ID not found in master recipe list. This should not have happend.");
                    } else {
                        let recipe_locked = locked_recipes.remove(&recipe.id);
                        if !recipe_locked {
                            //TODO: handle this better
                            panic!("Attempted to release recipe lock that wasn't present. This shouldn't have happened.");
                        }
                        tx_channels[thread_id].clone().send(ThreadResponse::RecipeSaved(recipe.id)).unwrap();
                    }
                },
                ThreadMessage::NewRecipe(recipe) => {
                    // insert new recipe into recipes hashmap
                    let recipe_present = recipes.insert(recipe.id, recipe.clone());
                    if recipe_present.is_some() {
                        //TODO: handle this better
                        panic!("Edited recipe ID found in master recipe list while inserting new recipe. This should not have happend.");
                    }
                    tx_channels[thread_id].clone().send(ThreadResponse::RecipeSaved(recipe.id)).unwrap();
                }
            };
        }
    }));

    // spawn worker threads
    for i in 0..num_threads {
        trace! {"starting thread: {i}"}
        let server = server.clone();
        let tags = tags.clone();
        let tx = tx.clone();
        let rx = rx_channels.pop().unwrap();

        join_guards.push(thread::spawn(move || {
            loop {
                let server = server.clone();
                let tags = tags.clone();
                let tx = tx.clone();
                //TODO: remove this expect and also investigate if we can eliminate the usage of .ok()
                #[expect(clippy::option_map_unit_fn)]
                panic::catch_unwind( || -> Result<(), Box<dyn Error>> {
                    for mut request in server.incoming_requests() {
                        let method = request.method().clone();
                        let path = request.url().path();
                        trace!("{method} request received with path {path}");
                        match request.method().clone() {
                            // Not using GET requests here, as the request adds a parameter flag on
                            // the end of the URL for some reason, even though there are no
                            // parameters in use.
                            Method::GET => match request.url().path() {
                                "/" => request.respond(root::webroot()?)?,
                                "/database" => request.respond(error_responses::method_not_allowed([Method::POST]))?,
                                "/browse" =>  request.respond(error_responses::method_not_allowed([Method::POST]))?,
                                _ => request.respond(error_responses::not_found())?,
                            },
                            Method::POST => match request.url().path() {
                                "/database" => {
                                    todo!()
                                }
                                "/browse" =>  {
                                    tx.send((i, ThreadMessage::AllRecipes))?;
                                    let recipes = match rx.recv()? {
                                        ThreadResponse::AllRecipes(recipes) => recipes,
                                        _ => return Err(anyhow!("Incorrect response to request for AllRecipes").into()),
                                    };
                                    request.respond(browser::browser(recipes, &tags)?)?
                                },
                                "/view-recipe" => {
                                    // this data comes from the browse page
                                    let form_data = http_helper::parse_post_form_data(&mut request)?;
                                    if form_data.contains_key("recipe_list") {
                                        tx.send((i, ThreadMessage::RecipeRO(Uuid::parse_str(form_data["recipe_list"].as_str())?)))?;
                                        let recipe = match rx.recv()? {
                                            ThreadResponse::Recipe(recipe) => recipe,
                                            _ => return Err(anyhow!("Incorrect response to request for RecipeRW").into()),
                                        };
                                        request.respond(recipe_editor::recipe_editor(recipe)?)?
                                    }
                                }
                                "/edit-recipe" => {
                                    // this data comes from the browse page
                                    let form_data = http_helper::parse_post_form_data(&mut request)?;
                                    if form_data.contains_key("recipe_list") {
                                        tx.send((i, ThreadMessage::RecipeRW(Uuid::parse_str(form_data["recipe_list"].as_str())?)))?;
                                        let recipe = match rx.recv()? {
                                            ThreadResponse::Recipe(recipe) => recipe,
                                            //TODO: figure out how to actually provide the
                                            //offending recipe name and id to users
                                            ThreadResponse::EditingError(recipe_id) => return Ok(request.respond(error_responses::locked())?),
                                            x => {
                                                trace!("{x:?}");
                                                return Err(anyhow!("Incorrect response to request for RecipeRW").into())},
                                        };
                                        request.respond(recipe_editor::recipe_editor(recipe)?)?
                                    }
                                }
                                "/save-recipe" => {
                                    let form_data = http_helper::parse_post_form_data(&mut request)?;
                                }
                                _ => request.respond(error_responses::bad_request())?,
                            },
                            method => {
                                warn!("Unsupported method: {method:?}");
                                request.respond(error_responses::method_not_allowed([Method::GET, Method::POST]))?
                            }
                        }
                    }
                    Ok(())
                })
                .ok()
                    //TODO: can we handle these errors better rather than unwrapping
                    .map(|e| e.unwrap());
                }
        }));
    }
    for g in join_guards {
        //TODO: try using if Err(err_val) = g.join() and then using format!("{:?"}") or .downcast()
        //to move it into an anyhow!()
        //per the suggestions in https://users.rust-lang.org/t/avoiding-usage-of-unwrap-with-joinhandle/130280/12
        g.join().unwrap();
    }

    Ok(())
}

//TODO: add a status message box at the bottom of the window and log some errors to it
fn run_tui(input_dir: &Path, recipe_repo: gix::Repository) -> anyhow::Result<()> {
    use cookbook_core::tui::{
        app::{self, App},
        event::{Event, EventHandler},
        key_handler,
        keybinds::Keybinds as AppKeybinds,
        style::Style as AppStyle,
        Tui,
    };
    let events = EventHandler::new(Duration::from_millis(250));

    // TODO: set keybinds and style from config file
    let style = AppStyle::default();
    let keybinds = AppKeybinds::default();
    let mut app = App::new(keybinds, style);
    app.git_repo = Some(recipe_repo);

    app.recipes = Recipe::load_recipes_from_directory(input_dir)?;

    tui_panic_hook();
    let mut tui = Tui::init(events)?;
    let mut app_state = app::State::new(&app.save_prompt);
    app.running = true;
    while app.running {
        // render interface
        tui.draw(&app, &mut app_state)?;
        #[expect(clippy::match_same_arms)] //TODO: remove this eventually
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

fn load_git_repo(input_dir: &Path) -> anyhow::Result<gix::Repository> {
    //TODO: need to verify all recipe files are tracked in git repo
    //
    // first try to load git repo if present
    let recipe_repo: gix::Repository;
    match gix::discover(input_dir) {
        Ok(repo) => recipe_repo = repo,
        Err(e) => {
            match e {
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
                                            Ok(repo) => recipe_repo = repo,
                                            Err(e) => return Err(e.into()),
                                        }
                                        break;
                                    }
                                    "N" | "NO" => {
                                        println!("Exiting without creating git repo");
                                        // return always exits the function which in this case is main
                                        return Err(std::io::Error::new(
                                            std::io::ErrorKind::Unsupported,
                                            "Git Repository required",
                                        ))
                                        .context("No Git Repository discovered to store recipes. This is required")?;
                                    }
                                    _ => {
                                        if input.is_empty() {
                                            match gix::init(input_dir) {
                                                Ok(repo) => recipe_repo = repo,
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
                                            Ok(repo) => recipe_repo = repo,
                                            Err(e) => return Err(e.into()),
                                        }
                                        break;
                                    }

                                    "N" | "NO" => {
                                        println!("Exiting without creating git repo");
                                        // return always exits the function which in this case is main
                                        return Err(std::io::Error::new(
                                            std::io::ErrorKind::Unsupported,
                                            "Git Repository required",
                                        ))
                                        .context("No Git Repository discovered to store recipes. This is required")?;
                                    }
                                    _ => {
                                        if input.is_empty() {
                                            match gix::init(input_dir) {
                                                Ok(repo) => recipe_repo = repo,
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
            }
        }
    };

    //TODO: need to check and set committer/author
    //
    //TODO: use commit_as for automated commits (maybe provide an option for this)

    //TODO: maybe change this load function to use gix::repo::dirwalk

    // TODO: check for untracked files
    // if let Some(git_repo) = app.git_repo {
    //     match git_repo.status() {}
    // } else {
    //     return Err(Error::CookbookError(
    //         "No Git Repo defined in app. This should not have happened.".to_owned(),
    //     ));
    // }
    Ok(recipe_repo)
}

//https://ratatui.rs/recipes/apps/panic-hooks/
fn tui_panic_hook() {
    use cookbook_core::tui::Tui;
    let original_hook = take_hook();
    set_hook(Box::new(move |panic_info| {
        // intentionally ignore errors here since we're already in a panic
        // do the same functions here that the normal
        let _ = Tui::restore();
        //let _ = stdout().flush();
        original_hook(panic_info);
    }))
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
    /// Run Web Server to host a simple web gui
    #[arg(short, long)]
    run_web_server: bool,
    // Enable PostGresSql features
    //#[arg(short, long)]
    //enable_post_gres: bool,
    // PostGres DSN (optional)
    //#[arg(short, long)]
    //post_gres_dsn: Option<String>,
    /// Only shows log messages with `Error` level. Use twice to completely eliminate output. Takes precidence over verbose
    #[arg(short, long, action = clap::ArgAction::Count)]
    quiet: u8,
    /// Check recipe files for errors or bad formatting
    #[arg(short, long)]
    check_recipe_files: bool,
    /// Check recipe files for errors or bad formatting
    #[arg(short, long)]
    print_recipe_files: bool,
    // Export complete PDF
    //#[arg(short, long)]
    //export_pdf: bool,
}
