use crate::core::interfaces::TemplateRenderer;
use crate::core::template_specification::TemplateSpecification;
use common::core::errors::{Error, Result};
use liquid::model::Value;

pub struct LiquidTemplateRenderer {}

impl TemplateRenderer for LiquidTemplateRenderer {
    fn render(&self, input: &str, config: &TemplateSpecification) -> Result<String> {
        let templater = liquid::ParserBuilder::with_stdlib()
            .build()
            .map_err(|e| Error::new(format!("issue to build liquid template parser: {}", e)))?;

        let template = templater
            .parse(input)
            .map_err(|e| Error::new(format!("issue to parse liquid template: {}", e)))?;

        let data = map_to_liquid_object(config);
        let output = template
            .render(&data)
            .map_err(|e| Error::new(format!("issue to render liquid template: {}", e)))?;

        Ok(output)
    }
}

fn map_to_liquid_object(value: &TemplateSpecification) -> liquid::Object {
    let mut creatorly_data: liquid::Object = liquid::Object::new();
    for template_item in value.placeholders.iter() {
        let key = template_item.get_template_key().to_string();
        creatorly_data.insert(key.into(), Value::scalar(template_item.get_answer().to_string()));
    }

    let mut root = liquid::Object::new();
    let key = TemplateSpecification::PREFIX.to_string();
    root.insert(key.into(), Value::Object(creatorly_data));

    root
}

#[cfg(test)]
mod tests {
    use crate::core::template_specification::{TemplateSpecificationItem, TemplateSpecificationItemType};

    use super::*;

    #[test]
    fn render_should_return_rendered_value() {
        let liquid_template_renderer = LiquidTemplateRenderer {};
        let mut data = TemplateSpecification {
            placeholders: Vec::new(),
        };
        let mut template_item = TemplateSpecificationItem::new(
            "name".to_string(),
            TemplateSpecificationItemType::SingleChoice("Max".to_string()),
        );
        template_item.set_answer("Max".to_string());
        data.placeholders.push(template_item);
        let output = liquid_template_renderer
            .render("Hello {{ creatorly.name }}!", &data)
            .unwrap();

        assert_eq!(output, "Hello Max!");
    }

    #[test]
    fn render_should_not_return_rendered_value() {
        let liquid_template_renderer = LiquidTemplateRenderer {};
        let data = TemplateSpecification {
            placeholders: Vec::new(),
        };
        let output = liquid_template_renderer.render("Hello {{ name }}!", &data);
        assert!(output.is_err());
    }

    #[test]
    fn render_empty_brackets_should_return_orginal_string() {
        let liquid_template_renderer = LiquidTemplateRenderer {};
        let data = TemplateSpecification {
            placeholders: Vec::new(),
        };
        let output = liquid_template_renderer.render("Hello {{ }}!", &data);
        assert!(output.is_err());
    }

    #[test]
    fn render_should_input_without_render() {
        let liquid_template_renderer = LiquidTemplateRenderer {};
        let mut data = TemplateSpecification {
            placeholders: Vec::new(),
        };
        data.placeholders.push(TemplateSpecificationItem::new(
            "name".to_string(),
            TemplateSpecificationItemType::SingleChoice("Max".to_string()),
        ));
        let output = liquid_template_renderer.render("Hello Max!", &data).unwrap();
        assert_eq!(output, "Hello Max!");
    }

    #[test]
    fn render_should_skip_raw_endraw() {
        let liquid_template_renderer = LiquidTemplateRenderer {};
        let mut data = TemplateSpecification {
            placeholders: Vec::new(),
        };
        data.placeholders.push(TemplateSpecificationItem::new(
            "name".to_string(),
            TemplateSpecificationItemType::SingleChoice("Max".to_string()),
        ));

        let input = String::from(r#"{% raw %}Hello {{ creatorly.name }}!{% endraw %}"#);

        let output = liquid_template_renderer.render(input.as_str(), &data).unwrap();
        assert_eq!(output, "Hello {{ creatorly.name }}!");
    }
}
