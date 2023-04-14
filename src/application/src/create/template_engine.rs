use log::warn;

use super::{file_list::FileList, interfaces::TemplateRenderer, template_specification::TemplateSpecification};
use crate::common::interfaces::Os;

pub struct TemplateEngine<'a> {
    template_renderer: &'a dyn TemplateRenderer,
    file_system: &'a dyn Os,
}

impl<'a> TemplateEngine<'a> {
    pub fn new(template_renderere: &'a dyn TemplateRenderer, file_system: &'a dyn Os) -> Self {
        Self {
            template_renderer: template_renderere,
            file_system,
        }
    }

    pub fn render_and_push(
        &self,
        input_root_path: &String,
        destination_path: &str,
        file_list: &FileList,
        _template_specification: TemplateSpecification,
    ) -> Result<(), String> {
        for file in file_list.clone().files {
            let renderd_file_name_result = self.template_renderer.render(file.clone(), _template_specification.clone());
            let rendered_file_name = match renderd_file_name_result {
                Ok(renderd_file_name) => renderd_file_name,
                Err(error) => {
                    warn!("Error while rendering path {}: {}", file, error);
                    file.clone()
                }
            };
            let renderd_file_name = rendered_file_name.replace(input_root_path, destination_path);

            // render file content
            let content = self.file_system.read_file(file.clone()).expect("issue to read file");
            let renderd_file_content = self.template_renderer.render(content, _template_specification.clone());
            let renderd_file_content = match renderd_file_content {
                Ok(renderd_file_content) => renderd_file_content,
                Err(error) => {
                    warn!("Error while rendering content of path {}: {}", file, error);
                    continue;
                }
            };

            self.file_system
                .write_file(renderd_file_name, renderd_file_content)
                .expect("issue to write file")
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{common::interfaces::MockOs, create::interfaces::MockTemplateRenderer};

    use super::*;

    #[test]
    fn test_render() {
        let mut file_list = FileList { files: Vec::new() };
        file_list.files.push("file1.txt".to_owned());
        file_list.files.push("file2.txt".to_owned());
        let input_root_path = "/input/root/path".to_owned();
        let destination_path = "/destination/path";
        let template_specification = TemplateSpecification::default();
        let mut file_contents = vec!["file1 content", "file2 content"];

        let mut template_renderer_mock = MockTemplateRenderer::new();
        template_renderer_mock
            .expect_render()
            .times(4)
            .returning(|_, _| Ok(String::from("template_content")));

        let mut os_mock = MockOs::new();
        os_mock
            .expect_read_file()
            .times(2)
            .returning(move |_| Ok(file_contents.pop().unwrap().to_string()));
        os_mock.expect_write_file().times(2).returning(move |_, _| Ok(()));

        let template_engine = TemplateEngine::new(&template_renderer_mock, &os_mock);
        let result = template_engine.render_and_push(&input_root_path, &destination_path, &file_list, template_specification);
        assert!(result.is_ok());
    }

    #[test]
    fn render_should_return_same_input_if_rendering_has_failed() {
        // arrange
        let mut file_list = FileList { files: Vec::new() };
        file_list.files.push("file1.txt".to_owned());
        file_list.files.push("file2.txt".to_owned());
        let input_root_path = "/input/root/path".to_owned();
        let destination_path = "/destination/path";
        let template_specification = TemplateSpecification::default();
        let mut file_contents = vec!["file1 content", "file2 content"];

        let mut template_renderer_mock = MockTemplateRenderer::new();
        template_renderer_mock.expect_render().times(4).returning(|_, _| Err(String::from("error")));

        let mut os_mock = MockOs::new();
        os_mock
            .expect_read_file()
            .times(2)
            .returning(move |_| Ok(file_contents.pop().unwrap().to_string()));
        os_mock.expect_write_file().times(0);

        // act
        let template_engine = TemplateEngine::new(&template_renderer_mock, &os_mock);
        let result = template_engine.render_and_push(&input_root_path, &destination_path, &file_list, template_specification);

        // assert
        assert!(result.is_ok());
    }
}
