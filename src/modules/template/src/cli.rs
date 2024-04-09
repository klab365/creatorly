use clap::Command;
use common::cli::functions::handle_subcommand;
use common::cli::interface::{ICommand, IGroupCommands};
use common::core::errors::Result;

use crate::{check::cli::CheckCliCommand, create::cli::CreateCommand, generate::cli::generate::GenerateCliCommand};

pub struct TemplateGroupCommands {}

impl IGroupCommands for TemplateGroupCommands {
    fn get_commands(&self) -> Vec<Box<dyn ICommand>> {
        vec![
            Box::new(CheckCliCommand {}),
            Box::new(CreateCommand {}),
            Box::new(GenerateCliCommand {}),
        ]
    }
}

#[async_trait::async_trait]
impl ICommand for TemplateGroupCommands {
    fn get_name(&self) -> &'static str {
        "template"
    }

    async fn execute(&self, args: &clap::ArgMatches) -> Result<()> {
        handle_subcommand(self, args).await?;
        Ok(())
    }

    fn register_cli(&self, cli: clap::Command) -> clap::Command {
        let mut template_group = Command::new(self.get_name()).about("template commands");

        for command in self.get_commands().iter() {
            template_group = command.register_cli(template_group);
        }

        cli.subcommand(template_group)
    }
}
