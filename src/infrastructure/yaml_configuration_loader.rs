use crate::{application::configuration::interfaces::ConfigurationLoader, domain::template_specification::TemplateSpecification};

pub struct YamlConfigurationLoader {}

impl ConfigurationLoader for YamlConfigurationLoader {
    fn load_configuration(&self, configuration_path: String) -> Result<TemplateSpecification, String> {
        let file = std::fs::File::open(configuration_path).expect("Unable to open file");
        let value: TemplateSpecification = serde_yaml::from_reader(file).expect("Unable to read file");

        println!("{:?}", value);

        Ok(value)
    }
}

mod tests {
    use serde_yaml::Value;

    use super::*;
    use std::{collections::HashMap, hash::Hash};

    #[test]
    fn test_load_configuration() {
        // let yaml_configuration_loader = YamlConfigurationLoader {};
        // let mut expected_provider_options: HashMap<String, Value> = HashMap::new();
        // expected_provider_options.insert("project_name".to_string(), Value::String("DemoProject".to_string()));

        // // act
        // let template_specification = yaml_configuration_loader
        //     .load_configuration("/workspaces/creatorly/src/infrastructure/test_data/test_template_spec.yml".to_string())
        //     .unwrap();

        // // assert
        // assert_eq!(template_specification.options, expected_provider_options);
    }
}
