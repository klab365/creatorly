use std::path::Path;

use crate::core::errors::Result;
use crate::core::file::File;

#[cfg(test)]
use mockall::automock;

/// interface for the file system
#[cfg_attr(test, automock)]
#[async_trait::async_trait]
pub trait FileSystemInterface: Send + Sync {
    /// Clear the folder and his subfolders
    async fn clear_folder(&self, path: &Path) -> Result<()>;

    /// move file from source to target
    async fn move_file(&self, source_path: &File, target_path: &File) -> Result<()>;

    /// read file
    async fn read_file(&self, path: &File) -> Result<String>;

    /// write file
    async fn write_file(&self, path: &File, content: &str) -> Result<()>;

    /// read file buffered
    async fn read_file_buffered(&self, path: &File) -> Result<Vec<String>>;

    /// write line to file
    async fn write_line_to_file(&self, path: &File, content: String) -> Result<()>;

    /// check if the file is an image
    async fn is_binary(&self, path: &File) -> Result<bool>;
}
