#![allow(dead_code)]
#![warn(unused_extern_crates)]

use check::cli::CheckCliCommand;
use clap::{command, Args};
use common::{
    cli::{cli_user_interaction_interface::CliUserInteractionInterface, interface::ICommand},
    core::{errors::Result, user_interaction_interface::UserInteractionInterface},
};
use create::cli::CreateCommand;
use generate::cli::generate::GenerateCliCommand;

#[tokio::main]
async fn main() {
    let mut app = CliApp::new();
    app.register_command(Box::new(GenerateCliCommand {}));
    app.register_command(Box::new(CreateCommand {}));
    app.register_command(Box::new(CheckCliCommand {}));

    let res = app.parse().await;
    match res {
        Ok(_) => {}
        Err(_) => {
            std::process::exit(1);
        }
    }
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
    pub async fn parse(&self) -> Result<()> {
        let cli = self.build_cli();
        let matches = cli.get_matches();

        self.handle_argmatches(&matches).await
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
    async fn handle_argmatches(&self, matches: &clap::ArgMatches) -> Result<()> {
        for command in self.commands.iter() {
            let subcommad_name = matches.subcommand_name().expect("No subcommand found");
            if command.get_name() == subcommad_name {
                let res = command
                    .execute(matches.subcommand_matches(subcommad_name).unwrap())
                    .await;

                if let Err(ref err) = res {
                    let user_interaction_interface = CliUserInteractionInterface {};
                    user_interaction_interface.print_error(err.to_string().as_str()).await;
                }

                return res;
            }
        }

        Ok(())
    }
}
