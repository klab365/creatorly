use std::{path::PathBuf, sync::Arc};

use clap::{Args, FromArgMatches};
use common::{
    cli::cli_user_interaction_interface::CliUserInteraction, cli::command::Command, core::errors::Error,
    core::errors::Result, infrastructure::file_system::FileSystem,
};

use crate::templatespecification::core::service::TemplateSpecificationService;

use super::service::{CreateTemplateArgs, CreateTemplateSpecificationService};

pub struct CreateCommand {}

#[derive(Args, Clone, Debug)]
struct CreateTemplateSpecificationArgs {
    /// The directory where should start
    entry_dir: PathBuf,
}

#[async_trait::async_trait]
impl Command for CreateCommand {
    fn get_name(&self) -> &'static str {
        "create"
    }

    async fn execute(&self, args: &clap::ArgMatches) -> Result<()> {
        let args = CreateTemplateSpecificationArgs::from_arg_matches(args);
        if let Err(err) = args {
            return Err(Error::new(err.to_string()));
        }

        let args = args.unwrap();
        handle_create_template(args).await?;
        Ok(())
    }

    fn register_cli(&self, cli: clap::Command) -> clap::Command {
        let mut create_cli = clap::Command::new(self.get_name())
            .about("Create different things")
            .arg_required_else_help(true);

        create_cli = CreateTemplateSpecificationArgs::augment_args(create_cli);
        cli.subcommand(create_cli)
    }
}

async fn handle_create_template(template_args: CreateTemplateSpecificationArgs) -> Result<()> {
    let file_system = Arc::new(FileSystem::default());
    let interface = Arc::new(CliUserInteraction {});
    let templatespecification_service =
        Arc::new(TemplateSpecificationService::with_local_file_loader(interface.clone()));
    let create_service =
        CreateTemplateSpecificationService::new(templatespecification_service, file_system, interface.clone());

    let args = CreateTemplateArgs {
        entry_dir: template_args.entry_dir.clone(),
    };
    create_service.create_template_specification(args).await?;

    Ok(())
}
