use common::core::file::File;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct FileList {
    pub root_path: PathBuf,
    pub files: Vec<File>,
}

impl FileList {
    /// Creates a new instance of FileList.
    pub fn new() -> Self {
        Self {
            root_path: PathBuf::new(),
            files: Vec::new(),
        }
    }

    /// add file path
    pub fn add(&mut self, file_path: File) {
        self.files.push(file_path)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_file_list() {
        let file_list = FileList {
            root_path: PathBuf::from_str("/tmp").unwrap(),
            files: vec![File::from("test")],
        };

        assert_eq!(
            file_list,
            FileList {
                root_path: PathBuf::from_str("/tmp").unwrap(),
                files: vec![File::from("test")]
            }
        );
    }

    #[test]
    fn test_add_file_path() {
        let mut file_list = FileList::default();

        file_list.add(File::from("test"));

        assert_eq!(
            file_list,
            FileList {
                root_path: PathBuf::new(),
                files: vec![File::from("test")]
            }
        )
    }
}
