use crate::{
    application::common::interfaces::Os,
    domain::{file_list::FileList, template_specification::TemplateSpecification},
};

use super::interfaces::{FileTreeLoader, TemplateRenderer};

pub struct TemplateEngine<'a> {
    template_renderer: &'a dyn TemplateRenderer,
    file_loader: &'a dyn FileTreeLoader,
    file_system: &'a dyn Os,
}

impl<'a> TemplateEngine<'a> {
    pub fn new(template_renderere: &'a dyn TemplateRenderer, file_loader: &'a dyn FileTreeLoader, file_system: &'a dyn Os) -> Self {
        Self {
            template_renderer: template_renderere,
            file_loader,
            file_system,
        }
    }

    pub fn render(
        &self,
        input_root_path: &String,
        destination_path: &str,
        file_list: &FileList,
        _template_specification: TemplateSpecification,
    ) -> Result<(), String> {
        println!("rendering....");

        for file in file_list.clone().files {
            println!("templateing: {file:?}");

            // render file name
            let mut renderd_file_name = self.template_renderer.render(file.clone(), _template_specification.clone())?;
            renderd_file_name = renderd_file_name.replace(input_root_path, destination_path);

            // render file content
            let content = self.file_system.read_file(file.clone()).expect("issue to read file");
            let renderd_file_content = self.template_renderer.render(content, _template_specification.clone())?;

            self.file_system
                .write_file(renderd_file_name, renderd_file_content)
                .expect("issue to write file")
        }

        Ok(())
    }
}
