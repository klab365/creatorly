use std::path::Path;

use crate::templatespecification::core::{
    interfaces::ConfigurationLoader, template_specification::TemplateSpecification,
};

use common::core::errors::{Error, Result};
use serde_yaml::Value;

#[derive(Default)]
pub struct YamlConfigurationLoader {}

#[async_trait::async_trait]
impl ConfigurationLoader for YamlConfigurationLoader {
    async fn load_configuration(&self, configuration_path: &Path) -> Result<TemplateSpecification> {
        let contents = tokio::fs::read_to_string(configuration_path)
            .await
            .map_err(|e| Error::new(format!("Unable to read file: {}", e)))?;

        let parsed_value: Value =
            serde_yaml::from_str(&contents).map_err(|e| Error::new(format!("Unable to parse file: {}", e)))?;

        let parsed_value: TemplateSpecification = serde_yaml::from_value(parsed_value.clone())
            .map_err(|e| Error::new(format!("Unable to parse yaml: {}", e)))?;

        Ok(parsed_value)
    }

    #[doc = r" Saves the configuration to the specified path."]
    async fn save_configuration(
        &self,
        configuration_path: &Path,
        template_specification: TemplateSpecification,
    ) -> Result<()> {
        let yaml_string = serde_yaml::to_string(&template_specification)
            .map_err(|_| Error::new("Unable to serialize template specification".into()))?;

        tokio::fs::write(configuration_path, yaml_string).await.map_err(|e| {
            let msg = format!("Unable to write template specification to file: {}", e);
            Error::new(msg)
        })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_load_configuration() {
        let mut yaml_config_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        yaml_config_file.push("resources/test");
        yaml_config_file.push("creatorly_test.yml");

        // arrange
        let sut = YamlConfigurationLoader {};

        // act
        let template_specification = sut.load_configuration(&yaml_config_file).await.unwrap();

        // assert
        assert_eq!(template_specification.placeholders.len(), 5);
        assert!(template_specification.placeholders.contains_key("project_name"));
        assert!(template_specification.placeholders.contains_key("description"));
        assert!(template_specification.placeholders.contains_key("author_name"));
        assert!(template_specification.placeholders.contains_key("author_email"));
        assert!(template_specification.placeholders.contains_key("license"));
    }
}
