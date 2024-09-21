use std::collections::HashMap;

use crate::templatespecification::core::{interfaces::TemplateRenderer, template_specification::TemplateSpecification};
use common::core::errors::{Error, Result};
use regex::Regex;

/// RegexTemplateRenderer is a struct that implements the TemplateRenderer trait.
/// It is responsible for rendering the template by replacing the placeholders with the answers.
/// It uses the regex method to replace the placeholders with the answers.
pub struct RegexTemplateRenderer {}

impl TemplateRenderer for RegexTemplateRenderer {
    fn render(&self, input: &str, config: &TemplateSpecification, answers: &HashMap<String, String>) -> Result<String> {
        let mut output = input.to_string();

        for (key, answer) in answers.iter() {
            let replacable_string = format!(
                "{}{}{}",
                config.get_placeholder_id(),
                config.get_placeholder_delimiter(),
                key
            );

            let regex = Regex::new(&regex::escape(&replacable_string).to_string())
                .map_err(|e| Error::new(format!("Error creating regex: {}", e)))?;

            output = regex.replace_all(&output, answer.as_str()).to_string();
        }

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use crate::templatespecification::core::template_specification::TemplateSpecificationItemType;

    use super::*;

    #[test]
    fn render_should_return_rendered_value() {
        let sut = RegexTemplateRenderer {};
        let mut data = TemplateSpecification::new();
        let mut answers = HashMap::new();
        let template_item = TemplateSpecificationItemType::SingleChoice("Max".to_string());
        data.placeholders.insert("name".to_string(), template_item);
        answers.insert("name".to_string(), "Max".to_string());

        // act
        let output = sut.render("Hello CREATORLY.name!", &data, &answers).unwrap();
        assert_eq!(output, "Hello Max!");

        let output = sut.render("Hello CREATORLY.name_test!", &data, &answers).unwrap();
        assert_eq!(output, "Hello Max_test!");
    }

    #[test]
    fn render_should_not_render_when_delimeter_is_underscore_but_placeholder_is_dot() {
        let sut = RegexTemplateRenderer {};
        let mut data = TemplateSpecification::from_id_delimiter("CREATORLY".to_string(), "_".to_string());
        let mut answers = HashMap::new();
        let template_item = TemplateSpecificationItemType::SingleChoice("Max".to_string());
        data.placeholders.insert("name".to_string(), template_item);
        answers.insert("name".to_string(), "Max".to_string());

        // act
        let output = sut.render("Hello CREATORLY.name_test!", &data, &answers).unwrap();
        assert_eq!(output, "Hello CREATORLY.name_test!");

        let output = sut.render("Hello CREATORLY_name_test", &data, &answers).unwrap();
        assert_eq!(output, "Hello Max_test");
    }

    #[test]
    fn render_should_render_both_template() {
        let sut = RegexTemplateRenderer {};
        let mut data = TemplateSpecification::new();
        let mut answers = HashMap::new();
        let template_item = TemplateSpecificationItemType::SingleChoice("Max".to_string());
        data.placeholders.insert("name".to_string(), template_item);
        answers.insert("name".to_string(), "Max".to_string());

        let template_item = TemplateSpecificationItemType::SingleChoice("30".to_string());
        data.placeholders.insert("age".to_string(), template_item);
        answers.insert("age".to_string(), "30".to_string());

        // act
        let output = sut
            .render(
                "Hello CREATORLY.name! You are CREATORLY.age years old.",
                &data,
                &answers,
            )
            .unwrap();

        assert_eq!(output, "Hello Max! You are 30 years old.");
    }

    #[test]
    fn render_should_return_input_if_no_render_data_match() {
        let sut = RegexTemplateRenderer {};
        let mut data = TemplateSpecification::new();
        let answers = HashMap::new();
        data.placeholders.insert(
            "name".to_string(),
            TemplateSpecificationItemType::SingleChoice("Max".to_string()),
        );
        let output = sut.render("Hello Max!", &data, &answers).unwrap();
        assert_eq!(output, "Hello Max!");
    }

    #[test]
    fn render_should_not_render_if_template_is_not_write_correctly() {
        let sut = RegexTemplateRenderer {};
        let mut data = TemplateSpecification::new();
        let answers = HashMap::new();
        data.placeholders.insert(
            "name".to_string(),
            TemplateSpecificationItemType::SingleChoice("Max".to_string()),
        );

        let output = sut.render("Hello CreatOrly.name!", &data, &answers).unwrap();

        assert_eq!(output, "Hello CreatOrly.name!");
    }

    #[test]
    fn render_should_render_with_custom_placeholder_id_and_placeholder_delimeter() {
        let sut = RegexTemplateRenderer {};
        let mut data = TemplateSpecification::from_id_delimiter("creatorly".to_string(), "-".to_string());
        let mut answers = HashMap::new();
        let item = TemplateSpecificationItemType::SingleChoice("Max".to_string());
        data.placeholders.insert("name".to_string(), item);
        answers.insert("name".to_string(), "Max".to_string());

        // act & assert
        let output1 = sut.render("Hello CREATORLY.name!", &data, &answers).unwrap();
        assert_eq!(output1, "Hello CREATORLY.name!");

        let output2 = sut.render("Hello creatorly-name!", &data, &answers).unwrap();
        assert_eq!(output2, "Hello Max!");
    }

    #[test]
    fn render_should_render_with_klab_placeholder_id_and_placeholder_delimeter() {
        let sut = RegexTemplateRenderer {};
        let mut data = TemplateSpecification::from_id_delimiter("KlabTestFramework".to_string(), ".".to_string());
        let mut answers = HashMap::new();
        let item = TemplateSpecificationItemType::SingleChoice("Max".to_string());
        data.placeholders.insert("ProjectName".to_string(), item);
        answers.insert("ProjectName".to_string(), "SuperDuper".to_string());

        // act & assert
        let output1 = sut
            .render("Hello KlabTestFramework.ProjectName.Core!", &data, &answers)
            .unwrap();
        assert_eq!(output1, "Hello SuperDuper.Core!");
    }

    #[test]
    fn render_should_return_path() {
        let sut = RegexTemplateRenderer {};
        let mut data = TemplateSpecification::from_id_delimiter("creatorly".to_string(), ".".to_string());
        let mut answers = HashMap::new();
        let item = TemplateSpecificationItemType::SingleChoice("Max".to_string());
        data.placeholders.insert("name".to_string(), item);
        answers.insert("name".to_string(), "superduper".to_string());

        // act & assert
        let output1 = sut
            .render("/core/creatorly.name/docs/hello.txt", &data, &answers)
            .unwrap();
        assert_eq!(output1, "/core/superduper/docs/hello.txt");
    }
}
