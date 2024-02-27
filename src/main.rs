pub mod arguments;
pub mod cache;
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

// use notebook_api::ThreadPool;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};

use arguments::Arguments;
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
        let directories_as_path_options =
            Directory::get_clean_directory_paths(&directory_arguments);
        let mut directories_as_paths: Vec<String> = vec![];
        directories_as_path_options
            .into_iter()
            .for_each(|dir_option| match dir_option {
                Some(dir) => {
                    directories_as_paths.push(dir);
                }
                None => (),
            });
        handle_connection(
            stream,
            StaticDirectoryManager {
                directories: directories_as_paths,
                backup_file: DefaultFile::get_default_file_or_default(&args),
            },
        );

        // let pool = ThreadPool::new(2);

        // pool.execute(|| {
        //     let args: Vec<String> = env::args().collect();
        //     let directory_arguments = Arguments::find_directory_arguments();
        //     let directories_as_path_options =
        //         Directory::get_clean_directory_paths(&directory_arguments);
        //     let mut directories_as_paths: Vec<String> = vec![];
        //     directories_as_path_options
        //         .into_iter()
        //         .for_each(|dir_option| match dir_option {
        //             Some(dir) => {
        //                 directories_as_paths.push(dir);
        //             }
        //             None => (),
        //         });
        //     handle_connection(
        //         stream,
        //         StaticDirectoryManager {
        //             directories: directories_as_paths,
        //             backup_file: DefaultFile::get_default_file_or_default(&args),
        //         },
        //     );
        // });
    }
}

fn handle_connection(
    mut stream: TcpStream,
    static_directory_manager_instance: StaticDirectoryManager,
) {
    let buf_reader = BufReader::new(&mut stream);
    let request_result = Request::new(buf_reader);

    match request_result {
        Ok(request) => {
            Logger::info(&format!("Headers are {:#?}", request.headers()));
            let path = request.path();
            let requested_file_result = static_directory_manager_instance
                .search_for_file_path_in_approved_directories(path);
            let file = match requested_file_result {
                Ok(file_contents) => file_contents,
                Err(e) => fs::read_to_string(&static_directory_manager_instance.backup_file)
                    .unwrap_or_else(|_| format!("{{ \"error\": \"{:#?}\" }}", e)),
            };

            let response_headers = Headers::new(vec![(
                String::from("Content-Length"),
                file.len().to_string(),
            )]);

            let response = Response::new(
                String::from("HTTP/1.1"),
                200,
                String::from("OK"),
                file,
                response_headers.map,
            );

            stream
                .write_all(response.build_as_string().as_bytes())
                .unwrap();
        }
        Err(e) => {
            let error_body = format!("{{ \"error\": \"{}\" }}", e);
            let response_headers = Headers::new(vec![(
                String::from("Content-Length"),
                error_body.len().to_string(),
            )]);
            let response = Response::new(
                String::from("HTTP/1.1"),
                500,
                String::from("SERVER ERROR"),
                error_body,
                response_headers.map,
            );

            stream
                .write_all(response.build_as_string().as_bytes())
                .unwrap();
        }
    }
}
