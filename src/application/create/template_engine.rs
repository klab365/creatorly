use crate::{
    application::common::interfaces::Os,
    domain::{file_tree::FileList, template_specification::TemplateSpecification},
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

    pub fn render(&self, _destination_path: &String, _template_specification: TemplateSpecification) -> Result<(), String> {
        println!("rendering....");

        let file_list: Result<FileList, String> = self.file_loader.load(_destination_path);
        if let Err(error) = file_list {
            println!("error: {error:?}");
            return Err(error);
        }

        for file in file_list.unwrap().files {
            println!("templateing: {file:?}");

            // render file name
            let renderd_file_name = self.template_renderer.render(file.clone(), _template_specification.clone())?;

            // render file content
            let content = self.file_system.read_file(file.clone()).expect("issue to read file");
            let renderd_file_content = self.template_renderer.render(content, _template_specification.clone())?;

            self.file_system
                .replace_file(file, renderd_file_name, renderd_file_content)
                .expect("issue to replace file");
        }

        Ok(())
    }
}
