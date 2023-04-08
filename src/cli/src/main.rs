#![allow(dead_code)]
#![warn(unused_extern_crates)]

use clap::{command, Parser, Subcommand};
use create::Create;
use infrastructure::logger::setup_logger;

mod create;

#[derive(Parser)]
#[command(author, version)]
#[command(about = "creatorly - a simple cli to generate projects from templates")]
#[command(arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new project from a template
    Create(Create),
}

fn main() {
    setup_logger();

    let cli = Cli::parse();
    match cli.command {
        Commands::Create(create) => create::parse_command(create),
    }
}
