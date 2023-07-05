#[cfg(test)]
use mockall::automock;

use super::{
    file_list::FileList,
    template_specification::{TemplateSpecification, TemplateSpecificationItem},
};

#[cfg_attr(test, automock)]
pub trait ConfigurationLoader {
    fn load_configuration(&self, configuration_path: String) -> Result<TemplateSpecification, String>;
}

// This interface is used to load the file tree from the given path
#[cfg_attr(test, automock)]
pub trait FileListLoader {
    fn load(&self) -> Result<FileList, String>;
}

/// This interface is used to render the input with the given template specification
/// The input can be the content of a file or a path to a file
#[cfg_attr(test, automock)]
pub trait TemplateRenderer: Send + Sync {
    /// render the input with the given template specification
    fn render(&self, input: String, config: TemplateSpecification) -> Result<String, String>;
}

/// This interface is used to get the answer of the question
#[cfg_attr(test, automock)]
pub trait Prompt {
    /// get the answer of the questions
    fn get_answer(&self, template_specification_item: &mut TemplateSpecificationItem);
}
