use clap::{Args, Command, FromArgMatches, Subcommand};
use common::core::errors::{Error, Result};
use common::{cli::interface::ICommand, infrastructure::file_system::FileSystem};
use std::path::PathBuf;
use std::sync::Arc;
use templatespecification::{core::service::TemplateSpecificationService, infrastructure::cli_prompt::CliPrompt};

use crate::core::{
    service::{GenerateProjectInput, GenerateService},
    template_engine::TemplateEngine,
};
use crate::infrastructure::liquid_template_renderer::LiquidTemplateRenderer;

/// Represents a command for generating a project from a template.
pub struct GenerateCliCommand {}

#[async_trait::async_trait]
impl ICommand for GenerateCliCommand {
    fn get_name(&self) -> &'static str {
        "generate"
    }

    async fn execute(&self, args: &clap::ArgMatches) -> Result<()> {
        let generate_args = GenerateArgs::from_arg_matches(args)
            .map_err(|e| Error::new(format!("issue to parse generate args: {}", e)))?;

        let file_system = Arc::new(FileSystem {});
        let prompt = Arc::new(CliPrompt {});
        let liquid_template_renderer = Arc::new(LiquidTemplateRenderer {});
        let template_engine = Arc::new(TemplateEngine::new(liquid_template_renderer, file_system));

        let dry_run = generate_args.dry_run;
        let sub_command = generate_args.command;
        match sub_command {
            GenerateSubCommands::Local(local_create) => {
                let input = GenerateProjectInput {
                    dry_run,
                    input_path: Some(local_create.template_path),
                    destination_path: local_create.destination_path,
                };
                let template_specification_service = Arc::new(TemplateSpecificationService::with_local_file_loader());
                let service = GenerateService::new(template_specification_service, prompt, template_engine);
                service.generate_project(input).await?;
            }
            GenerateSubCommands::Git(git_create) => {
                let input: GenerateProjectInput = GenerateProjectInput {
                    dry_run,
                    input_path: git_create.input_path,
                    destination_path: git_create.destination_path,
                };
                let template_specification_service = Arc::new(TemplateSpecificationService::with_git_file_loader(
                    git_create.remote_path,
                    git_create.branch,
                ));
                let service = GenerateService::new(template_specification_service, prompt, template_engine);
                service.generate_project(input).await?;
            }
        }

        Ok(())
    }

    fn register_cli(&self, cli: clap::Command) -> clap::Command {
        let mut generate_cli = Command::new(self.get_name())
            .about("Generate a project from a template")
            .arg_required_else_help(true);

        generate_cli = GenerateArgs::augment_args(generate_cli);
        cli.subcommand(generate_cli)
    }
}

#[derive(Args)]
struct GenerateArgs {
    /// If set, the command will not create any files
    #[arg(short, long)]
    dry_run: bool,

    #[command(subcommand)]
    command: GenerateSubCommands,
}

#[derive(Subcommand)]
enum GenerateSubCommands {
    /// Check if the template is valid
    /// Create a new project from a template
    Local(GenerateFromLocal),

    /// Create a new project from a git repository
    Git(GenerateFromGit),
}

#[derive(Args)]
struct GenerateFromLocal {
    /// The path to the template
    #[arg(short, long)]
    template_path: PathBuf,

    /// The path to the destination path (it will be created if it does not exist)
    #[arg(short, long)]
    destination_path: PathBuf,
}

#[derive(Args)]
struct GenerateFromGit {
    /// The path to the template
    #[arg(short, long)]
    remote_path: String,

    /// The name of the destination
    #[arg(short, long)]
    branch: String,

    /// The path to the template, if not specified, the root directory of the git repo will be used
    #[arg(short, long)]
    input_path: Option<PathBuf>,

    /// The path to the destination path (it will be created if it does not exist)
    #[arg(short, long)]
    destination_path: PathBuf,
}
