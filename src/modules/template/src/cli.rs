use clap::Command;
use common::cli::command::GroupCommands;
use common::cli::functions::handle_subcommand;
use common::core::errors::Result;

use crate::generate::cli::GenerateCliCommand;

pub struct TemplateGroupCommands {}

impl GroupCommands for TemplateGroupCommands {
    fn get_commands(&self) -> Vec<Box<dyn common::cli::command::Command>> {
        vec![Box::new(GenerateCliCommand {})]
    }
}

#[async_trait::async_trait]
impl common::cli::command::Command for TemplateGroupCommands {
    fn get_name(&self) -> &'static str {
        "template"
    }

    async fn execute(&self, args: &clap::ArgMatches) -> Result<()> {
        handle_subcommand(self, args).await?;
        Ok(())
    }

    fn register_cli(&self, cli: clap::Command) -> clap::Command {
        let mut template_group = Command::new(self.get_name())
            .about("template commands")
            .arg_required_else_help(true);

        for command in self.get_commands().iter() {
            template_group = command.register_cli(template_group);
        }

        cli.subcommand(template_group)
    }
}
