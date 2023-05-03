use std::sync::{Arc, Mutex};

use super::{file_list::FileList, interfaces::TemplateRenderer, template_specification::TemplateSpecification};
use crate::common::interfaces::Os;
use log::{info, warn};

/// Struct for the function of the template engine
pub struct RenderPushArgument {
    pub input_root_path: String,
    pub destination_path: String,
    pub file_list: FileList,
    pub template_specification: TemplateSpecification,
}

pub struct TemplateEngine {
    template_renderer: Arc<Mutex<dyn TemplateRenderer>>,
    file_system: Arc<dyn Os>,
}

impl TemplateEngine {
    /// create a new instance of the template engine
    pub fn new(template_renderer: Arc<Mutex<dyn TemplateRenderer>>, file_system: Arc<dyn Os>) -> Self {
        Self {
            template_renderer,
            file_system,
        }
    }

    /// render files and push it directly to the destination path
    pub fn render_and_push(&self, args: RenderPushArgument) -> Result<(), String> {
        let now = std::time::Instant::now();

        for file in args.file_list.files {
            let target_file_name = self.render_file_name(&file, &args.template_specification, &args.input_root_path, &args.destination_path)?;
            self.render_file_content_line_by_line(&file, &args.template_specification, &target_file_name);
        }

        info!("Rendered in {}ms", now.elapsed().as_millis());
        Ok(())
    }

    /// render the file name if it contains template token
    fn render_file_name(
        &self,
        file_name: &String,
        template_specification: &TemplateSpecification,
        input_root_path: &str,
        destination_path: &str,
    ) -> Result<String, String> {
        let renderer = self.template_renderer.lock().unwrap();
        let renderd_file_name_result = renderer.render(file_name.clone(), template_specification.clone());
        let rendered_file_name = match renderd_file_name_result {
            Ok(renderd_file_name) => renderd_file_name,
            Err(error) => {
                warn!("Warn while rendering path {}: {}", file_name, error);
                file_name.clone()
            }
        };

        let renderd_file_name = rendered_file_name.replace(input_root_path, destination_path);
        self.file_system.write_file(renderd_file_name.clone(), String::from(""))?;

        Ok(renderd_file_name)
    }

    /// render the file content line by line
    fn render_file_content_line_by_line(&self, file_name: &String, template_specification: &TemplateSpecification, target_file_path: &str) {
        let content = self.file_system.read_file_buffered(file_name.clone()).expect("issue to read file");

        let renderer = self.template_renderer.lock().unwrap();
        for line in content {
            let renderd_line = renderer.render(line.clone(), template_specification.clone());
            let renderd_line = match renderd_line {
                Ok(renderd_line) => renderd_line,
                Err(error) => {
                    warn!("Warn while rendering content of path {}: {}", file_name, error);
                    line.clone()
                }
            };
            self.file_system
                .write_line_to_file(target_file_path, renderd_line)
                .expect("issue to write file");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{common::interfaces::MockOs, create::interfaces::MockTemplateRenderer};
    use mockall::predicate::eq;
    use std::vec;

    #[test]
    fn test_render_push_should_be_correct() {
        // arrange
        let template_specification = TemplateSpecification::default();

        let mut file_list = FileList::default();
        let first_file_path = String::from("input/file1.txt");
        let second_file_path = String::from("input/file2.txt");

        file_list.add(first_file_path);
        file_list.add(second_file_path);

        let mut os_mock = MockOs::new();
        os_mock
            .expect_write_file()
            .with(eq(String::from("destination/file1.txt")), eq(String::from("")))
            .times(1)
            .returning(move |_, _| Ok(()));
        os_mock
            .expect_write_file()
            .with(eq(String::from("destination/file2.txt")), eq(String::from("")))
            .times(1)
            .returning(move |_, _| Ok(()));
        os_mock
            .expect_read_file_buffered()
            .with(eq(String::from("input/file1.txt")))
            .times(1)
            .returning(|_| Ok(vec![String::from("file1"), String::from("file2"), String::from("file3")]));
        os_mock
            .expect_read_file_buffered()
            .with(eq(String::from("input/file2.txt")))
            .times(1)
            .returning(|_| Ok(vec![String::from("file4"), String::from("file5"), String::from("file6")]));
        os_mock.expect_write_line_to_file().times(6).returning(|_, _| Ok(()));

        let mut template_renderer_mock = MockTemplateRenderer::new();
        template_renderer_mock.expect_render().times(8).returning(|input, _| Ok(input));

        // act
        let template_engine = TemplateEngine::new(&template_renderer_mock, &os_mock);
        let args = RenderPushArgument {
            input_root_path: String::from("input"),
            destination_path: String::from("destination"),
            file_list: file_list.clone(),
            template_specification: template_specification,
        };
        let result = template_engine.render_and_push(args);

        // assert
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_render_push_should_return_same_input_if_rendering_has_failed() {
        // arrange
        let template_specification = TemplateSpecification::default();

        let mut file_list = FileList::default();
        let first_file_path = String::from("input/file1.txt");
        let second_file_path = String::from("input/file2.txt");

        file_list.add(first_file_path);
        file_list.add(second_file_path);

        let mut os_mock = MockOs::new();
        os_mock
            .expect_write_file()
            .with(eq(String::from("destination/file1.txt")), eq(String::from("")))
            .times(1)
            .returning(move |_, _| Ok(()));
        os_mock
            .expect_write_file()
            .with(eq(String::from("destination/file2.txt")), eq(String::from("")))
            .times(1)
            .returning(move |_, _| Ok(()));
        os_mock
            .expect_read_file_buffered()
            .with(eq(String::from("input/file1.txt")))
            .times(1)
            .returning(|_| Ok(vec![String::from("file1"), String::from("file2"), String::from("file3")]));
        os_mock
            .expect_read_file_buffered()
            .with(eq(String::from("input/file2.txt")))
            .times(1)
            .returning(|_| Ok(vec![String::from("file4"), String::from("file5"), String::from("file6")]));
        os_mock.expect_write_line_to_file().times(6).returning(|_, _| Ok(()));

        let mut template_renderer_mock = MockTemplateRenderer::new();
        template_renderer_mock.expect_render().times(8).returning(|_, _| Err(String::from("error")));

        // act
        let template_engine = TemplateEngine::new(&template_renderer_mock, &os_mock);
        let args = RenderPushArgument {
            input_root_path: String::from("input"),
            destination_path: String::from("destination"),
            file_list: file_list.clone(),
            template_specification: template_specification,
        };
        let result = template_engine.render_and_push(args);

        // assert
        assert_eq!(result.is_ok(), true);
    }
}
