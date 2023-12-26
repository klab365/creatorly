use std::sync::Arc;

use super::{interfaces::Prompt, template_engine::TemplateEngine};
use crate::core::template_engine::RenderPushArgument;
use log::info;
use templatespecification::core::service::TemplateSpecificationService;
use templatespecification::core::template_specification::TemplateSpecification;

/// Represents the input parameters for generating a project.
pub struct GenerateProjectInput {
    /// The path of the input file or directory.
    pub input_path: String,
    /// The path where the generated project will be saved.
    pub destination_path: String,
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
    pub async fn generate_project(&self, input: GenerateProjectInput) -> Result<(), String> {
        if input.input_path.is_empty() {
            return Err("path is empty".to_string());
        }

        if input.destination_path.is_empty() {
            return Err("destination path is empty".to_string());
        }

        let mut template_configuration = self.template_specification_service.load_template_configuration()?;

        // parse answer for question
        self.parse_answer_for_questions(&mut template_configuration.template_specification);

        // render files and push it to the destination folder
        info!("🚀 render files and push it to the destination folder");
        let args = RenderPushArgument {
            input_root_path: input.input_path.clone(),
            destination_path: input.destination_path,
            file_list: template_configuration.file_list,
            template_specification: template_configuration.template_specification,
        };
        self.template_engine.render_and_push(args).await?;

        info!("🎉 project created!");
        Ok(())
    }

    fn parse_answer_for_questions(&self, template_specification: &mut TemplateSpecification) {
        info!("📝 parse answer for questions");
        for item in &mut template_specification.questions {
            self.prompt.get_answer(item)
        }
    }
}
