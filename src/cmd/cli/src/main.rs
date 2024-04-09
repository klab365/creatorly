#![warn(unused_extern_crates)]

use clap::{command, Args};
use common::{
    cli::{
        functions::handle_subcommand,
        interface::{ICommand, IGroupCommands},
    },
    core::errors::Result,
};
use template::cli::TemplateGroupCommands;

#[tokio::main]
async fn main() {
    let app = CliApp::new();

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

struct CliApp {}

impl IGroupCommands for CliApp {
    fn get_commands(&self) -> Vec<Box<dyn ICommand>> {
        vec![Box::new(TemplateGroupCommands {})]
    }
}

/// Implementation of the `CliApp` struct.
impl CliApp {
    pub fn new() -> Self {
        Self {}
    }

    /// Parses the command line arguments and handles them accordingly.
    pub async fn parse(&self) -> Result<()> {
        let cli = self.build_cli();
        let args = cli.get_matches();

        handle_subcommand(self, &args).await
    }

    /// Builds the command line interface using `clap` crate.
    ///
    /// # Returns
    ///
    /// The built `clap::Command` instance.
    fn build_cli(&self) -> clap::Command {
        let mut cli = clap::Command::default();
        cli = CliAppRootArgs::augment_args(cli);

        for command in self.get_commands().iter() {
            cli = command.register_cli(cli);
        }

        cli
    }
}
