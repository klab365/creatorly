use mockall::automock;

use crate::domain::folder_tree::Folder;

#[automock]
pub trait FolderLoader {
    fn load(&self, path: &str) -> Result<Folder, String>;
}
