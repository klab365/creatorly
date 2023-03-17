#![allow(dead_code)]

use application::create::{
    service::{CreateProjectInput, CreateService},
    template_engine::TemplateEngine,
};
use clap::{command, Args, Parser, Subcommand};
use infrastructure::{
    configuration_loader::yaml_configuration_loader::YamlConfigurationLoader, file_system::FileSystem, folder_loader::local_file_loader::LocalFileLoader,
    logger::setup_logger, prompt::cli_prompt::CliPrompt, template_renderer::liquid_template::LiquidTemplateRenderer,
};
use log::error;

#[derive(Parser)]
#[command(author, version)]
#[command(about = "creatorly - a simple cli to generate projects from templates")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new project from a template
    Create(Create),
}

#[derive(Args)]
struct Create {
    /// The path to the template
    template_path: String,

    /// The name of the destination
    destination_path: String,
}

fn main() {
    setup_logger();

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Create(_create)) => {
            let file_tree_loader: LocalFileLoader = LocalFileLoader::default();
            let configuration_loader: YamlConfigurationLoader = YamlConfigurationLoader {};
            let file_system: FileSystem = FileSystem {};
            let prompt: CliPrompt = CliPrompt {};

            let liquid_template_renderer: LiquidTemplateRenderer = LiquidTemplateRenderer {};
            let template_engine: TemplateEngine = TemplateEngine::new(&liquid_template_renderer, &file_system);

            let input: CreateProjectInput = CreateProjectInput {
                input_path: _create.template_path.trim_end_matches('/').to_string(),
                destination_path: _create.destination_path.trim_end_matches('/').to_string(),
            };
            let service: CreateService = CreateService::new(&file_tree_loader, &configuration_loader, &prompt, &template_engine);
            service.create_project(input).unwrap();
        }
        None => {
            error!("command not found");
        }
    }
}
