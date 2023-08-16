use application::generate::{
    service::{GenerateProjectInput, GenerateService},
    template_engine::TemplateEngine,
};
use clap::{command, Args, Subcommand};
use infrastructure::{
    configuration_loader::yaml_configuration_loader::YamlConfigurationLoader, file_system::FileSystem,
    folder_loader::git_files_loader::GitFileListLoader, folder_loader::local_file_loader::LocalFileListLoader,
    prompt::cli_prompt::CliPrompt, template_renderer::liquid_template_renderer::LiquidTemplateRenderer,
};
use std::sync::Arc;

/// Create cli command
#[derive(Args)]
pub struct Generate {
    /// command
    #[command(subcommand)]
    sub_command: GenerateSubCommands,
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

// parse create commands
pub async fn parse_command(generate: Generate) {
    let configuration_loader = Arc::new(YamlConfigurationLoader {});
    let file_system = Arc::new(FileSystem {});
    let prompt = Arc::new(CliPrompt {});
    let liquid_template_renderer = Arc::new(LiquidTemplateRenderer {});
    let template_engine = Arc::new(TemplateEngine::new(liquid_template_renderer, file_system));

    match generate.sub_command {
        GenerateSubCommands::Local(local_create) => {
            let local_file_list_loader = Arc::new(LocalFileListLoader::new(local_create.template_path.clone()));
            let input = GenerateProjectInput {
                input_path: local_create.template_path.trim_end_matches('/').to_string(),
                destination_path: local_create.destination_path.trim_end_matches('/').to_string(),
            };
            let service = GenerateService::new(local_file_list_loader, configuration_loader, prompt, template_engine);
            service.generate_project(input).await.unwrap();
        }
        GenerateSubCommands::Git(git_create) => {
            let git_file_list_loader = Arc::new(GitFileListLoader::new(
                git_create.remote_path.clone(),
                "/tmp/".to_string(),
                git_create.branch,
            ));
            let input: GenerateProjectInput = GenerateProjectInput {
                input_path: git_create.remote_path,
                destination_path: git_create.destination_path.trim_end_matches('/').to_string(),
            };
            let service = GenerateService::new(git_file_list_loader, configuration_loader, prompt, template_engine);
            service.generate_project(input).await.unwrap();
        }
    }
}
