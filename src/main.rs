//! cookbook TODO: add more documentation

use cookbook_core::tui::{self, app::App};

use std::io;

use clap::Parser;

fn main() -> io::Result<()> {
    let _cli = Cli::parse();
    let mut terminal = tui::init()?;
    let app_result = App::new().run(&mut terminal);
    tui::restore()?;
    app_result
}
/// `Cli` holds the defintions for command line arguments used in this binary
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Directory that project lives in
    //project_directory: PathBuf,
    /// Increase verbosity of program by adding more v
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
    // Enable PostGresSql features
    //#[arg(short, long)]
    //enable_post_gres: bool,
    // PostGres DSN (optional)
    //#[arg(short, long)]
    //post_gres_dsn: Option<String>,
    /// Only shows log messages with <Error> level. Use twice to completely eliminate output. Takes precidence over verbose
    #[arg(short, long, action = clap::ArgAction::Count)]
    quiet: u8,
    // Do not use default libraries included with program
    //#[arg(short, long)]
    //no_default_libs: bool,
    // Export complete PDF
    //#[arg(short, long)]
    //export_pdf: bool,
}
