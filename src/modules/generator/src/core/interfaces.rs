use common::core::errors::Result;

#[cfg(test)]
use mockall::automock;
use templatespecification::core::template_specification::TemplateSpecification;

#[cfg_attr(test, automock)]
/// This interface is used to render the input with the given template specification
/// The input can be the content of a file or a path to a file
pub trait TemplateRenderer: Send + Sync {
    /// render the input with the given template specification
    fn render(&self, input: &str, config: &TemplateSpecification) -> Result<String>;
}
