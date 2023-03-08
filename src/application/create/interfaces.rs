use mockall::automock;

use crate::domain::file_tree::FileTree;

#[automock]
pub trait FileTreeLoader {
    fn load(&self, path: &str) -> Result<FileTree, String>;
}
