use application::common::interfaces::Os;
use std::{
    fs::{self, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::PathBuf,
};

pub struct FileSystem {}

impl Os for FileSystem {
    fn clear_folder(&self, path: String) -> Result<(), String> {
        fs::remove_dir_all(path).expect("issue to remove");
        Ok(())
    }

    fn move_file(&self, source_file: String, target_file: String) -> Result<(), String> {
        let target_file = PathBuf::from(target_file);
        let target_dir = target_file.parent().expect("issue to get dir").to_str().expect("issue to str");

        std::fs::create_dir_all(target_dir).expect("issue to create target directory");
        std::fs::copy(source_file, target_file.clone()).expect("issue to copy");

        Ok(())
    }

    fn read_file(&self, path: String) -> Result<String, String> {
        let content = fs::read_to_string(path).expect("issue to read file");
        Ok(content)
    }

    fn write_file(&self, path: String, content: String) -> Result<(), String> {
        let path = PathBuf::from(path);
        let dir = path.parent().unwrap();

        std::fs::create_dir_all(dir).expect("issue to create target directory");
        std::fs::write(path, content).expect("issue to write");

        Ok(())
    }

    fn read_file_buffered(&self, path: String) -> Result<Vec<String>, String> {
        let path = PathBuf::from(path);

        let file = std::fs::File::open(path).unwrap();
        let reader = BufReader::new(file);
        let lines = reader.lines().map(|l| l.unwrap()).collect();

        Ok(lines)
    }

    fn write_line_to_file(&self, path: &str, content: String) -> Result<(), String> {
        let path = PathBuf::from(path);

        let mut file = OpenOptions::new().write(true).append(true).open(path).unwrap();
        writeln!(file, "{}", content).expect("issue to write line");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_read_file_buffered() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("my-temporary-note.txt");
        let mut file = File::create(file_path.to_str().unwrap().to_string().clone()).unwrap();
        writeln!(file, "file1\nfile2\nfile3").unwrap();

        let file_system = FileSystem {};
        let lines = file_system.read_file_buffered(file_path.to_str().unwrap().to_string()).unwrap();

        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "file1");
        assert_eq!(lines[1], "file2");
        assert_eq!(lines[2], "file3");
    }
}
