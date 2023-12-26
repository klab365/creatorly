#[derive(Debug, Clone, PartialEq, Default)]
pub struct FileList {
    pub files: Vec<String>,
}

impl FileList {
    /// Creates a new instance of FileList.
    pub fn new() -> Self {
        Self { files: Vec::new() }
    }

    /// add file path
    pub fn add(&mut self, file_path: String) {
        self.files.push(file_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_list() {
        let file_list = FileList {
            files: vec!["test".to_string()],
        };

        assert_eq!(
            file_list,
            FileList {
                files: vec!["test".to_string()]
            }
        );
    }

    #[test]
    fn test_add_file_path() {
        let mut file_list = FileList::default();

        file_list.add("newpath".to_string());

        assert_eq!(
            file_list,
            FileList {
                files: vec!["newpath".to_string()]
            }
        )
    }
}
