use crate::core::file_list::FileList;
use crate::core::template_specification::TemplateSpecification;
#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
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
    fn load_configuration(&self, configuration_path: String) -> Result<TemplateSpecification, String>;
}

#[cfg_attr(test, automock)]
/// Trait for loading a file list.
pub trait FileListLoader {
    /// Loads the file list.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the loaded `FileList` if successful, or an error message as a `String` if unsuccessful.
    fn load(&self) -> Result<FileList, String>;
}
