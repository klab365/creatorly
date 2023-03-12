use std::path::PathBuf;

use super::{
    interfaces::{FileTreeLoader, Prompt},
    template_engine::TemplateEngine,
};
use crate::{
    application::common::interfaces::{ConfigurationLoader, Os},
    domain::{file_tree::FileList, template_specification::TemplateSpecification},
};

pub struct CreateProjectInput {
    pub input_path: String,
    pub destination_path: String,
}

pub struct Service<'a> {
    folder_loader: &'a dyn FileTreeLoader,
    configuration_loader: &'a dyn ConfigurationLoader,
    file_system: &'a dyn Os,
    prompt: &'a dyn Prompt,
    template_engine: &'a TemplateEngine,
}

impl<'a> Service<'a> {
    pub fn new(
        folder_loader: &'a dyn FileTreeLoader,
        configuration_loader: &'a dyn ConfigurationLoader,
        file_system: &'a dyn Os,
        prompt: &'a dyn Prompt,
        template_engine: &'a TemplateEngine,
    ) -> Self {
        Self {
            folder_loader,
            configuration_loader,
            file_system,
            prompt,
            template_engine,
        }
    }

    pub fn create_project(&self, input: CreateProjectInput) -> Result<(), String> {
        if input.input_path.is_empty() {
            return Err("path is empty".to_string());
        }

        if input.destination_path.is_empty() {
            return Err("destination path is empty".to_string());
        }

        // load file list
        let files = self.load_files(&input.input_path);

        // load template specification
        let mut template_configuration = self.get_template_configuration(files.clone().unwrap())?;

        // get answer for question
        self.answer_questions(&mut template_configuration);

        // move files to destination folder
        self.move_files_to_destination_folder(input.destination_path, files.unwrap());

        // render files on destination folder
        // self.template_engine.render(input.destination_path.clone(), template_configuration.unwrap());

        println!("project created!");
        Ok(())
    }

    // get template configuration from the template path
    fn get_template_configuration(&self, files: FileList) -> Result<TemplateSpecification, String> {
        let found_file = files
            .files
            .iter()
            .find(|file| file.contains("creatorly.yaml") || file.contains("creatorly.yml"));
        if found_file.is_none() {
            return Err("creatorly.yaml not found".to_string());
        }

        let found_file = found_file.unwrap();
        self.configuration_loader.load_configuration(found_file.to_string())
    }

    fn load_files(&self, input_path: &str) -> Result<FileList, String> {
        let files: Result<FileList, String> = self.folder_loader.load(input_path);
        if let Err(error) = files {
            println!("error: {error:?}");
            return Err(error);
        }

        if let Ok(files) = files.clone() {
            let mut counter_found_files = 0;
            for file in files.files {
                println!("found file: {file:?}");
                counter_found_files += 1;
            }
            println!("found {counter_found_files} files");
        }

        Ok(files.unwrap())
    }

    fn move_files_to_destination_folder(&self, destination_path: String, files: FileList) {
        println!("moving files to destination folder: {destination_path:?}");

        let result = self.file_system.clear_folder(destination_path.clone());
        if let Err(_error) = result {
            return;
        }

        for file in files.files.iter() {
            let target_path = PathBuf::from(destination_path.clone()).join(file).to_str().unwrap().to_string();
            let result = self.file_system.move_file(file.clone(), target_path);

            if let Err(_error) = result {
                return;
            }
        }
    }

    fn answer_questions(&self, template_specification: &mut TemplateSpecification) {
        for item in &mut template_specification.questions {
            self.prompt.get_answer(item)
        }

        println!("answered questions: {template_specification:?}")
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use crate::{application::create::interfaces::MockFileTreeLoader, domain::file_tree::FileList};

    // #[test]
    // fn should_create_project() {
    //     // arrange
    //     let mut filetree_loader_mock = MockFileTreeLoader::new();
    //     filetree_loader_mock
    //         .expect_load()
    //         .with(mockall::predicate::eq("path".to_string()))
    //         .times(1)
    //         .returning(|_| Ok(FileList { files: vec![] }));

    //     let service = Service::new(&filetree_loader_mock);
    //     let input = CreateProjectInput { input_path: "path".to_string() };

    //     // act
    //     let result = service.create_project(input);

    //     // assert
    //     assert_eq!(result, Ok(()));
    // }
}
