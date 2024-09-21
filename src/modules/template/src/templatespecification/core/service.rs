use super::interfaces::{ConfigurationLoader, FileListLoader};
use super::sort_by_directory_structure;
use super::template_configuration::{TemplateConfiguration, TemplateConfigurationItem};
use super::template_specification::{TemplateSpecification, TemplateSpecificationItemType};
use super::validate_template::validate_template_configuration;
use common::core::errors::{Error, Result};
use common::core::user_interaction_interface::UserInteraction;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// This struct represents the service for template specifications.
pub struct TemplateSpecificationService {
    folder_loader: Arc<dyn FileListLoader + Send + Sync>,
    configuration_loader: Arc<dyn ConfigurationLoader + Send + Sync>,
    user_interaction_interface: Arc<dyn UserInteraction>,
}

impl TemplateSpecificationService {
    pub fn new(
        folder_loader: Arc<dyn FileListLoader + Send + Sync>,
        configuration_loader: Arc<dyn ConfigurationLoader + Send + Sync>,
        user_interaction_interface: Arc<dyn UserInteraction>,
    ) -> Self {
        Self {
            folder_loader,
            configuration_loader,
            user_interaction_interface,
        }
    }

    pub async fn load_template_configuration(&self, entry_point_path: &Path) -> Result<TemplateConfiguration> {
        let files = self.load_files(entry_point_path).await?;

        let mut template_configuration = TemplateConfiguration::new();

        // get the template configuration items
        for file in files {
            let temp_config_item = self.get_template_configuration_item(&file).await?;
            template_configuration.templates.push(temp_config_item)
        }

        validate_template_configuration(&template_configuration)?;

        Ok(template_configuration)
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

    async fn get_template_configuration_item(
        &self,
        files: &(PathBuf, Vec<PathBuf>),
    ) -> Result<TemplateConfigurationItem> {
        let specification = self.configuration_loader.load_configuration(&files.0).await?;

        let template_config_item = TemplateConfigurationItem::new(files.0.clone(), specification, files.1.clone());

        Ok(template_config_item)
    }

    /// load the files
    ///
    /// returns a vector of tuples where the first element is the creatorly file and the second element is
    /// a vector of files associated with the creatorly file
    async fn load_files(&self, path: &Path) -> Result<Vec<(PathBuf, Vec<PathBuf>)>> {
        let files = self.folder_loader.load(path).await?;
        if files.is_empty() {
            return Err(Error::with_advice(
                "No files found".into(),
                "Please run the command in a directory with files".into(),
            ));
        }

        // separate creatorly files from other files
        let mut other_files = Vec::new();
        let mut creatorly_files = Vec::new();
        for file in files {
            if file.ends_with("creatorly.yaml") || file.ends_with("creatorly.yml") {
                creatorly_files.push(file);
            } else {
                other_files.push(file);
            }
        }

        sort_by_directory_structure(&mut creatorly_files);
        sort_by_directory_structure(&mut other_files);
        let mut results = Vec::new();
        for creatorly_file in &creatorly_files {
            results.push((creatorly_file.clone(), Vec::new()));
        }

        // parse the files and group them to the nearest creatorly file
        for other_file in other_files {
            let mut found_creatorly_file: Option<&Path> = None;
            for result in &creatorly_files {
                let Some(creatorly_file_parent) = result.parent() else {
                    continue;
                };

                if other_file.starts_with(creatorly_file_parent) {
                    found_creatorly_file = Some(result.as_path());
                }
            }

            let Some(found_creatorly_file) = found_creatorly_file else {
                return Err(Error::new(format!(
                    "No creatorly file found for file {}",
                    other_file.display()
                )));
            };

            for result in &mut results {
                if result.0 == found_creatorly_file {
                    result.1.push(other_file.clone());
                }
            }
        }

        Ok(results)
    }

    /// get the answer of the questions
    pub async fn get_answers(&self, template_configuration: &mut TemplateConfiguration) -> Result<()> {
        template_configuration.answers.clear();

        for template_configuration_item in &template_configuration.templates {
            for (key, template_specification_item) in &template_configuration_item.template_specification.placeholders {
                let is_key_present = template_configuration.answers.contains_key(key);
                if is_key_present {
                    continue;
                }

                let answer = match &template_specification_item {
                    TemplateSpecificationItemType::SingleChoice(choice) => {
                        let prompt = format!("{}: ", key);
                        let mut answer = self.user_interaction_interface.get_input(&prompt, choice).await?;
                        if answer.is_empty() {
                            answer = choice.clone();
                        }

                        answer
                    }
                    TemplateSpecificationItemType::MultipleChoice(choices) => {
                        let prompt = format!("{}: ", key);
                        self.user_interaction_interface.get_selection(&prompt, choices).await?
                    }
                };

                template_configuration.answers.insert(key.clone(), answer.to_string());
            }
        }

        Ok(())
    }

    /// get the default answer of the questions
    pub async fn get_default_answer(&self, placeholder: &str) -> Result<TemplateSpecificationItemType> {
        let answer = self
            .user_interaction_interface
            .get_input(&format!("{}: ", placeholder), "")
            .await?;

        let cleaned_answer: Vec<String> = answer.split(',').map(|s| s.trim().to_string()).collect();
        let item = match cleaned_answer.len() {
            0 => Err(Error::new("No answer provided".into())),
            1 => Ok(TemplateSpecificationItemType::SingleChoice(cleaned_answer[0].clone())),
            _ => Ok(TemplateSpecificationItemType::MultipleChoice(cleaned_answer.clone())),
        };

        let item = item?;
        Ok(item)
    }
}

#[cfg(test)]
mod tests {
    use crate::templatespecification::{
        core::{interfaces::MockConfigurationLoader, service::TemplateSpecificationService},
        infrastructure::folder_loader::local_file_loader::LocalFileListLoader,
    };
    use async_trait::async_trait;
    use common::core::errors::Result;
    use common::core::user_interaction_interface::UserInteraction;
    use mockall::mock;
    use std::{fs, sync::Arc};
    use tempdir::TempDir;

