use super::interfaces::TemplateRenderer;
use ::futures::future::join_all;
use common::core::errors::Error;
use common::core::interfaces::FileSystemInterface;
use common::core::{errors::Result, file::File};
use log::info;
use std::{path::PathBuf, sync::Arc};
use templatespecification::core::{file_list::FileList, template_specification::TemplateSpecification};

/// Struct for the function of the template engine
#[derive(Clone)]
pub struct RenderPushArgument {
    pub input_root_path: PathBuf,
    pub destination_path: PathBuf,
    pub file_list: FileList,
    pub template_specification: TemplateSpecification,
    pub dry_run: bool,
}

#[derive(Clone)]
pub struct TemplateEngine {
    template_renderer: Arc<dyn TemplateRenderer>,
    file_system: Arc<dyn FileSystemInterface>,
}

impl TemplateEngine {
    /// create a new instance of the template engine
    pub fn new(template_renderer: Arc<dyn TemplateRenderer>, file_system: Arc<dyn FileSystemInterface>) -> Self {
        Self {
            template_renderer,
            file_system,
        }
    }

    /// render files and push it directly to the destination path
    pub async fn render_and_push(self: &Arc<Self>, args: RenderPushArgument) -> Result<()> {
        let now = std::time::Instant::now();

        let mut handles = vec![];
        let files = args.file_list.files.clone();
        for file in files {
            let cloned_self = Arc::clone(self);
            let cloned_args = args.clone();
            // spawn a new thread for each file, there is no implement a mutex for the file system ;)
            let handle = tokio::spawn(async move {
                let _ = cloned_self.process_file(&file, cloned_args).await;
            });
            handles.push(handle);
        }

        join_all(handles).await;
        info!("Files rendered in {}ms", now.elapsed().as_millis());
        Ok(())
    }

    /// process one file
    /// first it will render the filename and then the content of the file line by line
    async fn process_file(&self, file: &File, args: RenderPushArgument) -> Result<()> {
        let target_file_name = self
            .render_file_name(
                file,
                &args,
                args.input_root_path.as_path().to_str().unwrap(),
                args.destination_path.as_path().to_str().unwrap(),
            )
            .await
            .map_err(|error| Error::new(error.to_string()));

        let Ok(target_file_name) = target_file_name else {
            return Err(Error::new(format!("Error while rendering file name of file {}", file)));
        };

        self.render_file_content(file, &args, &target_file_name).await?;

        Ok(())
    }

    /// render the file name if it contains template token
    async fn render_file_name(
        &self,
        file_name: &File,
        render_args: &RenderPushArgument,
        input_root_path: &str,
        destination_path: &str,
    ) -> Result<File> {
        let renderd_file_name_result = self
            .template_renderer
            .render(file_name.to_str(), &render_args.template_specification);

        let rendered_file_name = match renderd_file_name_result {
            Ok(renderd_file_name) => File::from(renderd_file_name),
            Err(error) => {
                log::error!("While rendering path {}: {}", file_name, error);
                file_name.clone()
            }
        };

        let renderd_file_name = rendered_file_name.replace(input_root_path, destination_path);

        if render_args.dry_run {
            return Ok(renderd_file_name);
        }
        self.file_system.write_file(&renderd_file_name, "").await?;

        Ok(renderd_file_name)
    }

    /// render the file content line by line
    async fn render_file_content(
        &self,
        file_name: &File,
        render_args: &RenderPushArgument,
        target_file_path: &File,
    ) -> Result<()> {
        let content = self.file_system.read_file(file_name).await?;

        let output = self
            .template_renderer
            .render(content.as_str(), &render_args.template_specification);

        let rendered_content = match output {
            Ok(rendered_content) => rendered_content,
            Err(error) => {
                log::error!(
                    "While rendering content of path {} was not success, therefore the previous content will be present: {}",
                    file_name, error
                );
                content.clone()
            }
        };

        if render_args.dry_run {
            return Ok(());
        }

        self.file_system
            .write_file(target_file_path, rendered_content.as_str())
            .await?;

        Ok(())
    }
}
