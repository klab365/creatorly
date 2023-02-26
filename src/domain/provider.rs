use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Provider {
    pub name: String,
    pub options: HashMap<String, String>,
}