    mock! {
        UserInteractionInterface {}

        #[async_trait]
        impl UserInteraction for UserInteractionInterface {
            async fn print_success(&self, message: &str);

            async fn print_error(&self, message: &str);

            async fn print(&self, message: &str);

            async fn get_input(&self, prompt: &str, default: &str) -> Result<String>;

            async fn get_selection(&self, prompt: &str, choices: &[String]) -> Result<String>;
        }
    }

    #[tokio::test]
    async fn test_load_files_should_return_correct_grouped_files() {
        // arrange
        let sut = TemplateSpecificationService::new(
            Arc::new(LocalFileListLoader::new()),
            Arc::new(MockConfigurationLoader::new()),
            Arc::new(MockUserInteractionInterface::new()),
        );

        let temp_dir = TempDir::new("test_scan_directories").expect("Failed to create temp dir");
        let temp_path = temp_dir.path();
        let creatorly1 = temp_path.join("creatorly.yml");
        let file1 = temp_path.join("file1.txt");
        let docs_dir = temp_path.join("docs");
        let creatorly2 = docs_dir.join("creatorly.yml");
        let file2 = docs_dir.join("file2.txt");
        let sub_sub_dir = docs_dir.join("subsubdir");
        let file3 = sub_sub_dir.join("file3.txt");
        let file4 = sub_sub_dir.join("file4.txt");
        let file5 = temp_path.join("file5.txt");
        let sub_sub_sub_dir = sub_sub_dir.join("subsubsubdir");
        let creatorly3 = sub_sub_sub_dir.join("creatorly.yml");
        let file6 = sub_sub_sub_dir.join("file6.txt");
        let src_dir = temp_path.join("src");
        let file7 = src_dir.join("file7.txt");
        let zfile8 = temp_path.join("zfile8.txt");

        // Write some dummy files
        fs::create_dir_all(&src_dir).expect("Failed to create src dir");
        fs::create_dir_all(&sub_sub_sub_dir).expect("Failed to create subdir/subsubdir");
        std::fs::File::create(&creatorly1).expect("Failed to create creatorly.yml");
        std::fs::File::create(&file1).expect("Failed to create file1.txt");
        std::fs::File::create(&creatorly2).expect("Failed to create subdir/creatorly.yml");
        std::fs::File::create(&file2).expect("Failed to create file2.txt");
        std::fs::File::create(&file3).expect("Failed to create subdir/subsubdir/file3.txt");
        std::fs::File::create(&file4).expect("Failed to create subdir/subsubdir/file4.txt");
        std::fs::File::create(&file5).expect("Failed to create file5.txt");
        std::fs::File::create(&creatorly3).expect("Failed to create subdir/subsubdir/subsubsubdir/creatorly.yml");
        std::fs::File::create(&file6).expect("Failed to create subdir/subsubdir/subsubsubdir/file6.txt");
        std::fs::File::create(&file7).expect("Failed to create src/file7.txt");
        std::fs::File::create(&zfile8).expect("Failed to create zfile8.txt");

        // act
        let results = sut.load_files(temp_path).await.expect("Failed to load files");

        // assert
        assert_eq!(results.len(), 3);

        assert_eq!(results[0].0, creatorly1);
        assert!(results[0].1.contains(&file1));
        assert!(results[0].1.contains(&file5));
        assert!(results[0].1.contains(&file7));
        assert!(results[0].1.contains(&zfile8));

        assert_eq!(results[1].0, creatorly2);
        assert!(results[1].1.contains(&file2));
        assert!(results[1].1.contains(&file3));
        assert!(results[1].1.contains(&file4));

        assert_eq!(results[2].0, creatorly3);
        assert!(results[2].1.contains(&file6));
    }
}
