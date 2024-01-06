use std::path::PathBuf;
use std::sync::Arc;

use super::template_engine::TemplateEngine;
use crate::core::template_engine::RenderPushArgument;
use common::core::errors::Result;
use log::{info, warn};
use templatespecification::core::interfaces::Prompt;
use templatespecification::core::service::TemplateSpecificationService;
use templatespecification::core::template_specification::TemplateSpecification;

/// Represents the input parameters for generating a project.
pub struct GenerateProjectInput {
    pub dry_run: bool,
    /// The path of the input file or directory.
    pub input_path: Option<PathBuf>,
    /// The path where the generated project will be saved.
    pub destination_path: PathBuf,
}

/// Structure for the create service
pub struct GenerateService {
    template_specification_service: Arc<TemplateSpecificationService>,
    prompt: Arc<dyn Prompt + Send + Sync>,
    template_engine: Arc<TemplateEngine>,
}

impl GenerateService {
    pub fn new(
        template_specification_service: Arc<TemplateSpecificationService>,
        prompt: Arc<dyn Prompt + Send + Sync>,
        template_engine: Arc<TemplateEngine>,
    ) -> Self {
        Self {
            template_specification_service,
            prompt,
            template_engine,
        }
    }

    /// Create a project from a given template
    pub async fn generate_project(&self, input: GenerateProjectInput) -> Result<()> {
        if input.dry_run {
            warn!("🚀 dry run");
        }

        let input_path = input.input_path;

        let mut template_configuration = self
            .template_specification_service
            .load_template_configuration(input_path.clone())
            .await?;

        info!(
            "found {} files on template project",
            template_configuration.file_list.files.len()
        );

        // parse answer for question
        self.parse_answer_for_questions(&mut template_configuration.template_specification);

        // render files and push it to the destination folder
        info!("🚀 render files and push it to the destination folder");
        let args = RenderPushArgument {
            input_root_path: template_configuration.clone().file_list.root_path,
            destination_path: input.destination_path,
            file_list: template_configuration.file_list,
            template_specification: template_configuration.template_specification,
            dry_run: input.dry_run,
        };
        self.template_engine.render_and_push(args).await?;

        if input.dry_run {
            info!("🎉 project created!");
        }

        Ok(())
    }

    fn parse_answer_for_questions(&self, template_specification: &mut TemplateSpecification) {
        info!("📝 parse answer for questions");
        for item in &mut template_specification.placeholders {
            self.prompt.get_answer(item)
        }
    }
}
