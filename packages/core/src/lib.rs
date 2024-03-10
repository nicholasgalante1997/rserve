pub mod arguments;
pub mod cache;
pub mod connection;
pub mod default_file;
pub mod directory;
pub mod filelike;
pub mod gzip;
pub mod headers;
pub mod hostname;
pub mod logger;
pub mod port;
pub mod request;
pub mod response;
pub mod static_directory_manager;

use std::{env, error::Error, net::TcpListener, process};

use arguments::Arguments;
use connection::ConnectionHandler;
use default_file::DefaultFile;
use directory::Directory;
use logger::Logger;
use static_directory_manager::StaticDirectoryManager;

const VERSION: &str = "1.1.0";

pub fn run() {
    echo_rsrv_process_started();

    if !ensure_directories_exist() {
        Logger::error("Supplied a static directory that was not found. Exiting process.");
        process::exit(1);
    }

    let server = get_server().unwrap_or_else(|e| {
        Logger::error(&format!(
            "ExceptionThrown while setting up server.\n{:#?}",
            e
        ));
        process::exit(1);
    });

    listen(server);
}

pub fn echo_rsrv_process_started() {
    let static_server_started_log = format!("Starting rsrv, version {:?}", VERSION);
    Logger::info(&static_server_started_log);
}

pub fn log_arguments() {
    let arguments_log = Arguments::get_formatted_args_log_line();
    Logger::info(&arguments_log);
}

pub fn log_directory_arguments() {
    let directory_arguments_log = Arguments::get_formatted_directory_arguments_line();
    Logger::info(&directory_arguments_log);
}

pub fn ensure_directories_exist() -> bool {
    let directory_arguments = Arguments::find_directory_arguments();
    Directory::ensure_directory_integrity(&directory_arguments)
}

pub fn get_directories_as_paths() -> Vec<String> {
    let directory_arguments = Arguments::find_directory_arguments();
    let dirs_as_options = Directory::get_clean_directory_paths(&directory_arguments);
    let mut dirs_as_paths: Vec<String> = vec![];

    dirs_as_options
        .into_iter()
        .for_each(|dir_option| match dir_option {
            Some(dir) => {
                dirs_as_paths.push(dir);
            }
            None => (),
        });

    dirs_as_paths
}

pub fn get_server() -> Result<TcpListener, Box<dyn Error>> {
    let port_argument = Arguments::find_port_argument_or_get_default();
    let host_argument = Arguments::find_host_argument_or_get_default();
    let host_and_port_string = format!("{host_argument}:{port_argument}");

    let listener = TcpListener::bind(host_and_port_string)?;
    Ok(listener)
}

pub fn listen(server: TcpListener) {
    for stream in server.incoming() {
        match stream {
            Ok(stream) => {
                let args: Vec<String> = env::args().collect();
                let dirs_as_paths = get_directories_as_paths();

                ConnectionHandler::handle(
                    stream,
                    StaticDirectoryManager {
                        directories: dirs_as_paths,
                        backup_file: DefaultFile::get_default_file_or_default(&args),
                    },
                );
            }
            Err(e) => {
                Logger::error(&format!("Stream Corrupted: {:#?}", e));
            }
        }
    }
}
