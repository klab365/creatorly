use crate::domain::template_specification::TemplateSpecification;

pub struct TemplateEngine {}

impl TemplateEngine {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(&self, _destination_path: String, _template_specification: TemplateSpecification) {
        println!("rendering....")
    }
}
