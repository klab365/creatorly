use common::core::errors::{Error, Result};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::templatespecification::core::interfaces::FileListLoader;

#[derive(Clone, Default)]
pub struct LocalFileListLoader {}

impl LocalFileListLoader {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl FileListLoader for LocalFileListLoader {
    async fn load(&self, path: &Path) -> Result<Vec<PathBuf>> {
        if !path.exists() {
            return Err(Error::new(format!("path {} does not exist", path.to_str().unwrap())));
        }

        if !path.is_dir() {
            return Err(Error::new(format!(
                "path {} is not a directory",
                path.to_str().unwrap()
            )));
        }

        let mut files = vec![];
        for entry in WalkDir::new(path).follow_links(true).into_iter().filter_map(|e| e.ok()) {
            if !entry.file_type().is_file() {
                continue;
            }

            let path = entry.path().display().to_string();
            files.push(path.into());
        }

        Ok(files)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[tokio::test]
    async fn test_load_should_return_correct_files() {
        let mut example_project_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        example_project_path.push("resources/test");
        example_project_path.push("test_project");
        let loader = LocalFileListLoader::default();
        let file_list = loader.load(&example_project_path).await.unwrap();

        let file_entry1 = example_project_path.join("test_dir1").join("test2.txt");
        let file_entry2 = example_project_path.join("test.txt");

        assert_eq!(file_list.len(), 2);
        assert!(file_list.contains(&file_entry1));
        assert!(file_list.contains(&file_entry2));
    }

    #[tokio::test]
    async fn test_load_should_return_error_when_path_does_not_exist() {
        let mut path = String::from(env!("CARGO_MANIFEST_DIR"));
        path.push_str("/resources/test/test_project_not_exists");
        let loader = LocalFileListLoader::default();

        let path = PathBuf::from_str("src/infrastructure/folder_loader/test_data/example_project_not_exists").unwrap();
        let result = loader.load(&path).await;

        assert!(result.is_err());

        assert_eq!(
            result.err().unwrap().to_string(),
            "path src/infrastructure/folder_loader/test_data/example_project_not_exists does not exist"
        );
    }

    #[tokio::test]
    async fn test_load_should_return_error_when_path_is_not_a_directory() {
        let mut path = String::from(env!("CARGO_MANIFEST_DIR"));
        path.push_str("/resources/test/test_project/test.txt");
        let path = PathBuf::from_str(&path).unwrap();
        let loader = LocalFileListLoader::default();
        let result = loader.load(&path).await;

        assert!(result.is_err());
    }
}
