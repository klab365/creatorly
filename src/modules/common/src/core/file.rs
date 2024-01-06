use std::{
    fmt,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct File {
    path: PathBuf,
}

impl File {
    pub fn to_str(&self) -> &str {
        self.path.to_str().unwrap_or_default()
    }

    pub fn contains(&self, file: File) -> bool {
        self.path.to_str().unwrap_or_default().contains(file.to_str())
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn replace(&self, from: &str, to: &str) -> File {
        let path = self.path.to_str().unwrap_or_default().replace(from, to);
        File::from(path)
    }
}

impl AsRef<Path> for File {
    fn as_ref(&self) -> &Path {
        &self.path
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.path.display())
    }
}

impl From<File> for String {
    fn from(file: File) -> Self {
        file.to_string()
    }
}

impl From<&File> for String {
    fn from(file: &File) -> Self {
        file.to_string()
    }
}

impl From<String> for File {
    fn from(value: String) -> Self {
        File {
            path: PathBuf::from(value),
        }
    }
}

impl From<&String> for File {
    fn from(value: &String) -> Self {
        File {
            path: PathBuf::from(value),
        }
    }
}

impl From<&str> for File {
    fn from(value: &str) -> Self {
        File {
            path: PathBuf::from(value),
        }
    }
}

impl From<PathBuf> for File {
    fn from(value: PathBuf) -> Self {
        File { path: value }
    }
}

impl From<&PathBuf> for File {
    fn from(value: &PathBuf) -> Self {
        File {
            path: value.to_path_buf(),
        }
    }
}
