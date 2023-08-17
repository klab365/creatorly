#![allow(dead_code)]
#![warn(unused_extern_crates)]

use clap::{command, Parser, Subcommand};
use generate::Generate;
use infrastructure::logger::setup_logger;

mod generate;

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
    /// Generate a new project from a template
    Generate(Generate),
}

#[tokio::main]
async fn main() {
    setup_logger();

    let cli = Cli::parse();
    match cli.command {
        Commands::Generate(generate) => generate::parse_command(generate).await,
    }
}
