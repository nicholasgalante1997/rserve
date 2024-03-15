use std::env;
use std::fs;

use crate::logger::Logger;

/// # Directory
///
/// Functional Struct
///
/// The Directory Struct has no data attributes.
///
/// It implements several associated functions
/// that can be used to facilitate working with the local fs.
///
///
pub struct Directory;

impl Directory {
    pub fn get_absolute_paths_from_dir_args(args: &Vec<String>) -> Vec<Option<String>> {
        args.clone()
            .into_iter()
            .map(|dir| {
                if let Ok(current_dir) = env::current_dir() {
                    if let Some(current_dir_as_string) = current_dir.to_str() {
                        Some(format!("{current_dir_as_string}/{dir}"))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect()
    }
}

impl Directory {
    pub fn ensure_directory_integrity(directories: &Vec<String>) -> bool {
        /* Set up a stateful variable to be returned */
        let mut operation_status = false;
        /* If we can read the cwd from env */
        if let Ok(current_dir) = env::current_dir() {
            /* If we succeed in converting the path buffer to string */
            if let Some(dir_str) = current_dir.to_str() {
                /* Iterate through the provided directories */
                directories.into_iter().for_each(|directory| {
                    /* Coerce a &str to a mutable String */
                    let mut full_directory_path = dir_str.to_string();
                    /* Append an fs directory marker */
                    full_directory_path.push_str("/");
                    /* Append the string that represents the provided directory argument */
                    full_directory_path.push_str(directory);
                    /* Attempt to load metadata on the full path */
                    if let Ok(metadata) = fs::metadata(&full_directory_path) {
                        /* The provided directory exists at the constructed path */
                        if metadata.is_dir() {
                            operation_status = true;
                        } else {
                            /* The provided path is not a directory but exists in fs */
                            Logger::warn(&format!(
                                "The path at {} exists, but it is not a directory.",
                                &full_directory_path
                            ));
                            /* Serve it regardless */
                            operation_status = true;
                        }
                    } else {
                        /* The constructed path does not exist */
                        Logger::error(&format!(
                            "The directory at {} does not exist.",
                            &full_directory_path
                        ));
                        operation_status = false;
                    }
                })
            } else {
                /* We cannot determine if the path is a valid dir */
                Logger::error(&format!(
                    "Failed to convert the current working directory to a string."
                ));
                operation_status = false;
            }
        } else {
            /* We cannot determine if the path is a valid dir */
            Logger::error(&format!(
                "Failed to retrieve the current working directory."
            ));
            operation_status = false;
        }

        /* Move the result of the operation into the calling scope */
        operation_status
    }
}
