use std::path::PathBuf;
use walkdir::WalkDir;

use crate::{application::create::interfaces::FileTreeLoader, domain::file_tree::FileList};

pub struct LocalFileLoader {}

impl LocalFileLoader {
    pub fn new() -> Self {
        Self {}
    }
}

impl FileTreeLoader for LocalFileLoader {
    fn load(&self, root_path: &str) -> Result<FileList, String> {
        let path = PathBuf::from(root_path);

        if !path.exists() {
            return Err(format!("path {} does not exist", path.to_str().unwrap()));
        }

        if !path.is_dir() {
            return Err(format!("path {} is not a directory", path.to_str().unwrap()));
        }

        let mut file_list: FileList = FileList { files: vec![] };
        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            if !entry.file_type().is_file() {
                continue;
            }

            file_list.files.push(entry.path().to_str().unwrap().to_string());
        }

        Ok(file_list)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_should_return_correct_files() {
        let loader = LocalFileLoader::new();
        let file_list = loader.load("src/infrastructure/folder_loader/test_data/example_project").unwrap();

        assert_eq!(file_list.files.len(), 2);
        assert_eq!(
            file_list.files[0],
            "src/infrastructure/folder_loader/test_data/example_project/test_dir1/test2.txt"
        );
        assert_eq!(file_list.files[1], "src/infrastructure/folder_loader/test_data/example_project/test.txt");
    }

    #[test]
    fn test_load_should_return_error_when_path_does_not_exist() {
        let loader = LocalFileLoader::new();
        let result = loader.load("src/infrastructure/folder_loader/test_data/example_project_not_exists");

        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            "path src/infrastructure/folder_loader/test_data/example_project_not_exists does not exist"
        );
    }

    #[test]
    fn test_load_should_return_error_when_path_is_not_a_directory() {
        let loader = LocalFileLoader::new();
        let result = loader.load("src/infrastructure/folder_loader/test_data/example_project/test.txt");

        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            "path src/infrastructure/folder_loader/test_data/example_project/test.txt is not a directory"
        );
    }
}
