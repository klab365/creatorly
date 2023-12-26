use crate::core::interfaces::{ConfigurationLoader, FileListLoader};
use crate::core::{
    file_list::FileList, template_configuration::TemplateConfiguration, template_specification::TemplateSpecification,
};
use crate::infrastructure::configuration_loader::yaml_configuration_loader::YamlConfigurationLoader;
use crate::infrastructure::folder_loader::git_files_loader::GitFileListLoader;
use crate::infrastructure::folder_loader::local_file_loader::LocalFileListLoader;
use log::info;
use std::sync::Arc;

/// This struct represents the service for template specifications.
pub struct TemplateSpecificationService {
    folder_loader: Arc<dyn FileListLoader + Send + Sync>,
    configuration_loader: Arc<dyn ConfigurationLoader + Send + Sync>,
}

impl TemplateSpecificationService {
    pub fn new_with_local_file_loader(local_path: String) -> Self {
        let folder_loader = Arc::new(LocalFileListLoader::new(local_path));
        let configuration_loader = Arc::new(YamlConfigurationLoader::default());

        Self {
            folder_loader,
            configuration_loader,
        }
    }

    pub fn new_with_git_file_loader(remote_git_url: String, local_storage_path: String, branch_name: String) -> Self {
        let folder_loader = Arc::new(GitFileListLoader::new(remote_git_url, local_storage_path, branch_name));
        let configuration_loader = Arc::new(YamlConfigurationLoader::default());

        Self {
            folder_loader,
            configuration_loader,
        }
    }

    pub fn load_template_configuration(&self) -> Result<TemplateConfiguration, String> {
        let mut files = self.load_files()?;

        let template_specification = self.get_template_specification(&mut files)?;
        if template_specification.questions.is_empty() {
            return Err("No placeholder template items found in template specification".to_string());
        }

        Ok(TemplateConfiguration {
            file_list: files,
            template_specification,
        })
    }

    fn get_template_specification(&self, files: &mut FileList) -> Result<TemplateSpecification, String> {
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
}
