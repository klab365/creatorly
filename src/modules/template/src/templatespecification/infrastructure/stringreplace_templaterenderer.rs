use crate::templatespecification::core::{interfaces::TemplateRenderer, template_specification::TemplateSpecification};
use common::core::errors::Result;

/// StringReplaceRenderer is a struct that implements the TemplateRenderer trait.
/// It is responsible for rendering the template by replacing the placeholders with the answers.
/// It uses the String::replace method to replace the placeholders with the answers.
pub struct StringReplaceRenderer {}

impl TemplateRenderer for StringReplaceRenderer {
    fn render(&self, input: &str, config: &TemplateSpecification) -> Result<String> {
        let mut output = input.to_string();

        for template_item in config.placeholders.iter() {
            let key = template_item.get_template_key();
            let value = template_item.get_answer();
            let replacable_string = format!("{}.{}", TemplateSpecification::PREFIX, key);

            if !output.contains(&replacable_string) {
                continue;
            }

            output = output.replace(&replacable_string, value);
        }

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use crate::templatespecification::core::template_specification::{
        TemplateSpecificationItem, TemplateSpecificationItemType,
    };

    use super::*;

    #[test]
    fn render_should_return_rendered_value() {
        let sut = StringReplaceRenderer {};
        let mut data = TemplateSpecification::new();
        let mut template_item = TemplateSpecificationItem::new(
            "name".to_string(),
            TemplateSpecificationItemType::SingleChoice("Max".to_string()),
        );
        template_item.set_answer("Max".to_string());
        data.placeholders.push(template_item);

        // act
        let output = sut.render("Hello CREATORLY.name!", &data).unwrap();

        assert_eq!(output, "Hello Max!");
    }

    #[test]
    fn render_should_render_both_template() {
        let sut = StringReplaceRenderer {};
        let mut data = TemplateSpecification::new();
        let mut template_item = TemplateSpecificationItem::new(
            "name".to_string(),
            TemplateSpecificationItemType::SingleChoice("Max".to_string()),
        );
        template_item.set_answer("Max".to_string());
        data.placeholders.push(template_item);

        let mut template_item = TemplateSpecificationItem::new(
            "age".to_string(),
            TemplateSpecificationItemType::SingleChoice("30".to_string()),
        );
        template_item.set_answer("30".to_string());
        data.placeholders.push(template_item);

        // act
        let output = sut
            .render(
                "Hello CREATORLY.name! You are CREATORLY.age years old.",
                &data,
            )
            .unwrap();

        assert_eq!(output, "Hello Max! You are 30 years old.");
    }

    #[test]
    fn render_should_return_input_if_no_render_data_match() {
        let sut = StringReplaceRenderer {};
        let mut data = TemplateSpecification::new();
        data.placeholders.push(TemplateSpecificationItem::new(
            "name".to_string(),
            TemplateSpecificationItemType::SingleChoice("Max".to_string()),
        ));
        let output = sut.render("Hello Max!", &data).unwrap();
        assert_eq!(output, "Hello Max!");
    }

    #[test]
    fn render_should_not_render_if_template_is_not_write_correctly() {
        let sut = StringReplaceRenderer {};
        let mut data = TemplateSpecification::new();
        data.placeholders.push(TemplateSpecificationItem::new(
            "name".to_string(),
            TemplateSpecificationItemType::SingleChoice("Max".to_string()),
        ));

        let output = sut.render("Hello {{ CreatOrly.name }!", &data).unwrap();

        assert_eq!(output, "Hello {{ CreatOrly.name }!");
    }
}
