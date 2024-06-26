use super::{file_list::FileList, template_specification::TemplateSpecification};

#[derive(Debug, Clone, PartialEq)]
pub struct TemplateConfiguration {
    pub template_specification: TemplateSpecification,
    pub file_list: FileList,
}
