use crate::core::interfaces::FileSystemInterface;
use crate::core::{errors::Error, errors::Result, file::File};
use std::io::Write;
use std::path::Path;
use tokio::{
    fs::OpenOptions,
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
};

#[derive(Default)]
pub struct FileSystem {}

#[async_trait::async_trait]
impl FileSystemInterface for FileSystem {
    async fn clear_folder(&self, path: &Path) -> Result<()> {
        tokio::fs::remove_dir_all(path)
            .await
            .map_err(|e| Error::new(format!("issue to clear: {}", e)))?;
        Ok(())
    }

    async fn move_file(&self, source_file: &File, target_file: &File) -> Result<()> {
        let target_dir = target_file.path().parent();
        if target_dir.is_none() {
            return Err(Error::new("issue to get dir".into()));
        }

        let target_dir = target_dir.unwrap().to_str();
        if target_dir.is_none() {
            return Err(Error::new("issue to str".into()));
        }

        let target_dir = target_dir.unwrap();
        tokio::fs::create_dir_all(target_dir)
            .await
            .map_err(|e| Error::new(format!("issue to create target directory: {}", e)))?;

        tokio::fs::copy(source_file, target_file)
            .await
            .map_err(|e| Error::new(format!("issue to move copy file: {}", e)))?;

        Ok(())
    }

    async fn read_file(&self, path: &File) -> Result<String> {
        let content_bytes = tokio::fs::read(path)
            .await
            .map_err(|e| Error::new(format!("issue to read file: {}", e)))?;
        let content = std::str::from_utf8(&content_bytes);
        let content = match content {
            Ok(content) => content,
            Err(_e) => "",
        };

        Ok(content.into())
    }

    async fn write_file(&self, path: &File, content: &str) -> Result<()> {
        let dir = path.path().parent().unwrap();

        tokio::fs::create_dir_all(dir)
            .await
            .map_err(|e| Error::new(format!("issue to create target directory:{}", e)))?;

        tokio::fs::write(path, content)
            .await
            .map_err(|e| Error::new(format!("issue to write file: {}", e)))?;

        Ok(())
    }

    async fn read_file_buffered(&self, path: &File) -> Result<Vec<String>> {
        let file = tokio::fs::File::open(path).await.unwrap();
        let reader = BufReader::new(file);

        let mut result_lines = vec![];
        let mut lines = reader.lines();
        while let Some(line) = lines.next_line().await.unwrap() {
            result_lines.push(line);
        }

        Ok(result_lines)
    }

    async fn write_line_to_file(&self, path: &File, content: String) -> Result<()> {
        let mut file = OpenOptions::new().write(true).append(true).open(path).await.unwrap();

        let mut buffer = Vec::<u8>::new();
        writeln!(buffer, "{}", content).expect("issue to write to buffer");

        file.write_all(&buffer).await.expect("issue to write to file");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::file::File;

    #[tokio::test]
    async fn test_read_file_buffered() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("my-temporary-note.txt");
        let file_path = File::from(file_path);
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
}
