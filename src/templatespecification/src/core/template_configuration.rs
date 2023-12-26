use crate::core::{file_list::FileList, template_specification::TemplateSpecification};

pub struct TemplateConfiguration {
    pub template_specification: TemplateSpecification,
    pub file_list: FileList,
}
