use super::template_specification::TemplateSpecification;
// use common::core::file::File;
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Clone, PartialEq)]
pub struct TemplateConfiguration {
    /// The answers to the questions.
    pub answers: HashMap<String, String>,

    /// The list of templates. Each template has a root path, a template specification, and a list of files.
    pub templates: Vec<TemplateConfigurationItem>,
}

impl TemplateConfiguration {
    pub fn new() -> Self {
        Self {
            answers: HashMap::new(),
            templates: Vec::new(),
        }
    }
}

impl Default for TemplateConfiguration {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TemplateConfigurationItem {
    pub root_path: PathBuf,

    /// The template specification.
    pub template_specification: TemplateSpecification,

    /// The list of files.
    pub file_list: Vec<PathBuf>,
}

impl TemplateConfigurationItem {
    pub fn new(root_path: PathBuf, template_specification: TemplateSpecification, file_list: Vec<PathBuf>) -> Self {
        Self {
            root_path,
            template_specification,
            file_list,
        }
    }
}
