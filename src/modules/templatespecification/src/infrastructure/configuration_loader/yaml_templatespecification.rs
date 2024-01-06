use crate::core::template_specification::{
    TemplateSpecification, TemplateSpecificationItem, TemplateSpecificationItemType,
};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;

/// This struct represents the template specification in yaml format. I need this struct to
/// be able to serialize and deserialize the template specification.
#[derive(Debug, Serialize, Deserialize)]
pub struct YamlTemplateSpecification {
    pub placeholders: IndexMap<String, Value>,
}

impl From<YamlTemplateSpecification> for TemplateSpecification {
    fn from(yaml_template_specification: YamlTemplateSpecification) -> Self {
        let mut template = TemplateSpecification::new();

        for (key, value) in yaml_template_specification.placeholders {
            match value {
                Value::String(choice) => {
                    template.placeholders.push(TemplateSpecificationItem::new(
                        key,
                        TemplateSpecificationItemType::SingleChoice(choice),
                    ));
                }
                Value::Sequence(choices) => {
                    let mut choices_tmp: Vec<String> = Vec::new();
                    for choice in choices {
                        if let Value::String(answer) = choice {
                            choices_tmp.push(answer);
                        }
                    }
                    template.placeholders.push(TemplateSpecificationItem::new(
                        key,
                        TemplateSpecificationItemType::MultipleChoice(choices_tmp),
                    ));
                }
                _ => {}
            }
        }

        template
    }
}

impl From<TemplateSpecification> for YamlTemplateSpecification {
    fn from(val: TemplateSpecification) -> YamlTemplateSpecification {
        let mut yaml_template_specification = YamlTemplateSpecification {
            placeholders: IndexMap::new(),
        };

        for question in val.placeholders {
            match question.item {
                TemplateSpecificationItemType::SingleChoice(choice) => {
                    yaml_template_specification
                        .placeholders
                        .insert(question.template_key, Value::String(choice));
                }
                TemplateSpecificationItemType::MultipleChoice(choices) => {
                    yaml_template_specification.placeholders.insert(
                        question.template_key,
                        Value::Sequence(choices.into_iter().map(Value::String).collect()),
                    );
                }
            }
        }

        yaml_template_specification
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yaml_template_specification_to_template_spec() {
        let mut yaml_template_specification = YamlTemplateSpecification {
            placeholders: IndexMap::new(),
        };
        yaml_template_specification
            .placeholders
            .insert("project_name".to_string(), Value::String("DemoBoilerplate".to_string()));
        yaml_template_specification.placeholders.insert(
            "project_type".to_string(),
            Value::Sequence(vec![Value::String("web".to_string()), Value::String("cli".to_string())]),
        );

        let template_specification: TemplateSpecification = yaml_template_specification.into();

        assert_eq!(template_specification.placeholders.len(), 2);
        assert_eq!(
            template_specification.placeholders[0].get_template_key(),
            "project_name"
        );
        assert_eq!(
            template_specification.placeholders[0].get_single_choice().unwrap(),
            "DemoBoilerplate"
        );
        assert_eq!(
            template_specification.placeholders[1].get_template_key(),
            "project_type"
        );
        assert_eq!(
            template_specification.placeholders[1]
                .get_multiple_choice()
                .unwrap()
                .len(),
            2
        );
    }

    #[test]
    fn test_template_specification_from_yaml_template_specification() {
        let mut yaml_template_specification = YamlTemplateSpecification {
            placeholders: IndexMap::new(),
        };
        yaml_template_specification
            .placeholders
            .insert("project_name".to_string(), Value::String("DemoBoilerplate".to_string()));
        yaml_template_specification.placeholders.insert(
            "project_type".to_string(),
            Value::Sequence(vec![Value::String("web".to_string()), Value::String("cli".to_string())]),
        );

        let template_specification: TemplateSpecification = TemplateSpecification::from(yaml_template_specification);

        assert_eq!(template_specification.placeholders.len(), 2);
        assert_eq!(
            template_specification.placeholders[0].get_template_key(),
            "project_name"
        );
        assert_eq!(
            template_specification.placeholders[0].get_single_choice().unwrap(),
            "DemoBoilerplate"
        );
        assert_eq!(
            template_specification.placeholders[1].get_template_key(),
            "project_type"
        );
        assert_eq!(
            template_specification.placeholders[1]
                .get_multiple_choice()
                .unwrap()
                .len(),
            2
        );
    }
}
