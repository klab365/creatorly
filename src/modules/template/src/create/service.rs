use common::core::{
    errors::{Error, Result},
    interfaces::FileSystemInterface,
    user_interaction_interface::UserInteractionInterface,
};
use std::{path::PathBuf, sync::Arc};

use crate::templatespecification::core::{
    file_list::FileList,
    service::TemplateSpecificationService,
    template_specification::{TemplateSpecification, TemplateSpecificationItem},
};

pub struct CreateTemplateSpecificationService {
    templatespecification_service: Arc<TemplateSpecificationService>,
    file_system: Arc<dyn FileSystemInterface>,
    user_interaction_interface: Arc<dyn UserInteractionInterface>,
}

impl CreateTemplateSpecificationService {
    pub fn new(
        templatespecification_service: Arc<TemplateSpecificationService>,
        file_system: Arc<dyn FileSystemInterface>,
        user_interaction_interface: Arc<dyn UserInteractionInterface>,
    ) -> Self {
        Self {
            templatespecification_service,
            file_system,
            user_interaction_interface,
        }
    }

    pub async fn create_template_specification(&self, args: CreateTemplateArgs) -> Result<()> {
        // get all files
        let mut files = self
            .templatespecification_service
            .load_files(Some(args.entry_dir.clone()))
            .await?;

        // check if a creatorly.yml already exists, if not then create one or read it
        let found_creatorly_file = self
            .templatespecification_service
            .get_template_specification(&mut files)
            .await;

        let upload_dir = args.entry_dir;
        match found_creatorly_file {
            Ok(found_template) => {
                self.user_interaction_interface
                    .print_success("creatorly.yml found, it will only update with the new placeholders")
                    .await;

                self.update_template_specification(found_template, files, upload_dir.clone())
                    .await?;
            }
            Err(_) => {
                self.user_interaction_interface
                    .print("No creatorly.yml found, it will be created")
                    .await;

                self.create_new_template_specification(files, upload_dir.clone())
                    .await?;
            }
        };

        let msg = format!("🚀 updated creatorly.yml in {}", upload_dir.display());
        self.user_interaction_interface.print(&msg).await;

        Ok(())
    }

    async fn create_new_template_specification(&self, files: FileList, upload_dir: PathBuf) -> Result<()> {
        let parsed_tempalte_specifications = self.get_parsed_template_specifications(files.clone()).await?;

        self.templatespecification_service
            .save_template_specification(upload_dir, parsed_tempalte_specifications)
            .await?;

        Ok(())
    }

    async fn update_template_specification(
        &self,
        found_template_specifiaction: TemplateSpecification,
        files: FileList,
        upload_dir: PathBuf,
    ) -> Result<()> {
        let placeholders = self.get_placeholder(files).await?;
        let mut found_template_specifiaction = found_template_specifiaction;

        for placeholder in placeholders.iter() {
            let found_placeholder = found_template_specifiaction
                .placeholders
                .clone()
                .into_iter()
                .find(|p| p.template_key == *placeholder);

            if found_placeholder.is_none() {
                let parsed_placeholder = self
                    .templatespecification_service
                    .get_default_answer(placeholder)
                    .await?;

                found_template_specifiaction.placeholders.push(parsed_placeholder);
            }
        }

        self.templatespecification_service
            .save_template_specification(upload_dir, found_template_specifiaction)
            .await?;

        Ok(())
    }

    async fn get_all_creatorly_placeholders(&self, files: FileList) -> Result<Vec<String>> {
        let mut placeholders = Vec::new();

        for file in files.files.iter() {
            let file_content = self.file_system.read_file(file).await?;
            let regex = regex::Regex::new(r"\{\{\s*creatorly\.(\w+)\s*\}\}")
                .map_err(|err| Error::new(format!("Error in regex: {}", err)))?;

            // check file_path for placeholders
            for capture in regex.captures_iter(file.to_str()) {
                placeholders.push(capture[1].to_string());
            }

            for capture in regex.captures_iter(&file_content) {
                placeholders.push(capture[1].to_string());
            }
        }

        // todo: do it better
        placeholders.sort();
        placeholders.dedup();
        Ok(placeholders)
    }

    async fn get_placeholder(&self, files: FileList) -> Result<Vec<String>> {
        // get all placeholders
        let found_placeholders = self.get_all_creatorly_placeholders(files).await?;
        if found_placeholders.is_empty() {
            return Err(Error::with_advice(
                "No placeholders found".into(),
                "Please add {{ creatorly.* }} to your files or file names".into(),
            ));
        }

        Ok(found_placeholders)
    }

    async fn get_parsed_template_specifications(&self, files: FileList) -> Result<TemplateSpecification> {
        // get all placeholders
        let found_placeholders = self.get_placeholder(files).await?;

        // get all answers
        let mut parsed_template_specification_items: Vec<TemplateSpecificationItem> = vec![];
        for placeholder in found_placeholders {
            let template_specification = self
                .templatespecification_service
                .get_default_answer(&placeholder)
                .await?;

            parsed_template_specification_items.push(template_specification);
        }

        let parsed_template = TemplateSpecification {
            placeholders: parsed_template_specification_items,
        };

        Ok(parsed_template)
    }
}

