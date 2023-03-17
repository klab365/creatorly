#[derive(Debug, Clone, PartialEq)]
pub struct FileList {
    pub files: Vec<String>,
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
}
