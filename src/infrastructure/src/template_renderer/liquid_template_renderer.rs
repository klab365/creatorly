use application::generate::{
    interfaces::TemplateRenderer,
    template_specification::{TemplateSpecification, TemplateSpecificationItem},
};
use liquid::model::Value;

pub struct LiquidTemplateRenderer {}

fn map_to_liquid_object(value: TemplateSpecification) -> liquid::Object {
    let mut creatorly_data: liquid::Object = liquid::Object::new();
    for template_item in value.questions {
        creatorly_data.insert(
            template_item.get_template_key().into(),
            Value::scalar(template_item.get_answer().to_string()),
        );
    }

    let mut root = liquid::Object::new();
    root.insert(TemplateSpecificationItem::PREFIX.into(), Value::Object(creatorly_data));

    root
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
            return Err(error.to_string());
        }

        Ok(output.unwrap())
    }
}

#[cfg(test)]
mod tests {
    use application::generate::template_specification::{TemplateSpecificationItem, TemplateSpecificationItemType};

    use super::*;

    #[test]
    fn render_should_return_rendered_value() {
        let liquid_template_renderer = LiquidTemplateRenderer {};
        let mut data = TemplateSpecification { questions: Vec::new() };
        let mut template_item = TemplateSpecificationItem::new(
            "name".to_string(),
            TemplateSpecificationItemType::SingleChoice("Max".to_string()),
        );
        template_item.set_answer("Max".to_string());
        data.questions.push(template_item);
        let output = liquid_template_renderer
            .render("Hello {{ creatorly.name }}!".to_string(), data)
            .unwrap();

        assert_eq!(output, "Hello Max!");
    }

    #[test]
    fn render_should_not_return_rendered_value() {
        let liquid_template_renderer = LiquidTemplateRenderer {};
        let data = TemplateSpecification { questions: Vec::new() };
        let output = liquid_template_renderer.render("Hello {{ name }}!".to_string(), data);
        assert!(output.is_err());
    }

    #[test]
    fn render_empty_brackets_should_return_orginal_string() {
        let liquid_template_renderer = LiquidTemplateRenderer {};
        let data = TemplateSpecification { questions: Vec::new() };
        let output = liquid_template_renderer.render("Hello {{ }}!".to_string(), data);
        assert!(output.is_err());
    }

    #[test]
    fn render_should_input_without_render() {
        let liquid_template_renderer = LiquidTemplateRenderer {};
        let mut data = TemplateSpecification { questions: Vec::new() };
        data.questions.push(TemplateSpecificationItem::new(
            "name".to_string(),
            TemplateSpecificationItemType::SingleChoice("Max".to_string()),
        ));
        let output = liquid_template_renderer.render("Hello Max!".to_string(), data).unwrap();
        assert_eq!(output, "Hello Max!");
    }
}
