use super::interfaces::FolderLoader;
use crate::domain::folder_tree::Folder;

pub struct CreateProjectInput {
    pub path: String,
}

pub struct Service<'a> {
    folder_loader: &'a dyn FolderLoader,
}

impl<'a> Service<'a> {
    pub fn new(folder_loader: &'a dyn FolderLoader) -> Self {
        Self { folder_loader }
    }

    pub fn create_project(&self, input: CreateProjectInput) -> Result<(), String> {
        if input.path.is_empty() {
            return Err("path is empty".to_string());
        }

        let folder = self.folder_loader.load(&input.path);
        match folder {
            Ok(folder) => {
                println!("folder: {:?}", folder);
            }
            Err(error) => {
                return Err(error);
            }
        }

        // TODO: template project with configuration

        // TODO: create project

        println!("project created!");
        Ok(())
    }

    pub fn template_project(&self, configuration: String) -> Result<(), String> {
        Ok(())
    }
}

mod tests {
    use super::*;
    use crate::application::create::interfaces::MockFolderLoader;

    #[test]
    fn should_create_project() {
        // arrange
        let mut folder_loader_mock = MockFolderLoader::new();
        folder_loader_mock
            .expect_load()
            .with(mockall::predicate::eq("path".to_string()))
            .times(1)
            .returning(|_| Ok(Folder::new("path".to_string())));

        let service = Service::new(&folder_loader_mock);
        let input = CreateProjectInput {
            path: "path".to_string(),
        };

        // act
        let result = service.create_project(input);

        // assert
        assert_eq!(result, Ok(()));
    }
}
