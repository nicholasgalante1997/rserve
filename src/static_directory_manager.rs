use crate::logger::Logger;
use std::fs;

#[derive(Debug, Clone)]
pub struct StaticDirectoryManager {
    pub directories: Vec<String>,
    pub backup_file: String,
}

impl StaticDirectoryManager {
    pub fn has(&self, value: &str) -> bool {
        let mut has_ancestor_dir = false;
        for approved_directory in &self.directories {
            if value.contains(approved_directory.as_str()) {
                has_ancestor_dir = true;
            }
        }
        has_ancestor_dir
    }
}

impl StaticDirectoryManager {
    pub fn get_file(&self, absolute_path: &str) -> Result<String, ()> {
        if self.has(absolute_path) {
            if let Ok(file_contents) = fs::read_to_string(&absolute_path) {
                Ok(file_contents)
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
}

impl StaticDirectoryManager {
    pub fn search_for_file_path_in_approved_directories(&self, path: &str) -> Result<String, ()> {
        let directories_as_strings = self.directories.clone();
        for mut directory_string in directories_as_strings {
            directory_string.push_str(path);
            Logger::info(&format!("Path is {}", &directory_string));
            let file_op_result = self.get_file(&directory_string);
            if let Ok(file) = file_op_result {
                return Ok(file);
            }
        }
        Err(())
    }
}
