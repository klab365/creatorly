use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// The template specification. It contains the questions, which are asked.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct TemplateSpecification {
    /// Represents the placeholder id. For example "CREATORLY".
    #[serde(skip_serializing_if = "Option::is_none")]
    placeholder_id: Option<String>,

    /// Represents the placeholder delimiter. For example "." (CREATORLY.xyz).
    #[serde(skip_serializing_if = "Option::is_none")]
    placeholder_delimiter: Option<String>,

    /// Represents a list of questions for a template specification.
    pub placeholders: IndexMap<String, TemplateSpecificationItemType>,
}

impl TemplateSpecification {
    pub const PREFIX: &'static str = "CREATORLY";
    pub const DELIMITER: &'static str = ".";

    /// Creates a new instance of the template specification with the default placeholder id.
    pub fn new() -> Self {
        Self {
            placeholder_id: None,
            placeholder_delimiter: None,
            placeholders: IndexMap::new(),
        }
    }

    /// Creates a new instance of the template specification with the given placeholder id.
    pub fn from_id_delimiter(placeholder_id: String, delimeter: String) -> Self {
        Self {
            placeholder_id: Some(placeholder_id),
            placeholder_delimiter: Some(delimeter),
            placeholders: IndexMap::new(),
        }
    }

    pub fn get_placeholder_id(&self) -> String {
        match self.placeholder_id {
            Some(ref id) => id.clone(),
            None => Self::PREFIX.to_string(),
        }
    }

    pub fn get_placeholder_delimiter(&self) -> String {
        match self.placeholder_delimiter {
            Some(ref delimeter) => delimeter.clone(),
            None => Self::DELIMITER.to_string(),
        }
    }
}

impl Default for TemplateSpecification {
    fn default() -> Self {
        Self::new()
    }
}

/// The type of the template specification item.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum TemplateSpecificationItemType {
    /// A single choice item.
    SingleChoice(String),

    /// A multiple choice item.
    MultipleChoice(Vec<String>),
}
