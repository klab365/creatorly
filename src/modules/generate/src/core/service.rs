use std::path::PathBuf;
use std::sync::Arc;

use common::core::errors::Result;
use log::info;
use templatespecification::core::interfaces::Prompt;
use templatespecification::core::service::TemplateSpecificationService;
use templatespecification::core::template_configuration::TemplateConfiguration;
use templatespecification::core::template_engine::{CheckTemplateArgs, RenderPushArgument, TemplateEngine};
use templatespecification::core::template_specification::TemplateSpecification;

/// Represents the input parameters for generating a project.
pub struct GenerateProjectInput {
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
        let input_path = input.input_path;

        // load template configuration
        let mut template_configuration = self
            .template_specification_service
            .load_template_configuration(input_path.clone())
            .await?;

        // check if the template is valid
        self.check_template_configuration(&template_configuration).await?;
        info!(
            "found {} files on template project",
            template_configuration.file_list.files.len()
        );

        // parse answer for question
        self.parse_answer_for_questions(&mut template_configuration.template_specification);

        // render files and push it to the destination folder
        info!("🚀 render files and push it to the destination folder");
        let args = RenderPushArgument {
            destination_path: input.destination_path,
            template_configuration,
        };
        self.template_engine.render_and_push(args).await?;

        Ok(())
    }

    fn parse_answer_for_questions(&self, template_specification: &mut TemplateSpecification) {
        info!("📝 parse answer for questions");
        for item in &mut template_specification.placeholders {
            self.prompt.get_answer(item)
        }
    }

    async fn check_template_configuration(&self, templatespecification: &TemplateConfiguration) -> Result<()> {
        info!("🔎 check if the template is valid");
        let check_template_args = CheckTemplateArgs {
            template_configuration: templatespecification.clone(),
        };
        let res_check_template = self.template_engine.check_template(&check_template_args).await?;
        if !res_check_template.is_valid() {
            return Err(res_check_template.into());
        }
        info!("✅ template is valid");
        Ok(())
    }
}
