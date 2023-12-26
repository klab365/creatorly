use super::yaml_templatespecification::YamlTemplateSpecification;
use crate::core::{interfaces::ConfigurationLoader, template_specification::TemplateSpecification};
use serde_yaml::Value;

#[derive(Default)]
pub struct YamlConfigurationLoader {}

impl ConfigurationLoader for YamlConfigurationLoader {
    fn load_configuration(&self, configuration_path: String) -> Result<TemplateSpecification, String> {
        let file = std::fs::File::open(configuration_path).expect("Unable to open file");
        let parsed_value: Value = serde_yaml::from_reader(file).expect("Unable to read file");
        let parsed_value = parsed_value
            .get("placeholders")
            .ok_or_else(|| "No creatorly key found in template specification".to_string())?;

        let parsed_value: YamlTemplateSpecification =
            serde_yaml::from_value(parsed_value.clone()).expect("Unable to parse file");
        if parsed_value.questions.is_empty() {
            return Err("No questions found in template specification".to_string());
        }

        let template: TemplateSpecification = parsed_value.into();
        Ok(template)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::core::template_specification::{TemplateSpecificationItem, TemplateSpecificationItemType};

    use super::*;

    #[test]
    fn test_load_configuration() {
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
        expected_template.questions.push(template_item);

        let mut template_item = TemplateSpecificationItem::new(
            "description".to_string(),
            TemplateSpecificationItemType::SingleChoice("Demo Boilerplate".to_string()),
        );
        template_item.set_answer("Demo Boilerplate".to_string());
        expected_template.questions.push(template_item);

        let mut template_item = TemplateSpecificationItem::new(
            "author_name".to_string(),
            TemplateSpecificationItemType::SingleChoice("Max Muster".to_string()),
        );
        template_item.set_answer("Max Muster".to_string());
        expected_template.questions.push(template_item);

        let mut template_item = TemplateSpecificationItem::new(
            "author_email".to_string(),
            TemplateSpecificationItemType::SingleChoice("max.muster@example.com".to_string()),
        );
        template_item.set_answer("max.muster@example.com".to_string());
        expected_template.questions.push(template_item);

        let mut template_item = TemplateSpecificationItem::new(
            "license".to_string(),
            TemplateSpecificationItemType::MultipleChoice(vec!["MIT".to_string(), "BSD".to_string()]),
        );
        template_item.set_answer("MIT".to_string());
        expected_template.questions.push(template_item);

        // act
        let template_specification = sut
            .load_configuration(yaml_config_file.to_string_lossy().to_string())
            .unwrap();

        // assert
        assert_eq!(
            template_specification.questions.len(),
            expected_template.questions.len()
        );
        for (index, template_item) in template_specification.questions.iter().enumerate() {
            assert_eq!(
                template_item.get_template_key(),
                expected_template.questions[index].get_template_key()
            );
            match &template_item.get_item() {
                TemplateSpecificationItemType::SingleChoice(choice) => {
                    if let TemplateSpecificationItemType::SingleChoice(expected_choice) =
                        &expected_template.questions[index].get_item()
                    {
                        assert_eq!(choice, expected_choice);
                    }
                }
                TemplateSpecificationItemType::MultipleChoice(choices) => {
                    if let TemplateSpecificationItemType::MultipleChoice(expected_choices) =
                        &expected_template.questions[index].get_item()
                    {
                        assert_eq!(choices, expected_choices);
                    }
                }
            }
        }
    }
}
