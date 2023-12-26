use crate::core::interfaces::FileSystemInterface;
use std::io::Write;
use std::path::PathBuf;
use tokio::{
    fs::OpenOptions,
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
};

pub struct FileSystem {}

#[async_trait::async_trait]
impl FileSystemInterface for FileSystem {
    async fn clear_folder(&self, path: String) -> Result<(), String> {
        tokio::fs::remove_dir_all(path).await.expect("issue to remove");
        Ok(())
    }

    async fn move_file(&self, source_file: String, target_file: String) -> Result<(), String> {
        let target_file = PathBuf::from(target_file);
        let target_dir = target_file
            .parent()
            .expect("issue to get dir")
            .to_str()
            .expect("issue to str");

        tokio::fs::create_dir_all(target_dir)
            .await
            .expect("issue to create target directory");

        tokio::fs::copy(source_file, target_file.clone())
            .await
            .expect("issue to copy");

        Ok(())
    }

    async fn read_file(&self, path: String) -> Result<String, String> {
        let content = tokio::fs::read_to_string(path).await.expect("issue to read file");
        Ok(content)
    }

    async fn write_file(&self, path: String, content: String) -> Result<(), String> {
        let path = PathBuf::from(path);
        let dir = path.parent().unwrap();

        tokio::fs::create_dir_all(dir)
            .await
            .expect("issue to create target directory");

        tokio::fs::write(path, content).await.expect("issue to write");

        Ok(())
    }

    async fn read_file_buffered(&self, path: String) -> Result<Vec<String>, String> {
        let path = PathBuf::from(path);

        let file = tokio::fs::File::open(path).await.unwrap();
        let reader = BufReader::new(file);

        let mut result_lines = vec![];
        let mut lines = reader.lines();
        while let Some(line) = lines.next_line().await.unwrap() {
            result_lines.push(line);
        }

        Ok(result_lines)
    }

    async fn write_line_to_file(&self, path: &str, content: String) -> Result<(), String> {
        let path = PathBuf::from(path);

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
    use tokio::fs::File;

    #[tokio::test]
    async fn test_read_file_buffered() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("my-temporary-note.txt");
        let mut file = File::create(file_path.to_str().unwrap().to_string().clone())
            .await
            .unwrap();

        let mut buffer = Vec::<u8>::new();
        writeln!(buffer, "file1\nfile2\nfile3").unwrap();
        file.write_all(&buffer).await.unwrap();

        let file_system = FileSystem {};
        let lines = file_system
            .read_file_buffered(file_path.to_str().unwrap().to_string())
            .await
            .unwrap();

        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "file1");
        assert_eq!(lines[1], "file2");
        assert_eq!(lines[2], "file3");
    }
}
