use std::{path::PathBuf, sync::Arc};

use common::core::{
    errors::{Error, Result},
    interfaces::FileSystemInterface,
};
use log::info;
use templatespecification::core::{
    file_list::FileList,
    interfaces::Prompt,
    service::TemplateSpecificationService,
    template_specification::{TemplateSpecification, TemplateSpecificationItem},
};

pub struct CreateTemplateSpecificationService {
    templatespecification_service: Arc<TemplateSpecificationService>,
    prompt: Arc<dyn Prompt + Send + Sync>,
    file_system: Arc<dyn FileSystemInterface>,
}

impl CreateTemplateSpecificationService {
    pub fn new(
        templatespecification_service: Arc<TemplateSpecificationService>,
        prompt: Arc<dyn Prompt + Send + Sync>,
        file_system: Arc<dyn FileSystemInterface>,
    ) -> Self {
        Self {
            templatespecification_service,
            prompt,
            file_system,
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
                info!("creatorly.yml found, it will only update with the new placeholders");
                self.update_template_specification(found_template, files, upload_dir.clone())
                    .await?;
                info!("🚀 updated creatorly.yml in {}", upload_dir.display().to_string());
            }
            Err(_) => {
                info!("No creatorly.yml found, it will be created");
                self.create_new_template_specification(files, upload_dir.clone())
                    .await?;
                info!("🚀 created a creatorly.yml in {}", upload_dir.display().to_string());
            }
        };

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
                let parsed_placeholder = self.prompt.get_default_answer(placeholder)?;
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
            let template_specification = self.prompt.get_default_answer(&placeholder)?;
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

    use super::*;
    use common::infrastructure::file_system::FileSystem;
    use mockall::{mock, predicate::eq};
    use templatespecification::core::template_specification::{
        TemplateSpecificationItem, TemplateSpecificationItemType,
    };

    mock! {
        Prompt {}

        impl Prompt for Prompt {
            fn get_answer(&self, template_specification_item: &mut TemplateSpecificationItem);

            fn get_default_answer(&self, placeholder: &str) -> Result<TemplateSpecificationItem>;
        }
    }

    #[tokio::test]
    async fn test_create_template_specification() {
        //arrange
        let mut prompt = MockPrompt::new();
        prompt
            .expect_get_default_answer()
            .with(eq("file_name"))
            .times(1)
            .return_const(Ok(TemplateSpecificationItem {
                template_key: "file_name".to_string(),
                answer: "file_name".to_string(),
                item: TemplateSpecificationItemType::SingleChoice("file_name".to_string()),
            }));
        prompt
            .expect_get_default_answer()
            .with(eq("name"))
            .times(1)
            .return_const(Ok(TemplateSpecificationItem {
                template_key: "name".to_string(),
                answer: "name".to_string(),
                item: TemplateSpecificationItemType::SingleChoice("name".to_string()),
            }));
        prompt
            .expect_get_default_answer()
            .with(eq("description"))
            .return_const(Ok(TemplateSpecificationItem {
                template_key: "description".to_string(),
                answer: "description".to_string(),
                item: TemplateSpecificationItemType::SingleChoice("description".to_string()),
            }))
            .times(1);

        let example_dir = get_correct_example_dir().unwrap();
        let sut = get_create_template_service(prompt);

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

        let service = CreateTemplateSpecificationService::new(
            Arc::new(TemplateSpecificationService::with_local_file_loader()),
            Arc::new(MockPrompt::new()),
            Arc::new(FileSystem::default()),
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

    fn get_create_template_service(mock_propt: MockPrompt) -> CreateTemplateSpecificationService {
        CreateTemplateSpecificationService::new(
            Arc::new(TemplateSpecificationService::with_local_file_loader()),
            Arc::new(mock_propt),
            Arc::new(FileSystem::default()),
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
