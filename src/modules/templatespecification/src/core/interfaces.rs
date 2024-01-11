use std::path::{Path, PathBuf};

use super::template_specification::TemplateSpecificationItem;
use crate::core::file_list::FileList;
use crate::core::template_specification::TemplateSpecification;
use common::core::errors::Result;
#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait::async_trait]
/// Trait for loading configuration.
pub trait ConfigurationLoader {
    /// Loads the configuration from the specified path.
    ///
    /// # Arguments
    ///
    /// * `configuration_path` - The path to the configuration file.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the loaded `TemplateSpecification` on success,
    /// or an error message as a `String` on failure.
    async fn load_configuration(&self, configuration_path: &Path) -> Result<TemplateSpecification>;

    /// Saves the configuration to the specified path.
    async fn save_configuration(
        &self,
        configuration_path: &Path,
        template_specification: TemplateSpecification,
    ) -> Result<()>;
}

#[cfg_attr(test, automock)]
#[async_trait::async_trait]
/// Trait for loading a file list.
pub trait FileListLoader {
    /// Loads the file list.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the loaded `FileList` if successful, or an error message as a `String` if unsuccessful.
    async fn load(&self, path: Option<PathBuf>) -> Result<FileList>;
}

#[cfg_attr(test, automock)]
/// This interface is used to get the answer of the question
pub trait Prompt {
    /// get the answer of the questions
    fn get_answer(&self, template_specification_item: &mut TemplateSpecificationItem);

    /// get the default answer of the placeholder
    fn get_default_answer(&self, placeholder: &str) -> Result<TemplateSpecificationItem>;
}

#[cfg_attr(test, automock)]
/// This interface is used to render the input with the given template specification
/// The input can be the content of a file or a path to a file
pub trait TemplateRenderer: Send + Sync {
    /// render the input with the given template specification
    fn render(&self, input: &str, config: &TemplateSpecification) -> Result<String>;
}
