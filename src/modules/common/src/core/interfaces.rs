use crate::core::errors::Result;
use std::path::Path;

#[cfg(test)]
use mockall::automock;

/// interface for the file system
#[cfg_attr(test, automock)]
#[async_trait::async_trait]
pub trait FileSystemInterface: Send + Sync {
    /// Clear the folder and his subfolders
    async fn clear_folder(&self, path: &Path) -> Result<()>;

    /// move file from source to target
    async fn move_file(&self, source_path: &Path, target_path: &Path) -> Result<()>;

    /// read file
    async fn read_file(&self, path: &Path) -> Result<String>;

    /// write file
    async fn write_file(&self, path: &Path, content: &str) -> Result<()>;

    /// read file buffered
    async fn read_file_buffered(&self, path: &Path) -> Result<Vec<String>>;

    /// check if the file is an image
    async fn is_binary(&self, path: &Path) -> Result<bool>;
}
