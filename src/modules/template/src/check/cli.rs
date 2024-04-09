use std::path::PathBuf;
use std::sync::Arc;

use clap::{Args, Command, FromArgMatches};
use common::cli::cli_user_interaction_interface::CliUserInteractionInterface;
use common::core::errors::{Error, Result};
use common::core::user_interaction_interface::UserInteractionInterface;
use common::{cli::interface::ICommand, infrastructure::file_system::FileSystem};

use crate::templatespecification::core::service::TemplateSpecificationService;
use crate::templatespecification::core::template_engine::TemplateEngine;

use super::logic::{CheckService, CheckServiceArgs};

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
        // prepare
        let check_args = CheckCliArgs::from_arg_matches(args)
            .map_err(|e| Error::new(format!("issue to parse check args: {}", e)))?;

        let user_interaction_interface = Arc::new(CliUserInteractionInterface {});
        let file_system = Arc::new(FileSystem::default());
        let template_engine = Arc::new(TemplateEngine::new_with_liquid_template_renderer(
            file_system,
            user_interaction_interface.clone(),
        ));
        let template_specification_service = Arc::new(TemplateSpecificationService::with_local_file_loader(
            user_interaction_interface.clone(),
        ));
        let service = CheckService::new(template_specification_service, template_engine);

        // execute
        let args = CheckServiceArgs {
            entry_dir: check_args.template_path,
        };
        let res = service.check(&args).await;

        // handle result
        match res {
            Ok(res) => {
                if !res.has_issues() {
                    user_interaction_interface.print_success("Template is valid").await;
                    return Ok(());
                }

                user_interaction_interface
                    .print_error("Template is not valid with this messages:")
                    .await;

                for issue in res.issues {
                    let msg = format!("{}\n", issue);
                    user_interaction_interface.print(&msg).await;
                }

                return Err(Error::from("Template is not valid"));
            }

            Err(e) => {
                user_interaction_interface
                    .print_error("An error occurred while checking the template")
                    .await;

                return Err(e);
            }
        }
    }

    fn register_cli(&self, cli: clap::Command) -> clap::Command {
        let mut check_cli = Command::new(self.get_name())
            .about("Check a template")
            .arg_required_else_help(true);

        check_cli = CheckCliArgs::augment_args(check_cli);
        cli.subcommand(check_cli)
    }
}
