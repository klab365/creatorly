#![allow(dead_code)]
#![warn(unused_extern_crates)]

/// This is the main entry point of the Creatorly CLI application.
/// It sets up the logger, handles the command line interface, and executes the specified command.
/// The CLI application uses the clap crate for command line argument parsing.
/// The available commands are defined as implementations of the `ICommand` trait.
/// The `handle_cli` function retrieves the list of commands, creates the CLI object, and matches the subcommand.
/// It then executes the corresponding command and logs any errors that occur.
/// The `get_commands` function returns a vector of command objects, currently only containing the `GenerateCliCommand`.
/// The `get_cli` function constructs the CLI object by augmenting the default clap::Command with arguments from the `Cli` struct
/// and registering each command from the list of commands.
/// The `main` function sets up the logger and calls the `handle_cli` function to start the CLI application.
/// It is marked as `async` to allow for asynchronous execution using the `tokio` runtime.
/// The `tokio::main` attribute is used to run the `main` function.
use clap::{command, Args, Parser};

use common::{cli::interface::ICommand, infrastructure::logger::setup_logger};
use generator::cli::generate::GenerateCliCommand;

#[tokio::main]
async fn main() {
    setup_logger();
    handle_cli().await;
}

/// This function handles the command line interface.
async fn handle_cli() {
    let commands = get_commands();
    let cli = get_cli(&commands);
    let matches = cli.get_matches();
    for command in commands.iter() {
        let subcommad_name = matches.subcommand_name().expect("No subcommand found");
        if command.get_name() == subcommad_name {
            let res = command
                .execute(matches.subcommand_matches(subcommad_name).unwrap())
                .await;

            if let Err(err) = res {
                log::error!("{}", err)
            }
        }
    }
}

/// This function returns a vector of command objects.
fn get_commands() -> Vec<Box<dyn ICommand>> {
    vec![Box::new(GenerateCliCommand {})]
}

#[derive(Parser)]
#[command(author, version)]
#[command(about = "creatorly - a simple cli to generate projects from templates")]
#[command(arg_required_else_help = true)]
struct Cli {}

/// This function returns the CLI object.
fn get_cli(commands: &[Box<dyn ICommand>]) -> clap::Command {
    let mut cli = clap::Command::default();
    cli = Cli::augment_args(cli);

    for command in commands.iter() {
        cli = command.register_cli(cli);
    }

    cli
}
