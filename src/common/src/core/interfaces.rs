#[cfg(test)]
use mockall::automock;

/// interface for the file system
#[cfg_attr(test, automock)]
#[async_trait::async_trait]
pub trait FileSystemInterface: Send + Sync {
    /// Clear the folder and his subfolders
    async fn clear_folder(&self, path: String) -> Result<(), String>;

    /// move file from source to target
    async fn move_file(&self, source_path: String, target_path: String) -> Result<(), String>;

    /// read file
    async fn read_file(&self, path: String) -> Result<String, String>;

    /// write file
    async fn write_file(&self, path: String, content: String) -> Result<(), String>;

    /// read file buffered
    async fn read_file_buffered(&self, path: String) -> Result<Vec<String>, String>;

    /// write line to file
    async fn write_line_to_file(&self, path: &str, content: String) -> Result<(), String>;
}
