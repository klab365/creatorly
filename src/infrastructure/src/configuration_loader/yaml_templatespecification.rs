use application::generate::template_specification::{
    TemplateSpecification, TemplateSpecificationItem, TemplateSpecificationItemType,
};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct YamlTemplateSpecification {
    #[serde(flatten)]
    pub questions: IndexMap<String, Value>,
}

impl From<YamlTemplateSpecification> for TemplateSpecification {
    fn from(yaml_template_specification: YamlTemplateSpecification) -> Self {
        let mut template = TemplateSpecification::new();

        for (key, value) in yaml_template_specification.questions {
            match value {
                Value::String(choice) => {
                    template.questions.push(TemplateSpecificationItem::new(
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
                    template.questions.push(TemplateSpecificationItem::new(
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yaml_template_specification_to_template_spec() {
        let mut yaml_template_specification = YamlTemplateSpecification {
            questions: IndexMap::new(),
        };
        yaml_template_specification
            .questions
            .insert("project_name".to_string(), Value::String("DemoBoilerplate".to_string()));
        yaml_template_specification.questions.insert(
            "project_type".to_string(),
            Value::Sequence(vec![Value::String("web".to_string()), Value::String("cli".to_string())]),
        );

        let template_specification: TemplateSpecification = yaml_template_specification.into();

        assert_eq!(template_specification.questions.len(), 2);
        assert_eq!(template_specification.questions[0].get_template_key(), "project_name");
        assert_eq!(
            template_specification.questions[0].get_single_choice().unwrap(),
            "DemoBoilerplate"
        );
        assert_eq!(template_specification.questions[1].get_template_key(), "project_type");
        assert_eq!(
            template_specification.questions[1].get_multiple_choice().unwrap().len(),
            2
        );
    }

    #[test]
    fn test_template_specification_from_yaml_template_specification() {
        let mut yaml_template_specification = YamlTemplateSpecification {
            questions: IndexMap::new(),
        };
        yaml_template_specification
            .questions
            .insert("project_name".to_string(), Value::String("DemoBoilerplate".to_string()));
        yaml_template_specification.questions.insert(
            "project_type".to_string(),
            Value::Sequence(vec![Value::String("web".to_string()), Value::String("cli".to_string())]),
        );

        let template_specification: TemplateSpecification = TemplateSpecification::from(yaml_template_specification);

        assert_eq!(template_specification.questions.len(), 2);
        assert_eq!(template_specification.questions[0].get_template_key(), "project_name");
        assert_eq!(
            template_specification.questions[0].get_single_choice().unwrap(),
            "DemoBoilerplate"
        );
        assert_eq!(template_specification.questions[1].get_template_key(), "project_type");
        assert_eq!(
            template_specification.questions[1].get_multiple_choice().unwrap().len(),
            2
        );
    }
}
