use crate::templatespecification::infrastructure::regex_templaterenderer::RegexTemplateRenderer;

use super::interfaces::TemplateRenderer;
use super::template_configuration::TemplateConfiguration;
use ::futures::future::join_all;
use common::core::errors::Error;
use common::core::interfaces::FileSystemInterface;
use common::core::user_interaction_interface::UserInteraction;
use common::core::{errors::Result, file::File};
use std::{path::PathBuf, sync::Arc};

/// Struct for the function of the template engine
#[derive(Clone)]
pub struct RenderPushArgument {
    pub destination_path: PathBuf,
    pub template_configuration: TemplateConfiguration,
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
        let files = args.template_configuration.file_list.files.clone();
        for file in files.into_iter() {
            let cloned_self = Arc::clone(self);
            let cloned_args = args.clone();
            // spawn a new thread for each file, there is no implement a mutex for the file system ;)
            let handle = tokio::spawn(async move {
                let _ = cloned_self.process_file(&file, cloned_args).await;
            });
            handles.push(handle);
        }

        join_all(handles).await;
        self.user_interface.print("🚀 Files rendered").await;

        Ok(())
    }

    /// process one file
    /// first it will render the filename and then the content of the file line by line
    async fn process_file(&self, file: &File, args: Arc<RenderPushArgument>) -> Result<()> {
        let target_file_name = self.render_file_name(file, &args).await?;

        self.render_file_content(file, &target_file_name, &args).await?;

        Ok(())
    }

    /// render the file name if it contains template token
    async fn render_file_name(&self, file_name: &File, render_args: &RenderPushArgument) -> Result<File> {
        let input_root_path = render_args
            .template_configuration
            .file_list
            .root_path
            .as_path()
            .to_str();

        let Some(input_root_path) = input_root_path else {
            return Err(Error::new(format!(
                "Error while rendering file name of file {}",
                file_name
            )));
        };

        let Some(destination_path) = render_args.destination_path.as_path().to_str() else {
            return Err(Error::new(format!("Destination path don't exist {}", file_name)));
        };

        let renderd_file_name_result = self.template_renderer.render(
            file_name.to_str(),
            &render_args.template_configuration.template_specification,
        );

        let rendered_file_name = match renderd_file_name_result {
            Ok(renderd_file_name) => File::from(renderd_file_name),
            Err(error) => {
                self.user_interface
                    .print_error(format!("While rendering path {}: {}", file_name, error).as_str())
                    .await;
                file_name.clone()
            }
        };

        let renderd_file_name = rendered_file_name.replace(input_root_path, destination_path);
        self.file_system.write_file(&renderd_file_name, "").await?;

        Ok(renderd_file_name)
    }

    /// render the file content line by line
    async fn render_file_content(
        &self,
        file_name: &File,
        target_file_path: &File,
        args: &RenderPushArgument,
    ) -> Result<()> {
        // if the file is a binary, move it directly
        if self.file_system.is_binary(file_name).await? {
            self.file_system.move_file(file_name, target_file_path).await?;
            return Ok(());
        }

        let content = self.file_system.read_file(file_name).await?;

        let output = self
            .template_renderer
            .render(content.as_str(), &args.template_configuration.template_specification);

        let rendered_content = match output {
            Ok(rendered_content) => rendered_content,
            Err(error) => {
                self.user_interface
                    .print_error(format!("While rendering content of path {}: {}", file_name, error).as_str())
                    .await;
                content.clone()
            }
        };

        self.file_system
            .write_file(target_file_path, rendered_content.as_str())
            .await?;

        Ok(())
    }
}
