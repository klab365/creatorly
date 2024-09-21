use crate::core::interfaces::FileSystemInterface;
use crate::core::{errors::Error, errors::Result};
use std::path::Path;
use tokio::io::{AsyncBufReadExt, BufReader};

#[derive(Default)]
pub struct FileSystem {}

#[async_trait::async_trait]
impl FileSystemInterface for FileSystem {
    async fn clear_folder(&self, path: &Path) -> Result<()> {
        if !path.is_dir() {
            return Ok(()); // skip if it is not a directory or does not exist
        }

        tokio::fs::remove_dir_all(path)
            .await
            .map_err(|e| Error::new(format!("issue to clear: {}", e)))?;
        Ok(())
    }

    async fn move_file(&self, source_file: &Path, target_file: &Path) -> Result<()> {
        let Some(target_dir) = target_file.parent() else {
            return Err(Error::new("issue to get dir".into()));
        };

        tokio::fs::create_dir_all(target_dir)
            .await
            .map_err(|e| Error::new(format!("issue to create target directory: {}", e)))?;

        tokio::fs::copy(source_file, target_file)
            .await
            .map_err(|e| Error::new(format!("issue to move copy file: {}", e)))?;

        Ok(())
    }

    async fn read_file(&self, path: &Path) -> Result<String> {
        let content_bytes = tokio::fs::read(path)
            .await
            .map_err(|e| Error::new(format!("issue to read file: {}", e)))?;
        let content =
            std::str::from_utf8(&content_bytes).map_err(|e| Error::new(format!("issue to convert to utf8: {}", e)))?;

        Ok(content.into())
    }

    async fn write_file(&self, path: &Path, content: &str) -> Result<()> {
        let dir = path.parent();
        let Some(dir) = dir else {
            return Err(Error::new("issue to get dir".into()));
        };

        tokio::fs::create_dir_all(dir)
            .await
            .map_err(|e| Error::new(format!("issue to create target directory:{}", e)))?;

        tokio::fs::write(path, content)
            .await
            .map_err(|e| Error::new(format!("issue to write file: {}", e)))?;

        Ok(())
    }

    async fn read_file_buffered(&self, path: &Path) -> Result<Vec<String>> {
        let file = tokio::fs::File::open(path).await.unwrap();
        let reader = BufReader::new(file);

        let mut result_lines = vec![];
        let mut lines = reader.lines();
        while let Some(line) = lines.next_line().await.unwrap() {
            result_lines.push(line);
        }

        Ok(result_lines)
    }

    async fn is_binary(&self, path: &Path) -> Result<bool> {
        let res = self.read_file(path).await;

        if res.is_err() {
            return Ok(true);
        }

        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tokio::io::AsyncWriteExt as _;

    #[tokio::test]
    async fn test_read_file_buffered() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("my-temporary-note.txt");
        let mut file = tokio::fs::File::create(&file_path).await.unwrap();

        let mut buffer = Vec::<u8>::new();
        writeln!(buffer, "file1\nfile2\nfile3").unwrap();
        file.write_all(&buffer).await.unwrap();

        let file_system = FileSystem {};
        let lines = file_system.read_file_buffered(&file_path).await.unwrap();

        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "file1");
        assert_eq!(lines[1], "file2");
        assert_eq!(lines[2], "file3");
    }

    #[tokio::test]
    async fn test_clear_folder_should_remove_all_files() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("my-temporary-note.txt");
        let mut file = tokio::fs::File::create(&file_path).await.unwrap();

        let mut buffer = Vec::<u8>::new();
        writeln!(buffer, "file1\nfile2\nfile3").unwrap();
        file.write_all(&buffer).await.unwrap();

        let file_system = FileSystem {};
        file_system.clear_folder(dir.path()).await.unwrap();

        let is_dir_exists = Path::new(&dir.path()).is_dir();
        assert!(!is_dir_exists);
    }

    #[tokio::test]
    async fn test_move_file_should_move_file() {
        let dir = tempfile::tempdir().unwrap();
        let source_file_path = dir.path().join("source.txt");
        let target_file_path = dir.path().join("target.txt");

        let mut file = tokio::fs::File::create(&source_file_path).await.unwrap();
        let mut buffer = Vec::<u8>::new();
        writeln!(buffer, "file1\nfile2\nfile3").unwrap();
        file.write_all(&buffer).await.unwrap();

        let file_system = FileSystem {};
        file_system
            .move_file(&source_file_path, &target_file_path)
            .await
            .unwrap();

        let is_source_file_exists = Path::new(&source_file_path).exists();
        let is_target_file_exists = Path::new(&target_file_path).exists();
        assert!(is_source_file_exists);
        assert!(is_target_file_exists);
    }

    #[tokio::test]
    async fn test_read_file_should_read_file() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("my-temporary-note.txt");
        let mut file = tokio::fs::File::create(&file_path).await.unwrap();

        let mut buffer = Vec::<u8>::new();
        writeln!(buffer, "file1\nfile2\nfile3").unwrap();
        file.write_all(&buffer).await.unwrap();

        let file_system = FileSystem {};
        let content = file_system.read_file(&file_path).await.unwrap();

        assert_eq!(content, "file1\nfile2\nfile3\n");
    }

    #[tokio::test]
    async fn test_is_binary_should_return_false_for_txt_file() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("my-temporary-note.txt");
        let mut binary_file = tokio::fs::File::create(&file_path).await.unwrap();

        let mut buffer = Vec::<u8>::new();
        writeln!(buffer, "file1\nfile2\nfile3").unwrap();
        binary_file.write_all(&buffer).await.unwrap();

        let file_system = FileSystem {};
        let is_binary = file_system.is_binary(&file_path).await.unwrap();

        assert!(!is_binary);
    }

    #[tokio::test]
    async fn test_is_binary_should_return_true_for_binary_file() {
        let resource_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("resources");
        let file_path = resource_dir.join("yoda.png");

        let file_system = FileSystem {};
        let is_binary = file_system.is_binary(&file_path).await.unwrap();

        assert!(is_binary);
    }
}