pub struct CreateTemplateArgs {
    pub entry_dir: PathBuf,
}

#[cfg(test)]
mod test {
    use std::io::Write;

    use crate::templatespecification::core::service::TemplateSpecificationService;

    use super::*;
    use async_trait::async_trait;
    use common::infrastructure::file_system::FileSystem;
    use mockall::{
        mock,
        predicate::{self, eq},
    };

    mock! {
        UserInteractionInterface {}

        #[async_trait]
        impl UserInteractionInterface for UserInteractionInterface {
            async fn print_success(&self, message: &str);

            async fn print_error(&self, message: &str);

            async fn print(&self, message: &str);

            async fn get_input(&self, prompt: &str) -> Result<String>;
        }
    }

    #[tokio::test]
    async fn test_create_template_specification() {
        //arrange
        let example_dir = get_correct_example_dir().unwrap();
        let mut mock_user_interaction_interface = MockUserInteractionInterface::new();
        mock_user_interaction_interface
            .expect_print()
            .with(eq("No creatorly.yml found, it will be created"))
            .return_const(())
            .times(1);
        mock_user_interaction_interface
            .expect_get_input()
            .with(eq("description: "))
            .return_const(Ok("description".to_string()));
        mock_user_interaction_interface
            .expect_get_input()
            .with(eq("file_name: "))
            .return_const(Ok("file_name".to_string()));
        mock_user_interaction_interface
            .expect_get_input()
            .with(eq("name: "))
            .return_const(Ok("name".to_string()));
        mock_user_interaction_interface
            .expect_print()
            .with(predicate::str::contains("🚀 updated creatorly.yml in /tmp/example"))
            .return_const(())
            .times(1);

        let sut = get_create_template_service(Arc::new(mock_user_interaction_interface));

        // act
        let args = CreateTemplateArgs {
            entry_dir: example_dir.path().to_path_buf(),
        };
        let result = sut.create_template_specification(args).await;

        // assert
        assert!(result.is_ok());
        let creatorly_file = example_dir.path().join("creatorly.yml");
        assert!(creatorly_file.is_file());
        let creatorly_file_content = std::fs::read_to_string(creatorly_file).unwrap();
        assert_eq!(
            creatorly_file_content,
            r#"placeholders:
  description: description
  file_name: file_name
  name: name
"#,
        ); // todo check a better way
    }

    #[tokio::test]
    async fn test_get_all_creatorly_placeholders() {
        // let mut os_mock = MockFileSystemInterface::new();
        let example_dir = get_correct_example_dir().unwrap();
        let mock_user_interaction_interface = Arc::new(MockUserInteractionInterface::new());
        let service = CreateTemplateSpecificationService::new(
            Arc::new(TemplateSpecificationService::with_local_file_loader(
                mock_user_interaction_interface.clone(),
            )),
            Arc::new(FileSystem::default()),
            mock_user_interaction_interface.clone(),
        );

        let files = service
            .templatespecification_service
            .load_files(Some(example_dir.into_path()))
            .await
            .unwrap();
        let placeholders = service.get_all_creatorly_placeholders(files).await.unwrap();

        assert_eq!(placeholders.len(), 3);
        assert!(placeholders.contains(&"file_name".to_string()));
        assert!(placeholders.contains(&"name".to_string()));
        assert!(placeholders.contains(&"description".to_string()));
    }

    fn get_create_template_service(
        mock_user_interaction_interface: Arc<MockUserInteractionInterface>,
    ) -> CreateTemplateSpecificationService {
        CreateTemplateSpecificationService::new(
            Arc::new(TemplateSpecificationService::with_local_file_loader(
                mock_user_interaction_interface.clone(),
            )),
            Arc::new(FileSystem::default()),
            mock_user_interaction_interface.clone(),
        )
    }

    fn get_correct_example_dir() -> Result<tempdir::TempDir> {
        let tmp_dir = tempdir::TempDir::new("example").map_err(|_| Error::new("issue to create temp dir".into()))?;
        create_file(
            tmp_dir.path().join("{{ creatorly.file_name }}.txt").to_str().unwrap(),
            "{{ creatorly.name }} say {{ creatorly.description }}",
        )?;
        create_file(
            tmp_dir.path().join("{{ creatorly.file_name }}.md").to_str().unwrap(),
            "{{ creatorly.name }} say {{ creatorly.description }}",
        )?;

        Ok(tmp_dir)
    }

    fn create_file(path: &str, content: &str) -> Result<std::fs::File> {
        let mut file = std::fs::File::create(path).map_err(|_| Error::new("issue to create file".into()))?;
        file.write_all(content.as_bytes())
            .map_err(|_| Error::new("issue to write file".into()))?;

        Ok(file)
    }
}
