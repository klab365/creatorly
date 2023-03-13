#![allow(dead_code)]

mod application;
mod domain;
mod infrastructure;

use application::create::{
    service::{CreateProjectInput, Service},
    template_engine::TemplateEngine,
};
use clap::{command, Args, Parser, Subcommand};
use infrastructure::{
    cli_prompt::CliPrompt, file_system::FileSystem, folder_loader::local_file_loader::LocalFileLoader, liquid_template::LiquidTemplateRenderer,
    yaml_configuration_loader::YamlConfigurationLoader,
};

#[derive(Parser)]
#[command(author, version)]
#[command(about = "creatorly - a simple cli to generate repos from templates", long_about = "")]

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
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Create(_create)) => {
            let file_tree_loader: LocalFileLoader = LocalFileLoader::new();
            let configuration_loader: YamlConfigurationLoader = YamlConfigurationLoader {};
            let file_system: FileSystem = FileSystem {};
            let prompt: CliPrompt = CliPrompt {};

            let liquid_template_renderer: LiquidTemplateRenderer = LiquidTemplateRenderer {};
            let template_engine: TemplateEngine = TemplateEngine::new(&liquid_template_renderer, &file_system);

            let input: CreateProjectInput = CreateProjectInput {
                input_path: _create.template_path,
                destination_path: _create.destination_path,
            };
            let service: Service = Service::new(&file_tree_loader, &configuration_loader, &file_system, &prompt, &template_engine);

            service.create_project(input).unwrap();
        }
        None => {
            println!("command not found");
        }
    }
}
