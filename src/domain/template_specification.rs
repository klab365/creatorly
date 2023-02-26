use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_yaml::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateSpecification {
    #[serde(flatten)]
    pub options: HashMap<String, Value>,
}
