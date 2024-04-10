use common::core::errors::{Error, Result};
use common::core::file::File;
use common::core::user_interaction_interface::UserInteraction;
use std::path::PathBuf;
use std::sync::Arc;

use crate::templatespecification::infrastructure::configuration_loader::yaml_configuration_loader::YamlConfigurationLoader;
use crate::templatespecification::infrastructure::folder_loader::git_files_loader::GitFileListLoader;
use crate::templatespecification::infrastructure::folder_loader::local_file_loader::LocalFileListLoader;

use super::file_list::FileList;
use super::interfaces::{ConfigurationLoader, FileListLoader};
use super::template_configuration::TemplateConfiguration;
use super::template_specification::{TemplateSpecification, TemplateSpecificationItem, TemplateSpecificationItemType};

/// This struct represents the service for template specifications.
pub struct TemplateSpecificationService {
    folder_loader: Arc<dyn FileListLoader + Send + Sync>,
    configuration_loader: Arc<dyn ConfigurationLoader + Send + Sync>,
    user_interaction_interface: Arc<dyn UserInteraction>,
}

impl TemplateSpecificationService {
    pub fn with_local_file_loader(user_interaction_interface: Arc<dyn UserInteraction>) -> Self {
        let folder_loader = Arc::new(LocalFileListLoader::default());
        let configuration_loader = Arc::new(YamlConfigurationLoader::default());

        Self {
            folder_loader,
            configuration_loader,
            user_interaction_interface,
        }
    }

    pub fn with_git_file_loader(
        user_interaction_interface: Arc<dyn UserInteraction>,
        remote_git_url: String,
        branch_name: String,
    ) -> Self {
        let folder_loader = Arc::new(GitFileListLoader::new(remote_git_url, branch_name));
        let configuration_loader = Arc::new(YamlConfigurationLoader::default());

        Self {
            folder_loader,
            configuration_loader,
            user_interaction_interface,
        }
    }

    pub async fn load_template_configuration(&self, path: Option<PathBuf>) -> Result<TemplateConfiguration> {
        let mut files = self.load_files(path).await?;
        let template_specification = self.get_template_specification(&mut files).await?;
        if template_specification.placeholders.is_empty() {
            return Err(Error::new(
                "No placeholder template items found in template specification".into(),
            ));
        }

        Ok(TemplateConfiguration {
            file_list: files,
            template_specification,
        })
    }

    pub async fn save_template_specification(
        &self,
        path: PathBuf,
        template_specification: TemplateSpecification,
    ) -> Result<()> {
        let path = path.join("creatorly.yml");
        self.configuration_loader
            .save_configuration(&path, template_specification)
            .await?;
        Ok(())
    }

    pub async fn get_template_specification(&self, files: &mut FileList) -> Result<TemplateSpecification> {
        let found_creatorly_file =
            files.files.iter().enumerate().find(|file| {
                file.1.contains(File::from("creatorly.yaml")) || file.1.contains(File::from("creatorly.yml"))
            });

        let Some(found_creatorly_file) = found_creatorly_file else {
            return Err(Error::new("creatorly.yaml not found".into()));
        };

        let found_creatorly_file_idx = found_creatorly_file.0;
        let found_creatorly_file = found_creatorly_file.1;
        let specification = self
            .configuration_loader
            .load_configuration(found_creatorly_file.path())
            .await?;

        files.files.remove(found_creatorly_file_idx);

        Ok(specification)
    }

    pub async fn load_files(&self, path: Option<PathBuf>) -> Result<FileList> {
        let files = self.folder_loader.load(path).await?;
        if files.files.is_empty() {
            return Err(Error::with_advice(
                "No files found".into(),
                "Please run the command in a directory with files".into(),
            ));
        }

        Ok(files)
    }

    /// get the answer of the questions
    pub async fn get_answers(&self, template_specification: &mut TemplateSpecification) -> Result<()> {
        for template_specification_item in &mut template_specification.placeholders {
            self.parse_answer(template_specification_item).await?;
        }

        Ok(())
    }

    async fn parse_answer(&self, template_specification_item: &mut TemplateSpecificationItem) -> Result<()> {
        match &template_specification_item.get_item() {
            TemplateSpecificationItemType::SingleChoice(choice) => {
                let prompt = format!("{}: ", template_specification_item.template_key);
                let answer = self.user_interaction_interface.get_input(&prompt, choice).await?;
                if answer.is_empty() {
                    template_specification_item.set_answer(choice.to_string());
                    return Ok(());
                }

                template_specification_item.set_answer(answer);
            }
            TemplateSpecificationItemType::MultipleChoice(choices) => {
                let prompt = format!("{}: ", template_specification_item.template_key);
                let answer = self.user_interaction_interface.get_selection(&prompt, choices).await?;
                template_specification_item.set_answer(answer);
            }
        }

        Ok(())
    }

    /// get the default answer of the questions
    pub async fn get_default_answer(&self, placeholder: &str) -> Result<TemplateSpecificationItem> {
        let answer = self
            .user_interaction_interface
            .get_input(&format!("{}: ", placeholder), "")
            .await?;

        let cleaned_answer: Vec<String> = answer.split(',').map(|s| s.trim().to_string()).collect();
        let item = match cleaned_answer.len() {
            0 => Err(Error::new("No answer provided".into())),
            1 => Ok(TemplateSpecificationItem::new(
                placeholder.to_string(),
                TemplateSpecificationItemType::SingleChoice(cleaned_answer[0].clone()),
            )),
            _ => Ok(TemplateSpecificationItem::new(
                placeholder.to_string(),
                TemplateSpecificationItemType::MultipleChoice(cleaned_answer.clone()),
            )),
        };

        let item = item?;
        Ok(item)
    }
}
