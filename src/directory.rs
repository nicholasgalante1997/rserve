use std::env;
use std::fs;

pub struct Directory;

impl Directory {
    pub fn find_directory_arguments(args: &Vec<String>) -> Vec<String> {
        args.clone()
            .into_iter()
            .filter(|arg| arg.contains("--dir="))
            .collect()
    }
}

impl Directory {
    pub fn get_directories_as_strings(args: &Vec<String>) -> Vec<String> {
        args.clone()
            .into_iter()
            .map(|arg| arg.replace("--dir=", ""))
            .collect()
    }
}

impl Directory {
    pub fn get_clean_directory_paths(args: &Vec<String>) -> Vec<Option<String>> {
        args.clone()
            .into_iter()
            .map(|arg| arg.replace("--dir=", ""))
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
        let mut operation_status = true;
        if let Ok(current_dir) = env::current_dir() {
            if let Some(dir_str) = current_dir.to_str() {
                directories.into_iter().for_each(|directory| {
                    let mut full_directory_path = dir_str.to_string();
                    full_directory_path.push_str("/");
                    full_directory_path
                        .push_str(&Directory::convert_directory_flag_into_string(directory));
                    if let Ok(metadata) = fs::metadata(&full_directory_path) {
                        if metadata.is_dir() {
                            println!("The directory at {} exists!", &full_directory_path);
                        } else {
                            println!(
                                "The path at {} exists, but it is not a directory.",
                                &full_directory_path
                            );
                        }
                    } else {
                        println!("The directory at {} does not exist.", &full_directory_path);
                    }
                })
            } else {
                println!("Failed to convert the current working directory to a string.");
                operation_status = false;
            }
        } else {
            println!("Failed to retrieve the current working directory.");
            operation_status = false;
        }
        operation_status
    }
}

impl Directory {
    pub fn convert_directory_flag_into_string(directory_flag: &String) -> String {
        let directory_flag_clone = directory_flag.clone();
        let long_directory_flag_prefix = "--dir=";
        let directory = directory_flag_clone.replace(long_directory_flag_prefix, "");
        directory
    }
}
