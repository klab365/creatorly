use crate::infrastructure::yaml_templatespecification::YamlTemplateSpecification;
use crate::{application::common::interfaces::ConfigurationLoader, domain::template_specification::TemplateSpecification};

pub struct YamlConfigurationLoader {}

impl ConfigurationLoader for YamlConfigurationLoader {
    fn load_configuration(&self, configuration_path: String) -> Result<TemplateSpecification, String> {
        let file = std::fs::File::open(configuration_path).expect("Unable to open file");
        let value: YamlTemplateSpecification = serde_yaml::from_reader(file).expect("Unable to read file");
        let template: TemplateSpecification = value.into();

        if template.is_valid() {
            return Ok(template);
        }

        Err("Template is invalid! Please check yaml file!".to_string())
    }
}

mod tests {
    use serde_yaml::Value;

    use crate::domain::template_specification::{TemplateSpecificationItem, TemplateSpecificationItemType};

    use super::*;
    use std::{collections::HashMap, hash::Hash};

    #[test]
    fn test_load_configuration() {
        // arrange
        let sut = YamlConfigurationLoader {};
        let mut expected_template: TemplateSpecification = TemplateSpecification::new();
        expected_template.questions.push(TemplateSpecificationItem::new(
            "project_name".to_string(),
            TemplateSpecificationItemType::SingleChoice("DemoBoilerplate".to_string()),
        ));
        expected_template.questions.push(TemplateSpecificationItem::new(
            "description".to_string(),
            TemplateSpecificationItemType::SingleChoice("Demo Boilerplate".to_string()),
        ));
        expected_template.questions.push(TemplateSpecificationItem::new(
            "author_name".to_string(),
            TemplateSpecificationItemType::SingleChoice("Max Muster".to_string()),
        ));
        expected_template.questions.push(TemplateSpecificationItem::new(
            "author_email".to_string(),
            TemplateSpecificationItemType::SingleChoice("max.muster@example.com".to_string()),
        ));
        expected_template.questions.push(TemplateSpecificationItem::new(
            "license".to_string(),
            TemplateSpecificationItemType::MultipleChoice(vec!["MIT".to_string(), "BSD license".to_string()]),
        ));

        // act
        let template_specification = sut
            .load_configuration("/workspaces/creatorly/src/infrastructure/test_data/test_template_spec.yml".to_string())
            .unwrap();

        // assert
        assert_eq!(template_specification, expected_template);
    }
}
