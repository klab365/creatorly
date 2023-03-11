use crate::domain::template_specification::TemplateSpecification;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait ConfigurationLoader {
    fn load_configuration(&self, configuration_path: String) -> Result<TemplateSpecification, String>;
}

// interface for the file system
pub trait Os {
    fn clear_folder(&self, path: String) -> Result<(), String>;

    fn move_file(&self, source_path: String, target_path: String) -> Result<(), String>;
}
