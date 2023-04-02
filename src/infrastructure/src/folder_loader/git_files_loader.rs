use application::create::{file_list::FileList, interfaces::FileListLoader};
use std::{path::PathBuf, process::Command, str::FromStr};

pub struct GitFileListLoader<'a> {
    file_list_loader: &'a dyn FileListLoader,
    local_storage_path: String,
    branch_name: String,
}

impl<'a> GitFileListLoader<'a> {
    pub fn new(file_list_loader: &'a dyn FileListLoader, local_storage_path: String, branch_name: String) -> Self {
        Self {
            file_list_loader,
            local_storage_path,
            branch_name,
        }
    }

    fn get_git_name(&self, git_url: &str) -> String {
        let git_url_split = git_url.split('/').collect::<Vec<&str>>();
        let git_name = git_url_split[git_url_split.len() - 1];
        git_name.replace(".git", "")
    }

    fn execute_git_clone(&self, git_url: &str) -> Result<(), String> {
        let git_name = self.get_git_name(git_url);
        let path_which_is_cloned = format!("{}{}", self.local_storage_path, git_name);
        let mut git_clone_cmd = Command::new("git");
        git_clone_cmd
            .arg("clone")
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

impl<'a> FileListLoader for GitFileListLoader<'a> {
    fn load(&self, remote_git_url: &str) -> Result<FileList, String> {
        let repo_name = self.get_git_name(remote_git_url);
        let path_which_is_cloned = format!("{}{}", self.local_storage_path, repo_name);

        self.try_remove_cloned_folder(&path_which_is_cloned)?;

        self.execute_git_clone(remote_git_url)?;

        let files = self.file_list_loader.load(&path_which_is_cloned)?;

        let filterd_files = self.remove_hidden_git_folder(files);

        Ok(filterd_files)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::folder_loader::local_file_loader::LocalFileListLoader;
    use std::{path::PathBuf, str::FromStr};

    #[test]
    fn test_try_remove_cloned_folder_should_remove_folder() {
        let file_list_loader_mock = LocalFileListLoader::default();
        let sut = GitFileListLoader::new(&file_list_loader_mock, "/tmp/".to_string(), "main".to_string());

        sut.try_remove_cloned_folder("/tmp/test").expect("failed to remove folder");

        // check if folder is removed
        let path_which_is_cloned = PathBuf::from_str("/tmp/test").unwrap();
        assert!(!path_which_is_cloned.exists());
    }

    #[test]
    fn test_get_git_name_should_return_correct_name() {
        let file_list_loader_mock = LocalFileListLoader::default();
        let sut = GitFileListLoader::new(&file_list_loader_mock, "/tmp/".to_string(), "main".to_string());

        let git_name = sut.get_git_name("https://github.com/BuriKizilkaya/creatorly.git");

        assert_eq!(git_name, "creatorly");
    }

    #[test]
    fn test_get_git_name_from_azuredevops_return_correct_name() {
        let file_list_loader_mock = LocalFileListLoader::default();
        let sut = GitFileListLoader::new(&file_list_loader_mock, "/tmp/".to_string(), "main".to_string());

        let files = sut.load("https://kizilkaya-lab@dev.azure.com/kizilkaya-lab/Demo/_git/Demo");

        assert!(files.is_ok());
        assert!(files.clone().unwrap().files.len() > 0);
    }

    #[test]
    fn test_load_should_return_correct_files() {
        let file_list_loader_mock = LocalFileListLoader::default();
        let sut = GitFileListLoader::new(&file_list_loader_mock, "/tmp/".to_string(), "main".to_string());

        let files: Result<FileList, String> = sut.load("https://github.com/BuriKizilkaya/creatorly.git");

        assert!(files.is_ok());
        assert!(files.clone().unwrap().files.len() > 0);

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
