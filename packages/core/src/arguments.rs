use crate::directory::Directory;
use std::env;

pub struct Arguments;

impl Arguments {
    pub fn get_command_line_args() -> Vec<String> {
        env::args().collect()
    }
    pub fn get_formatted_args_log_line() -> String {
        let args = Self::get_command_line_args();
        format!(
            "Rstatic Server initiated with command line args: {:#?}",
            &args
        )
    }
    pub fn get_formatted_directory_arguments_line() -> String {
        let directory_arguments = Arguments::find_directory_arguments();
        format!(
            "Rstatic Server requested to serve static directories: {:#?}",
            Directory::get_directories_as_strings(&directory_arguments)
        )
    }
    pub fn find_directory_arguments() -> Vec<String> {
        let args = Arguments::get_command_line_args();
        args.into_iter()
            .filter(|arg| arg.contains("--dir="))
            .collect()
    }
    pub fn find_port_argument_or_get_default() -> usize {
        let default_port = 8080usize;
        let args = Arguments::get_command_line_args();
        let port_args: Vec<String> = args
            .into_iter()
            .filter(|arg| arg.contains("--port="))
            .collect();
        if port_args.len() == 0 {
            default_port
        } else {
            let first_argument = port_args[0].clone().replace("--port=", "");
            match first_argument.parse::<usize>() {
                Ok(port) => port,
                Err(_) => default_port,
            }
        }
    }
    pub fn find_host_argument_or_get_default() -> String {
        let default_host = String::from("127.0.0.1");
        let args = Arguments::get_command_line_args();
        let host_args: Vec<String> = args
            .clone()
            .into_iter()
            .filter(|arg| arg.contains("--host="))
            .collect();
        if host_args.len() == 0 {
            default_host
        } else {
            host_args[0].clone().replace("--host=", "")
        }
    }
}
