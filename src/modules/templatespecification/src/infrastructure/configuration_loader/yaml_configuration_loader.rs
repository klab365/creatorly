use std::path::Path;

use super::yaml_templatespecification::YamlTemplateSpecification;
use crate::core::{interfaces::ConfigurationLoader, template_specification::TemplateSpecification};
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
        let parsed_value: YamlTemplateSpecification = serde_yaml::from_value(parsed_value.clone())
            .map_err(|e| Error::new(format!("Unable to parse yaml: {}", e)))?;

        let template: TemplateSpecification = parsed_value.into();
        Ok(template)
    }

    #[doc = r" Saves the configuration to the specified path."]
    async fn save_configuration(
        &self,
        configuration_path: &Path,
        template_specification: TemplateSpecification,
    ) -> Result<()> {
        let yaml_template_specification: YamlTemplateSpecification = template_specification.into();
        let yaml_string = serde_yaml::to_string(&yaml_template_specification)
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
    use std::path::PathBuf;

    use crate::core::template_specification::{TemplateSpecificationItem, TemplateSpecificationItemType};

    use super::*;

    #[tokio::test]
    async fn test_load_configuration() {
        let mut yaml_config_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        yaml_config_file.push("resources/test");
        yaml_config_file.push("test_template_spec.yml");

        // arrange
        let sut = YamlConfigurationLoader {};
        let mut expected_template: TemplateSpecification = TemplateSpecification::new();
        let mut template_item = TemplateSpecificationItem::new(
            "project_name".to_string(),
            TemplateSpecificationItemType::SingleChoice("DemoBoilerplate".to_string()),
        );
        template_item.set_answer("DemoBoilerplate".to_string());
        expected_template.placeholders.push(template_item);

        let mut template_item = TemplateSpecificationItem::new(
            "description".to_string(),
            TemplateSpecificationItemType::SingleChoice("Demo Boilerplate".to_string()),
        );
        template_item.set_answer("Demo Boilerplate".to_string());
        expected_template.placeholders.push(template_item);

        let mut template_item = TemplateSpecificationItem::new(
            "author_name".to_string(),
            TemplateSpecificationItemType::SingleChoice("Max Muster".to_string()),
        );
        template_item.set_answer("Max Muster".to_string());
        expected_template.placeholders.push(template_item);

        let mut template_item = TemplateSpecificationItem::new(
            "author_email".to_string(),
            TemplateSpecificationItemType::SingleChoice("max.muster@example.com".to_string()),
        );
        template_item.set_answer("max.muster@example.com".to_string());
        expected_template.placeholders.push(template_item);

        let mut template_item = TemplateSpecificationItem::new(
            "license".to_string(),
            TemplateSpecificationItemType::MultipleChoice(vec!["MIT".to_string(), "BSD".to_string()]),
        );
        template_item.set_answer("MIT".to_string());
        expected_template.placeholders.push(template_item);

        // act
        let template_specification = sut.load_configuration(&yaml_config_file).await.unwrap();

        // assert
        assert_eq!(
            template_specification.placeholders.len(),
            expected_template.placeholders.len()
        );
        for (index, template_item) in template_specification.placeholders.iter().enumerate() {
            assert_eq!(
                template_item.get_template_key(),
                expected_template.placeholders[index].get_template_key()
            );
            match &template_item.get_item() {
                TemplateSpecificationItemType::SingleChoice(choice) => {
                    if let TemplateSpecificationItemType::SingleChoice(expected_choice) =
                        &expected_template.placeholders[index].get_item()
                    {
                        assert_eq!(choice, expected_choice);
                    }
                }
                TemplateSpecificationItemType::MultipleChoice(choices) => {
                    if let TemplateSpecificationItemType::MultipleChoice(expected_choices) =
                        &expected_template.placeholders[index].get_item()
                    {
                        assert_eq!(choices, expected_choices);
                    }
                }
            }
        }
    }
}
