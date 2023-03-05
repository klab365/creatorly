use std::collections::HashMap;

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;

use crate::domain::template_specification::{TemplateSpecification, TemplateSpecificationItem, TemplateSpecificationItemType};

#[derive(Debug, Serialize, Deserialize)]
pub struct YamlTemplateSpecification {
    #[serde(flatten)]
    pub questions: IndexMap<String, Value>,
}

impl Into<TemplateSpecification> for YamlTemplateSpecification {
    fn into(self) -> TemplateSpecification {
        let mut template = TemplateSpecification::new();

        for (key, value) in self.questions {
            match value {
                Value::String(choice) => {
                    template.questions.push(TemplateSpecificationItem::new(key, TemplateSpecificationItemType::SingleChoice(choice)));
                }
                Value::Sequence(choices) => {
                    let mut choices_tmp: Vec<String> = Vec::new();
                    for choice in choices {
                        match choice {
                            Value::String(answer) => {
                                choices_tmp.push(answer);
                            }
                            _ => {}
                        }
                    }
                    template
                        .questions
                        .push(TemplateSpecificationItem::new(key, TemplateSpecificationItemType::MultipleChoice(choices_tmp)));
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
        let mut yaml_template_specification = YamlTemplateSpecification { questions: IndexMap::new() };
        yaml_template_specification
            .questions
            .insert("project_name".to_string(), Value::String("DemoBoilerplate".to_string()));
        yaml_template_specification.questions.insert(
            "project_type".to_string(),
            Value::Sequence(vec![Value::String("web".to_string()), Value::String("cli".to_string())]),
        );

        let template_specification: TemplateSpecification = yaml_template_specification.into();

        assert_eq!(template_specification.questions.len(), 2);
        assert_eq!(template_specification.questions[0].template_key, "project_name");
        assert_eq!(template_specification.questions[0].get_single_choice().unwrap(), "DemoBoilerplate");
        assert_eq!(template_specification.questions[1].template_key, "project_type");
        assert_eq!(template_specification.questions[1].get_multiple_choice().unwrap().len(), 2);
    }
}
