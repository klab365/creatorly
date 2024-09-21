use crate::templatespecification::infrastructure::regex_templaterenderer::RegexTemplateRenderer;

use super::interfaces::TemplateRenderer;
use super::template_configuration::{TemplateConfiguration, TemplateConfigurationItem};
use ::futures::future::join_all;
use common::core::errors::Error;
use common::core::errors::Result;
use common::core::interfaces::FileSystemInterface;
use common::core::user_interaction_interface::UserInteraction;
use std::collections::HashMap;
use std::path::Path;
use std::{path::PathBuf, sync::Arc};

/// Struct for the function of the template engine
#[derive(Clone)]
pub struct RenderPushArgument {
    pub input_path: PathBuf,
    pub destination_path: PathBuf,
    pub template_configuration: TemplateConfiguration,
}

struct RenderArgument {
    file: PathBuf,
    template: TemplateConfigurationItem,
    answers: HashMap<String, String>,
    input_path: PathBuf,
    destination_path: PathBuf,
}

#[derive(Clone)]
pub struct TemplateEngine {
    template_renderer: Arc<dyn TemplateRenderer>,
    file_system: Arc<dyn FileSystemInterface>,
    user_interface: Arc<dyn UserInteraction>,
}

impl TemplateEngine {
    /// create a new instance of the template engine
    pub fn new(
        template_renderer: Arc<dyn TemplateRenderer>,
        file_system: Arc<dyn FileSystemInterface>,
        user_interaction_interface: Arc<dyn UserInteraction>,
    ) -> Self {
        Self {
            template_renderer,
            file_system,
            user_interface: user_interaction_interface,
        }
    }

    pub fn new_with_default_template_renderer(
        file_system: Arc<dyn FileSystemInterface>,
        user_interaction_interface: Arc<dyn UserInteraction>,
    ) -> Self {
        let template_renderer = Arc::new(RegexTemplateRenderer {});
        Self::new(template_renderer, file_system, user_interaction_interface)
    }

    /// render files and push it directly to the destination path (async with multiple threads - one thread per file)
    pub async fn render_and_push(self: &Arc<Self>, args: RenderPushArgument) -> Result<()> {
        let args = Arc::new(args);
        let mut handles = vec![];

        // clear the destination folder
        self.file_system.clear_folder(&args.destination_path).await?;

        let templates = args.template_configuration.templates.clone();
        for template in templates.into_iter() {
            let cloned_template = template.clone();

            for file in template.file_list.into_iter() {
                let cloned_self = Arc::clone(self);
                let args = RenderArgument {
                    file: file.clone(),
                    template: cloned_template.clone(),
                    answers: args.template_configuration.answers.clone(),
                    input_path: args.input_path.clone(),
                    destination_path: args.destination_path.clone(),
                };

                // spawn a new thread for each file, there is no implement a mutex for the file system ;)
                let handle = tokio::spawn(async move {
                    let _ = cloned_self.process_file(args).await;
                });

                handles.push(handle);
            }
        }

        join_all(handles).await;

        Ok(())
    }

    /// process one file
    /// first it will render the filename and then the content of the file line by line
    async fn process_file(&self, args: RenderArgument) -> Result<()> {
        let target_file_name = self.render_file_name(&args).await?;

        self.render_file_content(&target_file_name, &args).await?;

        Ok(())
    }

    /// render the file name if it contains template token
    async fn render_file_name(&self, arg: &RenderArgument) -> Result<String> {
        let Some(input_root_path) = arg.input_path.as_path().to_str() else {
            return Err(Error::new(format!(
                "Input path don't exist {}",
                arg.input_path.display()
            )));
        };

        let Some(destination_path) = arg.destination_path.as_path().to_str() else {
            return Err(Error::new(format!(
                "Destination path don't exist {}",
                arg.file.display()
            )));
        };

        let renderd_file_name_result = self.template_renderer.render(
            arg.file.to_str().unwrap(),
            &arg.template.template_specification,
            &arg.answers,
        );

        let rendered_file_name: PathBuf = match renderd_file_name_result {
            Ok(renderd_file_name) => PathBuf::from(renderd_file_name),
            Err(error) => {
                self.user_interface
                    .print_error(format!("While rendering path {}: {}", arg.file.display(), error).as_str())
                    .await;

                arg.file.clone()
            }
        };

        let rendered_file_name_str = rendered_file_name.to_str().unwrap();
        let rendered_file_name = rendered_file_name_str.replace(input_root_path, destination_path);
        Ok(rendered_file_name)
    }

    /// render the file content line by line
    async fn render_file_content(&self, target_file_path: impl AsRef<Path>, args: &RenderArgument) -> Result<()> {
        // if the file is a binary, move it directly
        if self.file_system.is_binary(&args.file).await? {
            self.file_system
                .move_file(&args.file, target_file_path.as_ref())
                .await?;
            return Ok(());
        }

        let content = self.file_system.read_file(&args.file).await?;

        let output =
            self.template_renderer
                .render(content.as_str(), &args.template.template_specification, &args.answers);

        let rendered_content = match output {
            Ok(rendered_content) => rendered_content,
            Err(error) => {
                self.user_interface
                    .print_error(format!("While rendering content of path {}: {}", args.file.display(), error).as_str())
                    .await;
                content.clone()
            }
        };

        self.file_system
            .write_file(target_file_path.as_ref(), rendered_content.as_str())
            .await?;

        Ok(())
    }
}
