use application::create::{file_list::FileList, interfaces::FileListLoader};
use std::path::PathBuf;
use walkdir::WalkDir;

pub struct LocalFileListLoader {}

impl LocalFileListLoader {
    fn new() -> Self {
        Self {}
    }
}

impl Default for LocalFileListLoader {
    fn default() -> Self {
        Self::new()
    }
}

impl FileListLoader for LocalFileListLoader {
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
        let loader = LocalFileListLoader::new();
        let mut example_project_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        example_project_path.push("resources/test");
        example_project_path.push("test_project");
        let file_list = loader.load(&example_project_path.to_str().unwrap().to_string()).unwrap();

        let file_entry1 = example_project_path.join("test_dir1").join("test2.txt");
        let file_entry2 = example_project_path.join("test.txt");

        assert_eq!(file_list.files.len(), 2);
        assert_eq!(file_list.files.contains(&file_entry1.to_str().unwrap().to_string()), true);
        assert_eq!(file_list.files.contains(&file_entry2.to_str().unwrap().to_string()), true);
    }

    #[test]
    fn test_load_should_return_error_when_path_does_not_exist() {
        let mut path = String::from(env!("CARGO_MANIFEST_DIR"));
        path.push_str("/resources/test/test_project_not_exists");
        let loader = LocalFileListLoader::new();
        let result = loader.load("src/infrastructure/folder_loader/test_data/example_project_not_exists");

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
        let loader = LocalFileListLoader::new();
        let result = loader.load(&path);

        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), format!("path {} is not a directory", path));
    }
}
