#![allow(dead_code)]

use clap::{command, Parser, Subcommand};
use create::Create;
use infrastructure::logger::setup_logger;
use log::error;

mod create;

#[derive(Parser)]
#[command(author, version)]
#[command(about = "creatorly - a simple cli to generate projects from templates")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
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
        Some(Commands::Create(create)) => create::parse_command(create),
        None => {
            error!("command not found");
        }
    }
}
