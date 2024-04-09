use std::path::PathBuf;
use std::sync::Arc;

use common::core::errors::Result;
use common::core::user_interaction_interface::UserInteractionInterface;

use crate::templatespecification::core::service::TemplateSpecificationService;
use crate::templatespecification::core::template_configuration::TemplateConfiguration;
use crate::templatespecification::core::template_engine::{CheckTemplateArgs, RenderPushArgument, TemplateEngine};
use crate::templatespecification::core::template_specification::TemplateSpecification;

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
    template_engine: Arc<TemplateEngine>,
    user_interaction_interface: Arc<dyn UserInteractionInterface>,
}

impl GenerateService {
    pub fn new(
        template_specification_service: Arc<TemplateSpecificationService>,
        template_engine: Arc<TemplateEngine>,
        user_interaction_interface: Arc<dyn UserInteractionInterface>,
    ) -> Self {
        Self {
            template_specification_service,
            template_engine,
            user_interaction_interface,
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
        let msg = format!(
            "found {} files on template project",
            template_configuration.file_list.files.len()
        );
        self.user_interaction_interface.print_success(&msg).await;

        // parse answer for question
        self.parse_answer_for_questions(&mut template_configuration.template_specification)
            .await?;

        // render files and push it to the destination folder
        self.user_interaction_interface
            .print("🚀 render files and push it to the destination folder")
            .await;
        let args = RenderPushArgument {
            destination_path: input.destination_path,
            template_configuration,
        };
        self.template_engine.render_and_push(args).await?;

        Ok(())
    }

    async fn parse_answer_for_questions(&self, template_specification: &mut TemplateSpecification) -> Result<()> {
        self.user_interaction_interface
            .print("📝 parse answer for questions")
            .await;

        self.template_specification_service
            .get_answers(template_specification)
            .await?;

        Ok(())
    }

    async fn check_template_configuration(&self, templatespecification: &TemplateConfiguration) -> Result<()> {
        self.user_interaction_interface
            .print("🔎 check if the template is valid")
            .await;
        let check_template_args = CheckTemplateArgs {
            template_configuration: templatespecification.clone(),
        };
        let res_check_template = self.template_engine.check_template(&check_template_args).await?;
        if !res_check_template.is_valid() {
            return Err(res_check_template.into());
        }
        self.user_interaction_interface.print_success("template is valid").await;
        Ok(())
    }
}
