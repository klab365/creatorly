use crate::domain::{file_tree::FileList, template_specification::TemplateSpecificationItem};

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait FileTreeLoader {
    fn load(&self, path: &str) -> Result<FileList, String>;
}

#[cfg_attr(test, automock)]
pub trait TemplateRenderer {
    fn render(&self);
}

#[cfg_attr(test, automock)]
pub trait Prompt {
    // get the answer of the question
    fn get_answer(&self, template_specification_item: &mut TemplateSpecificationItem);
}
