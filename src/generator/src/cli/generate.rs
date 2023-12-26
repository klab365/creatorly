use clap::{Args, Command, FromArgMatches, Subcommand};
use common::{cli::interface::ICommand, infrastructure::file_system::FileSystem};
use std::sync::Arc;
use templatespecification::core::service::TemplateSpecificationService;

use crate::{
    core::{
        service::{GenerateProjectInput, GenerateService},
        template_engine::TemplateEngine,
    },
    infrastructure::{
        prompt::cli_prompt::CliPrompt, template_renderer::liquid_template_renderer::LiquidTemplateRenderer,
    },
};

/// Represents a command for generating a project from a template.
pub struct GenerateCliCommand {}

#[async_trait::async_trait]
impl ICommand for GenerateCliCommand {
    fn get_name(&self) -> &'static str {
        "generate"
    }

    async fn execute(&self, args: &clap::ArgMatches) -> Result<(), String> {
        let sub_commands = GenerateSubCommands::from_arg_matches(args);
        if let Err(err) = sub_commands {
            return Err(err.to_string());
        }

        let file_system = Arc::new(FileSystem {});
        let prompt = Arc::new(CliPrompt {});
        let liquid_template_renderer = Arc::new(LiquidTemplateRenderer {});
        let template_engine = Arc::new(TemplateEngine::new(liquid_template_renderer, file_system));

        let sub_commands = sub_commands.unwrap();
        match sub_commands {
            GenerateSubCommands::Local(local_create) => {
                let input = GenerateProjectInput {
                    input_path: local_create.template_path.trim_end_matches('/').to_string(),
                    destination_path: local_create.destination_path.trim_end_matches('/').to_string(),
                };
                let template_specification_service = Arc::new(
                    TemplateSpecificationService::new_with_local_file_loader(local_create.template_path),
                );
                let service = GenerateService::new(template_specification_service, prompt, template_engine);
                service.generate_project(input).await.unwrap();
            }
            GenerateSubCommands::Git(git_create) => {
                let input: GenerateProjectInput = GenerateProjectInput {
                    input_path: git_create.remote_path.clone(),
                    destination_path: git_create.destination_path.trim_end_matches('/').to_string(),
                };
                let template_specification_service = Arc::new(TemplateSpecificationService::new_with_git_file_loader(
                    git_create.remote_path,
                    "/tmp/".to_string(),
                    git_create.branch,
                ));
                let service = GenerateService::new(template_specification_service, prompt, template_engine);
                service.generate_project(input).await.unwrap();
            }
        }

        Ok(())
    }

    fn register_cli(&self, cli: clap::Command) -> clap::Command {
        let mut generate_cli = Command::new(self.get_name())
            .about("Generate a project from a template")
            .arg_required_else_help(true);

        generate_cli = GenerateSubCommands::augment_subcommands(generate_cli);
        cli.subcommand(generate_cli)
    }
}

#[derive(Subcommand)]
enum GenerateSubCommands {
    /// Create a new project from a template
    Local(GenerateFromLocal),

    /// Create a new project from a git repository
    Git(GenerateFromGit),
}

#[derive(Args)]
struct GenerateFromLocal {
    /// The path to the template
    #[arg(short, long)]
    template_path: String,

    /// The path to the destination path (it will be created if it does not exist)
    #[arg(short, long)]
    destination_path: String,
}

#[derive(Args)]
struct GenerateFromGit {
    /// The path to the template
    #[arg(short, long)]
    remote_path: String,

    /// The name of the destination
    #[arg(short, long)]
    branch: String,

    /// The path to the destination path (it will be created if it does not exist)
    #[arg(short, long)]
    destination_path: String,
}
