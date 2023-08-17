use std::sync::Arc;

use super::{
    file_list::FileList,
    interfaces::{ConfigurationLoader, FileListLoader, Prompt},
    template_engine::TemplateEngine,
    template_specification::TemplateSpecification,
};
use crate::generate::template_engine::RenderPushArgument;
use log::info;

pub struct GenerateProjectInput {
    pub input_path: String,
    pub destination_path: String,
}

/// Structure for the create service
pub struct GenerateService {
    folder_loader: Arc<dyn FileListLoader>,
    configuration_loader: Arc<dyn ConfigurationLoader>,
    prompt: Arc<dyn Prompt>,
    template_engine: Arc<TemplateEngine>,
}

impl GenerateService {
    pub fn new(
        folder_loader: Arc<dyn FileListLoader>,
        configuration_loader: Arc<dyn ConfigurationLoader>,
        prompt: Arc<dyn Prompt>,
        template_engine: Arc<TemplateEngine>,
    ) -> Self {
        Self {
            folder_loader,
            configuration_loader,
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

        // load file list
        let mut files = self.load_files()?;

        // load template specification
        let mut template_configuration = self.get_template_configuration(&mut files)?;
        if template_configuration.questions.is_empty() {
            return Err("No placeholder template items found in template specification".to_string());
        }

        // parse answer for question
        self.parse_answer_for_questions(&mut template_configuration);

        // render files and push it to the destination folder
        let args = RenderPushArgument {
            input_root_path: input.input_path.clone(),
            destination_path: input.destination_path,
            file_list: files,
            template_specification: template_configuration,
        };
        self.template_engine.render_and_push(args).await?;

        info!("🎉 project created!");
        Ok(())
    }

    // get template configuration from the template path and remove it from the file list
    fn get_template_configuration(&self, files: &mut FileList) -> Result<TemplateSpecification, String> {
        let found_creatorly_file = files
            .files
            .iter()
            .enumerate()
            .find(|file| file.1.contains("creatorly.yaml") || file.1.contains("creatorly.yml"));

        if found_creatorly_file.is_none() {
            return Err("creatorly.yaml not found".to_string());
        }

        let found_file = found_creatorly_file.unwrap();
        let specification = self.configuration_loader.load_configuration(found_file.1.to_string())?;

        files.files.remove(found_file.0);

        Ok(specification)
    }

    fn load_files(&self) -> Result<FileList, String> {
        let files = self.folder_loader.load()?;
        info!("found {} files on template project", files.files.len());
        Ok(files)
    }

    fn parse_answer_for_questions(&self, template_specification: &mut TemplateSpecification) {
        for item in &mut template_specification.questions {
            self.prompt.get_answer(item)
        }
    }
}
