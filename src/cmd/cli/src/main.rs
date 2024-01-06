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
use clap::{command, Args};

use common::{cli::interface::ICommand, infrastructure::logger::setup_logger};
use create::cli::CreateCommand;
use generator::cli::generate::GenerateCliCommand;

#[tokio::main]
async fn main() {
    setup_logger();

    let mut app = CliApp::new();
    app.register_command(Box::new(GenerateCliCommand {}));
    app.register_command(Box::new(CreateCommand {}));
    app.parse().await;
}

#[derive(Args)]
#[command(author, version)]
#[command(about = "creatorly - a simple cli to generate projects from templates")]
#[command(arg_required_else_help = true)]
struct CliAppRootArgs {}

struct CliApp {
    commands: Vec<Box<dyn ICommand>>,
}

/// Implementation of the `CliApp` struct.
impl CliApp {
    /// Creates a new instance of `CliApp`.
    pub fn new() -> Self {
        Self { commands: vec![] }
    }

    /// Registers a command with the `CliApp`.
    ///
    /// # Arguments
    ///
    /// * `command` - A boxed trait object representing the command to be registered.
    pub fn register_command(&mut self, command: Box<dyn ICommand>) {
        self.commands.push(command);
    }

    /// Parses the command line arguments and handles them accordingly.
    pub async fn parse(&self) {
        let cli = self.build_cli();
        let matches = cli.get_matches();
        self.handle_argmatches(&matches).await;
    }

    /// Builds the command line interface using `clap` crate.
    ///
    /// # Returns
    ///
    /// The built `clap::Command` instance.
    fn build_cli(&self) -> clap::Command {
        let mut cli = clap::Command::default();
        cli = CliAppRootArgs::augment_args(cli);

        for command in self.commands.iter() {
            cli = command.register_cli(cli);
        }

        cli
    }

    /// Handles the matched command line arguments.
    ///
    /// # Arguments
    ///
    /// * `matches` - The matched command line arguments.
    async fn handle_argmatches(&self, matches: &clap::ArgMatches) {
        for command in self.commands.iter() {
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
}
