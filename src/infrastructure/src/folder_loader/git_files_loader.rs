use application::generate::{file_list::FileList, interfaces::FileListLoader};
use std::{path::PathBuf, process::Command, str::FromStr};

use super::local_file_loader::LocalFileListLoader;

pub struct GitFileListLoader {
    local_storage_path: String,
    branch_name: String,
    remote_git_url: String,
}

impl GitFileListLoader {
    pub fn new(remote_git_url: String, local_storage_path: String, branch_name: String) -> Self {
        Self {
            local_storage_path,
            branch_name,
            remote_git_url,
        }
    }

    fn get_git_name(&self) -> String {
        let git_url_split = self.remote_git_url.split('/').collect::<Vec<&str>>();
        let git_name = git_url_split[git_url_split.len() - 1];
        git_name.replace(".git", "")
    }

    fn execute_git_clone(&self, git_url: &str) -> Result<(), String> {
        let git_name = self.get_git_name();
        let path_which_is_cloned = format!("{}{}", self.local_storage_path, git_name);
        let mut git_clone_cmd = Command::new("git");
        git_clone_cmd
            .arg("clone")
            .arg("--recurse-submodules")
            .arg("--branch")
            .arg(&self.branch_name)
            .arg(git_url)
            .arg(&path_which_is_cloned);

        let output = git_clone_cmd.output().expect("failed to execute git clone");
        if !output.status.success() {
            return Err(format!("git clone failed: {}", String::from_utf8_lossy(&output.stderr)));
        }

        Ok(())
    }

    fn try_remove_cloned_folder(&self, path: &str) -> Result<(), String> {
        let path_which_is_cloned = PathBuf::from_str(path).expect("failed to parse path");
        if path_which_is_cloned.exists() {
            let result = std::fs::remove_dir_all(path_which_is_cloned);
            if result.is_err() {
                return Err(format!("failed to remove folder: {}", path));
            }
        }

        Ok(())
    }

    fn remove_hidden_git_folder(&self, files: FileList) -> FileList {
        let mut filtered_files = files;

        filtered_files.files.retain(|file| !file.contains(".git/"));

        filtered_files
    }
}

impl FileListLoader for GitFileListLoader {
    fn load(&self) -> Result<FileList, String> {
        let repo_name = self.get_git_name();
        let path_which_is_cloned = format!("{}{}", self.local_storage_path, repo_name);

        self.try_remove_cloned_folder(&path_which_is_cloned)?;

        self.execute_git_clone(&self.remote_git_url)?;

        let file_list_loader = LocalFileListLoader::new(path_which_is_cloned);
        let files = file_list_loader.load()?;

        let filterd_files = self.remove_hidden_git_folder(files);

        Ok(filterd_files)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{path::PathBuf, str::FromStr};

    #[test]
    fn test_try_remove_cloned_folder_should_remove_folder() {
        let sut = GitFileListLoader::new(
            "https://github.com/BuriKizilkaya/creatorly.git".to_string(),
            "/tmp/".to_string(),
            "main".to_string(),
        );

        sut.try_remove_cloned_folder("/tmp/test")
            .expect("failed to remove folder");

        // check if folder is removed
        let path_which_is_cloned = PathBuf::from_str("/tmp/test").unwrap();
        assert!(!path_which_is_cloned.exists());
    }

    #[test]
    fn test_get_git_name_from_github_should_return_correct_name() {
        let sut = GitFileListLoader::new(
            "https://github.com/BuriKizilkaya/creatorly.git".to_string(),
            "/tmp/".to_string(),
            "main".to_string(),
        );

        let git_name = sut.get_git_name();

        assert_eq!(git_name, "creatorly");
    }

    #[test]
    fn test_get_git_name_from_azuredevops_return_correct_name() {
        let sut = GitFileListLoader::new(
            "https://kizilkaya-lab@dev.azure.com/kizilkaya-lab/Demo/_git/Demo".to_string(),
            "/tmp/".to_string(),
            "main".to_string(),
        );

        let git_name = sut.get_git_name();

        assert_eq!(git_name, "Demo");
    }

    #[test]
    fn test_load_should_return_correct_files() {
        let sut = GitFileListLoader::new(
            "https://kizilkaya-lab@dev.azure.com/kizilkaya-lab/Demo/_git/Demo".to_string(),
            "/tmp/".to_string(),
            "main".to_string(),
        );

        let files: Result<FileList, String> = sut.load();

        assert!(files.is_ok());
        assert!(!files.clone().unwrap().files.is_empty());

        // check if all .git folder is removed
        let found_files_indexes = files
            .unwrap()
            .files
            .iter()
            .enumerate()
            .filter(|file| file.1.contains(".git/"))
            .map(|(index, _)| index)
            .collect::<Vec<_>>();

        assert_eq!(found_files_indexes.len(), 0);
    }
}
