pub mod default_file;
pub mod directory;
pub mod hostname;
pub mod logger;
pub mod port;
pub mod response;
pub mod static_directory_manager;

use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};

use default_file::DefaultFile;
use directory::Directory;
use hostname::Hostname;
use logger::Logger;
use port::Port;
use response::Response;
use static_directory_manager::StaticDirectoryManager;

const VERSION: &str = "1.0.0";

fn main() {
    let static_server_started_log = format!("Starting Rstatic Server, version {:?}", VERSION);
    Logger::info(&static_server_started_log);

    let args: Vec<String> = env::args().collect();
    let arguments_log = format!(
        "Rstatic Server initiated with command line args: {:#?}",
        &args
    );
    Logger::info(&arguments_log);

    let directory_arguments = Directory::find_directory_arguments(&args);
    let directory_arguments_log = format!(
        "Rstatic Server requested to serve static directories: {:#?}",
        &directory_arguments
    );
    Logger::info(&directory_arguments_log);

    let has_static_directories = Directory::ensure_directory_integrity(&directory_arguments);
    let has_static_directories_log = format!("All directories exist: {}", has_static_directories);
    if has_static_directories {
        Logger::info(&has_static_directories_log);
    } else {
        Logger::error(&has_static_directories_log);
    }

    if !has_static_directories {
        // Handle closing the program here with a message.
    }

    let directories_as_path_options = Directory::get_clean_directory_paths(&directory_arguments);
    let mut directories_as_paths: Vec<String> = vec![];
    directories_as_path_options
        .into_iter()
        .for_each(|dir_option| match dir_option {
            Some(dir) => {
                directories_as_paths.push(dir);
            }
            None => (),
        });

    let static_dir_manager_instance = StaticDirectoryManager {
        directories: directories_as_paths,
        backup_file: DefaultFile::get_default_file_or_default(&args),
    };

    let port_argument = Port::find_port_argument_or_get_default(&args);
    let host_argument = Hostname::find_host_argument_or_get_default(&args);
    let host_and_port_string = format!("{host_argument}:{port_argument}");

    let listener = TcpListener::bind(host_and_port_string).unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, &static_dir_manager_instance);
    }
}

fn handle_connection(
    mut stream: TcpStream,
    static_directory_manager_instance: &StaticDirectoryManager,
) {
    // we create a new BufReader instance that wraps a mutable reference to the stream.
    // BufReader adds buffering by managing calls to the std::io::Read trait methods for us.
    let buf_reader = BufReader::new(&mut stream);
    // We create a variable named http_request to collect the lines of the request.
    // We indicate that we want to collect these lines in a vector
    // by adding the Vec<_> type annotation.
    let http_request: Vec<_> = buf_reader
        .lines() // BufReader implements the std::io::BufRead trait, which provides the lines method. The lines method returns an iterator of Result<String, std::io::Error> by splitting the stream of data whenever it sees a newline byte.
        .map(|result| result.unwrap()) // To get each String, we map and unwrap each Result. The Result might be an error if the data isn’t valid UTF-8 or if there was a problem reading from the stream.
        .take_while(|line| !line.is_empty()) // The browser signals the end of an HTTP request by sending two newline characters in a row, so to get one request from the stream, we take lines until we get a line that is the empty string.
        .collect();

    let request_line = &http_request[0];
    let request_line_split_on_whitespace: Vec<&str> = request_line[..].split(" ").collect();

    if request_line_split_on_whitespace.len() != 3 {
        // handle misformatted request line, with a 500 response.
    }

    let path = request_line_split_on_whitespace[1];

    let requested_file_result =
        static_directory_manager_instance.search_for_file_path_in_approved_directories(path);
    let file = match requested_file_result {
        Ok(file_contents) => file_contents,
        Err(_) => fs::read_to_string(&static_directory_manager_instance.backup_file).unwrap(),
    };

    let mut headers: HashMap<String, String> = HashMap::new();
    headers.insert(String::from("Content-Length"), file.len().to_string());

    let response = Response::new(
        String::from("HTTP/1.1"),
        200,
        String::from("OK"),
        file,
        headers,
    );

    // Once we’ve collected the lines into the vector,
    // we’re printing them out using pretty debug formatting
    // so we can take a look at the instructions the web browser is sending to our server.
    // println!("Request: {:#?}", http_request);

    // This is the first line in our response.
    // It contains the protocol, status code, and status text
    // let status_line = "HTTP/1.1 200 OK";
    // Request our index file, which exists at ./index.html
    // read_to_string returns a Result<String> so we need to unwrap it
    // in order to obtain its value (the String)
    // let contents = fs::read_to_string("index.html").unwrap();
    // We'll use the length of the content in headers below
    // let length = contents.len();
    // response follows the format
    // status_line CRLF
    // Headers CRLF
    // CRLF
    // Body
    // let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    // we call as_bytes on our response to convert the string data to bytes.
    // The write_all method on stream takes a &[u8]
    // and sends those bytes directly down the connection.
    // Because the write_all operation could fail,
    // we use unwrap on any error result as before.
    stream
        .write_all(response.build_as_string().as_bytes())
        .unwrap();
}
