use crate::application::common::interfaces::Os;
use std::{fs, path::PathBuf};

pub struct FileSystem {}

impl Os for FileSystem {
    fn clear_folder(&self, path: String) -> Result<(), String> {
        fs::remove_dir_all(path).expect("issue to remove");
        Ok(())
    }

    fn move_file(&self, source_file: String, target_file: String) -> Result<(), String> {
        let target_file = PathBuf::from(target_file);
        let target_dir = target_file.parent().expect("issue to get dir").to_str().expect("issue to str");

        std::fs::create_dir_all(target_dir).expect("issue to create target directory");
        std::fs::copy(source_file, target_file.clone()).expect("issue to copy");

        Ok(())
    }

    fn read_file(&self, path: String) -> Result<String, String> {
        let content = fs::read_to_string(path).expect("issue to read file");
        Ok(content)
    }

    fn write_file(&self, path: String, content: String) -> Result<(), String> {
        let path = PathBuf::from(path);
        let dir = path.parent().expect("issue to get dir").to_str().expect("issue to str");

        std::fs::create_dir_all(dir).expect("issue to create target directory");
        std::fs::write(path, content).expect("issue to write");

        Ok(())
    }
}
