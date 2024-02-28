pub mod arguments;
pub mod cache;
pub mod connection;
pub mod default_file;
pub mod directory;
pub mod gzip;
pub mod headers;
pub mod hostname;
pub mod logger;
pub mod port;
pub mod request;
pub mod response;
pub mod static_directory_manager;

use std::{env, net::TcpListener};

use arguments::Arguments;
use connection::ConnectionHandler;
use default_file::DefaultFile;
use directory::Directory;
use headers::Headers;
use logger::Logger;
use request::Request;
use response::Response;
use static_directory_manager::StaticDirectoryManager;

const VERSION: &str = "1.0.0";

fn main() {
    let static_server_started_log = format!("Starting Rserve, version {:?}", VERSION);
    Logger::info(&static_server_started_log);

    let arguments_log = Arguments::get_formatted_args_log_line();
    Logger::info(&arguments_log);

    let directory_arguments = Arguments::find_directory_arguments();
    let directory_arguments_log = Arguments::get_formatted_directory_arguments_line();
    Logger::info(&directory_arguments_log);

    let has_static_directories = Directory::ensure_directory_integrity(&directory_arguments);
    if has_static_directories {
        Logger::info("Directory Integrity Update: All supplied directories exist!");
    } else {
        Logger::error("Directory Integrity Update: Failed integrity check. Closing operation.");
    }

    if !has_static_directories {
        Logger::error("Rserve closed.");
        return;
    }

    let port_argument = Arguments::find_port_argument_or_get_default();
    let host_argument = Arguments::find_host_argument_or_get_default();
    let host_and_port_string = format!("{host_argument}:{port_argument}");

    let listener = TcpListener::bind(host_and_port_string).unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let args: Vec<String> = env::args().collect();

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

        ConnectionHandler::handle(
            stream,
            StaticDirectoryManager {
                directories: dirs_as_paths,
                backup_file: DefaultFile::get_default_file_or_default(&args),
            },
        );
    }
}
