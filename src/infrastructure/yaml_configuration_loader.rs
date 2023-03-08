use crate::infrastructure::yaml_templatespecification::YamlTemplateSpecification;
use crate::{application::common::interfaces::ConfigurationLoader, domain::template_specification::TemplateSpecification};

pub struct YamlConfigurationLoader {}

impl ConfigurationLoader for YamlConfigurationLoader {
    fn load_configuration(&self, configuration_path: String) -> Result<TemplateSpecification, String> {
        let file = std::fs::File::open(configuration_path).expect("Unable to open file");
        let value: YamlTemplateSpecification = serde_yaml::from_reader(file).expect("Unable to read file");
        if value.questions.is_empty() {
            return Err("No questions found in template specification".to_string());
        }

        let template: TemplateSpecification = value.into();
        Ok(template)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::template_specification::{TemplateSpecificationItem, TemplateSpecificationItemType};

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
            TemplateSpecificationItemType::MultipleChoice(vec!["MIT".to_string(), "BSD".to_string()]),
        ));

        // act
        let template_specification = sut.load_configuration("src/infrastructure/test_data/test_template_spec.yml".to_string()).unwrap();

        // assert
        for (index, template_item) in template_specification.questions.iter().enumerate() {
            assert_eq!(template_item.template_key, expected_template.questions[index].template_key);
            match &template_item.item {
                TemplateSpecificationItemType::SingleChoice(choice) => match &expected_template.questions[index].item {
                    TemplateSpecificationItemType::SingleChoice(expected_choice) => {
                        assert_eq!(choice, expected_choice);
                    }
                    _ => {}
                },
                TemplateSpecificationItemType::MultipleChoice(choices) => match &expected_template.questions[index].item {
                    TemplateSpecificationItemType::MultipleChoice(expected_choices) => {
                        assert_eq!(choices, expected_choices);
                    }
                    _ => {}
                },
            }
        }
    }
}
