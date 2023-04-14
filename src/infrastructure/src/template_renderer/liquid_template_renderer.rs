use application::create::{interfaces::TemplateRenderer, template_specification::TemplateSpecification};
use liquid::model::Value;
use log::warn;

pub struct LiquidTemplateRenderer {}

fn map_to_liquid_object(value: TemplateSpecification) -> liquid::Object {
    let mut data = liquid::Object::new();

    for template_item in value.questions {
        data.insert(template_item.template_key.into(), Value::scalar(template_item.answer.to_string()));
    }

    data
}

impl TemplateRenderer for LiquidTemplateRenderer {
    fn render(&self, input: String, config: TemplateSpecification) -> Result<String, String> {
        let template = liquid::ParserBuilder::with_stdlib().build().unwrap().parse(&input);
        if let Err(error) = template {
            return Err(error.to_string());
        }

        let data: liquid::Object = map_to_liquid_object(config);
        let output = template.unwrap().render(&data);
        if let Err(error) = output {
            warn!("Error while rendering template: {}", error);
            return Ok(input);
        }

        Ok(output.unwrap())
    }
}

#[cfg(test)]
mod tests {
    use application::create::template_specification::{TemplateSpecificationItem, TemplateSpecificationItemType};

    use super::*;

    #[test]
    fn render_should_return_rendered_value() {
        let liquid_template_renderer = LiquidTemplateRenderer {};
        let mut data = TemplateSpecification { questions: Vec::new() };
        data.questions.push(TemplateSpecificationItem {
            template_key: "name".to_string(),
            item: TemplateSpecificationItemType::SingleChoice("Max".to_string()),
            answer: "Max".to_string(),
        });
        let output = liquid_template_renderer.render("Hello {{name}}!".to_string(), data).unwrap();
        assert_eq!(output, "Hello Max!");
    }

    #[test]
    fn render_should_not_return_rendered_value() {
        let liquid_template_renderer = LiquidTemplateRenderer {};
        let data = TemplateSpecification { questions: Vec::new() };
        let output = liquid_template_renderer.render("Hello {{ name }}!".to_string(), data).unwrap();
        assert_ne!(output, "Hello {{ name }}!");
    }

    #[test]
    fn render_should_input_without_render() {
        let liquid_template_renderer = LiquidTemplateRenderer {};
        let mut data = TemplateSpecification { questions: Vec::new() };
        data.questions.push(TemplateSpecificationItem {
            template_key: "name".to_string(),
            item: TemplateSpecificationItemType::SingleChoice("Max".to_string()),
            answer: "Max".to_string(),
        });
        let output = liquid_template_renderer.render("Hello Max!".to_string(), data).unwrap();
        assert_eq!(output, "Hello Max!");
    }
}
