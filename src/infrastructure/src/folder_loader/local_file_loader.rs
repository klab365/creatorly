use application::create::{file_list::FileList, interfaces::FileListLoader};
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Clone)]
pub struct LocalFileListLoader {
    local_path: String,
}

impl LocalFileListLoader {
    pub fn new(local_path: String) -> Self {
        Self { local_path }
    }
}

impl FileListLoader for LocalFileListLoader {
    fn load(&self) -> Result<FileList, String> {
        let path = PathBuf::from(self.local_path.clone());

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
        let mut example_project_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        example_project_path.push("resources/test");
        example_project_path.push("test_project");
        let loader = LocalFileListLoader::new(example_project_path.to_str().unwrap().to_string());
        let file_list = loader.load().unwrap();

        let file_entry1 = example_project_path.join("test_dir1").join("test2.txt");
        let file_entry2 = example_project_path.join("test.txt");

        assert_eq!(file_list.files.len(), 2);
        assert!(file_list.files.contains(&file_entry1.to_str().unwrap().to_string()));
        assert!(file_list.files.contains(&file_entry2.to_str().unwrap().to_string()));
    }

    #[test]
    fn test_load_should_return_error_when_path_does_not_exist() {
        let mut path = String::from(env!("CARGO_MANIFEST_DIR"));
        path.push_str("/resources/test/test_project_not_exists");
        let loader = LocalFileListLoader::new(
            "src/infrastructure/folder_loader/test_data/example_project_not_exists".to_string(),
        );
        let result = loader.load();

        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            "path src/infrastructure/folder_loader/test_data/example_project_not_exists does not exist"
        );
    }

    #[test]
    fn test_load_should_return_error_when_path_is_not_a_directory() {
        let mut path = String::from(env!("CARGO_MANIFEST_DIR"));
        path.push_str("/resources/test/test_project/test.txt");
        let loader = LocalFileListLoader::new(path.clone());
        let result = loader.load();

        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), format!("path {} is not a directory", path));
    }
}
