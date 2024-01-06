use crate::core::{file_list::FileList, interfaces::FileListLoader};
use common::core::{
    errors::{Error, Result},
    file::File,
};
use std::path::PathBuf;
use tokio::process::Command;

use super::local_file_loader::LocalFileListLoader;

pub struct GitFileListLoader {
    branch_name: String,
    remote_git_url: String,
}

/// Implementation of a Git file list loader.
impl GitFileListLoader {
    /// The path where the downloaded files will be stored.
    pub const DOWNLOAD_FOLDER_PATH: &'static str = "/tmp";

    /// Creates a new instance of the GitFileListLoader.
    ///
    /// # Arguments
    ///
    /// * `remote_git_url` - The URL of the remote Git repository.
    /// * `branch_name` - The name of the branch to clone.
    ///
    /// # Returns
    ///
    /// A new instance of the GitFileListLoader.
    pub fn new(remote_git_url: String, branch_name: String) -> Self {
        Self {
            branch_name,
            remote_git_url,
        }
    }

    /// Extracts the name of the Git repository from the remote Git URL.
    ///
    /// # Returns
    ///
    /// The name of the Git repository.
    fn get_git_name(&self) -> String {
        let git_url_split = self.remote_git_url.split('/').collect::<Vec<&str>>();
        let git_name = git_url_split[git_url_split.len() - 1];
        git_name.replace(".git", "")
    }

    /// Executes the `git clone` command to clone the remote Git repository.
    ///
    /// # Arguments
    ///
    /// * `git_url` - The URL of the remote Git repository.
    /// * `destination_path` - The path where the repository will be cloned.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    async fn execute_git_clone(&self, git_url: &str, destination_path: PathBuf) -> Result<()> {
        let mut git_clone_cmd = Command::new("git");
        git_clone_cmd
            .arg("clone")
            .arg("--recurse-submodules")
            .arg("--branch")
            .arg(&self.branch_name)
            .arg(git_url)
            .arg(&destination_path);

        let output = git_clone_cmd
            .output()
            .await
            .map_err(|_| Error::new("Failed to execute".into()))?;
        if !output.status.success() {
            return Err(Error::new(format!(
                "git clone failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        Ok(())
    }

    /// Tries to remove the cloned folder if it exists.
    ///
    /// # Arguments
    ///
    /// * `path` - The path of the cloned folder.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    async fn try_remove_cloned_folder(&self, path: &PathBuf) -> Result<()> {
        if path.exists() {
            tokio::fs::remove_dir_all(path)
                .await
                .map_err(|_| Error::new(format!("failed to remove folder: {}", path.display())))?;
        }

        Ok(())
    }

    /// Removes the hidden `.git` folder from the list of files.
    ///
    /// # Arguments
    ///
    /// * `files` - The list of files.
    ///
    /// # Returns
    ///
    /// The filtered list of files with the hidden `.git` folder removed.
    fn remove_hidden_git_folder(&self, files: FileList) -> FileList {
        let mut filtered_files = files;

        filtered_files.files.retain(|file| !file.contains(File::from(".git/")));

        filtered_files
    }
}

#[async_trait::async_trait]
impl FileListLoader for GitFileListLoader {
    async fn load(&self, path: Option<PathBuf>) -> Result<FileList> {
        let repo_name = self.get_git_name();

        let download_path = match path {
            Some(path) => path,
            None => PathBuf::from(Self::DOWNLOAD_FOLDER_PATH),
        };

        let download_path = download_path.join(repo_name);
        self.try_remove_cloned_folder(&download_path).await?;
        self.execute_git_clone(&self.remote_git_url, download_path.clone())
            .await?;

        let file_list_loader = LocalFileListLoader::default();
        let files = file_list_loader.load(Some(download_path)).await?;
        let filterd_files = self.remove_hidden_git_folder(files);

        Ok(filterd_files)
    }
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;
    use std::{path::PathBuf, str::FromStr};

    #[tokio::test]
    async fn test_try_remove_cloned_folder_should_remove_folder() {
        let temp_dir = TempDir::new().unwrap();
        let test_dir = temp_dir.path().join("test");
        tokio::fs::create_dir_all(&test_dir).await.unwrap();
        let sut = GitFileListLoader::new(
            "https://github.com/BuriKizilkaya/creatorly.git".to_string(),
            "main".to_string(),
        );

        sut.try_remove_cloned_folder(&test_dir)
            .await
            .expect("failed to remove folder");

        // check if folder is removed
        let path_which_is_cloned = PathBuf::from_str("/tmp/test").unwrap();
        assert!(!path_which_is_cloned.exists());
    }

    #[test]
    fn test_get_git_name_from_github_should_return_correct_name() {
        let sut = GitFileListLoader::new(
            "https://github.com/BuriKizilkaya/creatorly.git".to_string(),
            "main".to_string(),
        );

        let git_name = sut.get_git_name();

        assert_eq!(git_name, "creatorly");
    }

    #[test]
    fn test_get_git_name_from_azuredevops_return_correct_name() {
        let sut = GitFileListLoader::new(
            "https://kizilkaya-lab@dev.azure.com/kizilkaya-lab/Demo/_git/Demo".to_string(),
            "main".to_string(),
        );

        let git_name = sut.get_git_name();

        assert_eq!(git_name, "Demo");
    }

    #[tokio::test]
    async fn test_load_should_return_correct_files() {
        let sut = GitFileListLoader::new(
            "https://kizilkaya-lab@dev.azure.com/kizilkaya-lab/Demo/_git/Demo".to_string(),
            "main".to_string(),
        );

        let path = PathBuf::from_str("/tmp").unwrap();
        let files: Result<FileList> = sut.load(Some(path)).await;

        assert!(files.is_ok());
        assert!(!files.clone().unwrap().files.is_empty());

        // check if all .git folder is removed
        let found_files_indexes = files
            .clone()
            .unwrap()
            .files
            .iter()
            .enumerate()
            .filter(|file| file.1.contains(File::from(".git/")))
            .map(|(index, _)| index)
            .collect::<Vec<_>>();

        assert_eq!(found_files_indexes.len(), 0);
        assert_eq!(files.unwrap().files.len(), 1);
    }
}
