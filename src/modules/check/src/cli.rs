use std::path::PathBuf;
use std::sync::Arc;

use crate::logic::{CheckService, CheckServiceArgs};
use clap::{Args, Command, FromArgMatches};
use common::core::errors::{Error, Result};
use common::{cli::interface::ICommand, infrastructure::file_system::FileSystem};
use templatespecification::core::{service::TemplateSpecificationService, template_engine::TemplateEngine};

/// Represents a command for generating a project from a template.
pub struct CheckCliCommand {}

#[derive(Args)]
struct CheckCliArgs {
    /// The directory where should start
    template_path: PathBuf,
}

#[async_trait::async_trait]
impl ICommand for CheckCliCommand {
    fn get_name(&self) -> &'static str {
        "check"
    }

    async fn execute(&self, args: &clap::ArgMatches) -> Result<()> {
        let check_args = CheckCliArgs::from_arg_matches(args)
            .map_err(|e| Error::new(format!("issue to parse check args: {}", e)))?;

        let file_system = Arc::new(FileSystem::default());
        let template_engine = Arc::new(TemplateEngine::new_with_liquid_template_renderer(file_system));
        let template_specification_service = Arc::new(TemplateSpecificationService::with_local_file_loader());
        let service = CheckService::new(template_specification_service, template_engine);

        let args = CheckServiceArgs {
            entry_dir: check_args.template_path,
        };
        let res = service.check(&args).await;

        match res {
            Ok(res) => {
                if !res.has_issues() {
                    log::info!("Template is valid");
                } else {
                    log::error!("Template is not valid with this messages:");
                    for issue in res.issues {
                        println!("{}\n", issue);
                    }
                }
            }
            Err(err) => {
                log::error!("Error: {}", err);
            }
        }

        Ok(())
    }

    fn register_cli(&self, cli: clap::Command) -> clap::Command {
        let mut check_cli = Command::new(self.get_name())
            .about("Check a template")
            .arg_required_else_help(true);

        check_cli = CheckCliArgs::augment_args(check_cli);
        cli.subcommand(check_cli)
    }
}