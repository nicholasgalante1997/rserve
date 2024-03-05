use crate::filelike::FileLike;
use crate::logger::Logger;
use core::fmt::{Debug, Display};
use std::error::Error;
use std::fmt::write;

pub struct UnknownFileError {
    path: String,
}

impl Debug for UnknownFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write(
            f,
            format_args!(
                "RSRV::UnknownFileError - Unable to read file @ path: {}",
                self.path
            ),
        )
    }
}

impl Display for UnknownFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write(
            f,
            format_args!(
                "RSRV::UnknownFileError - Unable to read file @ path: {}",
                self.path
            ),
        )
    }
}

impl Error for UnknownFileError {}

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
    pub fn get_file(&self, absolute_path: &str) -> Result<FileLike, Box<dyn Error>> {
        if self.has(absolute_path) {
            FileLike::get_filelike(absolute_path)
        } else {
            Err(Box::new(UnknownFileError {
                path: String::from(absolute_path),
            }))
        }
    }
}

impl StaticDirectoryManager {
    pub fn search_for_file_path_in_approved_directories(&self, path: &str) -> Result<FileLike, ()> {
        let directories_as_strings = self.directories.clone();
        for mut directory_string in directories_as_strings {
            directory_string.push_str(path);
            let file_op_result = self.get_file(&directory_string);
            match file_op_result {
                Ok(file) => {
                    Logger::info(&format!("Found path! File exists @ {}", &directory_string));
                    return Ok(file);
                }
                Err(e) => {
                    Logger::error(e.to_string().as_str());
                }
            }
        }

        Logger::warn("Unable to find requested file in known static directories.");
        Err(())
    }
}
