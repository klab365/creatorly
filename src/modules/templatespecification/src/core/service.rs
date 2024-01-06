use crate::core::interfaces::{ConfigurationLoader, FileListLoader};
use crate::core::{
    file_list::FileList, template_configuration::TemplateConfiguration, template_specification::TemplateSpecification,
};
use crate::infrastructure::configuration_loader::yaml_configuration_loader::YamlConfigurationLoader;
use crate::infrastructure::folder_loader::git_files_loader::GitFileListLoader;
use crate::infrastructure::folder_loader::local_file_loader::LocalFileListLoader;
use common::core::errors::{Error, Result};
use common::core::file::File;
use std::path::PathBuf;
use std::sync::Arc;

/// This struct represents the service for template specifications.
pub struct TemplateSpecificationService {
    folder_loader: Arc<dyn FileListLoader + Send + Sync>,
    configuration_loader: Arc<dyn ConfigurationLoader + Send + Sync>,
}

impl TemplateSpecificationService {
    pub fn with_local_file_loader() -> Self {
        let folder_loader = Arc::new(LocalFileListLoader::default());
        let configuration_loader = Arc::new(YamlConfigurationLoader::default());

        Self {
            folder_loader,
            configuration_loader,
        }
    }

    pub fn with_git_file_loader(remote_git_url: String, branch_name: String) -> Self {
        let folder_loader = Arc::new(GitFileListLoader::new(remote_git_url, branch_name));
        let configuration_loader = Arc::new(YamlConfigurationLoader::default());

        Self {
            folder_loader,
            configuration_loader,
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
}
