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
}
