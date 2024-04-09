use std::{path::PathBuf, sync::Arc};

use clap::{Args, Command, FromArgMatches, Subcommand};
use common::{
    cli::cli_user_interaction_interface::CliUserInteractionInterface, cli::interface::ICommand, core::errors::Error,
    core::errors::Result, infrastructure::file_system::FileSystem,
};

use crate::templatespecification::core::service::TemplateSpecificationService;

use super::service::{CreateTemplateArgs, CreateTemplateSpecificationService};

pub struct CreateCommand {}

#[derive(Subcommand, Debug)]
enum CreateSubCommands {
    /// Create a template based on a directory
    Template(CreateTemplateSpecificationArgs),
}

#[derive(Args, Clone, Debug)]
struct CreateTemplateSpecificationArgs {
    /// The directory where should start
    entry_dir: PathBuf,
}

#[async_trait::async_trait]
impl ICommand for CreateCommand {
    fn get_name(&self) -> &'static str {
        "create"
    }

    async fn execute(&self, args: &clap::ArgMatches) -> Result<()> {
        let subcommands = CreateSubCommands::from_arg_matches(args);
        if let Err(err) = subcommands {
            return Err(Error::new(err.to_string()));
        }

        let subcommands = subcommands.unwrap();
        match subcommands {
            CreateSubCommands::Template(template) => {
                handle_create_template(template.clone()).await?;
            }
        }

        Ok(())
    }

    fn register_cli(&self, cli: clap::Command) -> clap::Command {
        let mut create_cli = Command::new(self.get_name())
            .about("Create different things")
            .arg_required_else_help(true);

        create_cli = CreateSubCommands::augment_subcommands(create_cli);
        cli.subcommand(create_cli)
    }
}

async fn handle_create_template(template: CreateTemplateSpecificationArgs) -> Result<()> {
    let file_system = Arc::new(FileSystem::default());
    let interface = Arc::new(CliUserInteractionInterface {});
    let templatespecification_service =
        Arc::new(TemplateSpecificationService::with_local_file_loader(interface.clone()));
    let create_service =
        CreateTemplateSpecificationService::new(templatespecification_service, file_system, interface.clone());

    let args = CreateTemplateArgs {
        entry_dir: template.entry_dir.clone(),
    };
    create_service.create_template_specification(args).await?;

    Ok(())
}
