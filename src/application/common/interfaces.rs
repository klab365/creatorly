use crate::domain::template_specification::TemplateSpecification;

pub trait ConfigurationLoader {
    fn load_configuration(&self, configuration_path: String) -> Result<TemplateSpecification, String>;
}
