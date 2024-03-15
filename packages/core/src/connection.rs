use crate::headers::Headers;
use crate::request::Request;
use crate::response::Response;
use crate::static_directory_manager::StaticDirectoryManager;
use crate::{filelike::FileLike, logger::Logger};

use std::{
    io::{prelude::*, BufReader},
    net::TcpStream,
};

use serde_json;

pub struct ConnectionHandler;

impl ConnectionHandler {
    pub fn handle(
        mut stream: TcpStream,
        static_directory_manager_instance: StaticDirectoryManager,
    ) {
        let buf_reader = BufReader::new(&mut stream);
        let request_result = Request::new(buf_reader);
        match request_result {
            Ok(request) => {
                let path = request.path();

                let file_result = static_directory_manager_instance
                    .search_for_file_path_in_approved_directories(path);

                let file = match file_result {
                    Ok(file_contents) => file_contents,
                    Err(_) => {
                        let backup_file_result = static_directory_manager_instance
                            .search_for_file_path_in_approved_directories(&format!(
                                "/{}",
                                static_directory_manager_instance.backup_file.as_str()
                            ));

                        match backup_file_result {
                            Ok(file) => file,
                            Err(_) => {
                                Self::handle_request_with_error(
                                    String::from(
                                        "ConnectionHandler::SimpleException Failed to read requested file.",
                                    ),
                                    &mut stream,
                                );
                                return;
                            }
                        }
                    }
                };

                let accept_encoding_header =
                    match request.headers().get_header_by_key("Accept-Encoding") {
                        Some(header) => header.clone(),
                        None => String::new(),
                    };

                let compressed = accept_encoding_header.contains("gzip");

                let response = Response::new(
                    String::from("HTTP/1.1"),
                    200,
                    String::from("OK"),
                    Headers::construct_outgoing_headers(request, &file, compressed).map,
                    file,
                    compressed,
                );

                response.respond(&mut stream);
            }
            Err(e) => Self::handle_request_with_error(e, &mut stream),
        }
    }

    pub fn handle_request_with_error(e: String, stream: &mut TcpStream) {
        let connection_error = ConnectionError::new(e);
        let error_body = connection_error.get_error_as_json_string();

        let response_headers = Headers::new(vec![
            (String::from("Content-Length"), error_body.len().to_string()),
            (
                String::from("Content-Type"),
                String::from(Headers::format_content_type_header_based_on_request_path(
                    ".json",
                )),
            ),
        ]);

        let response = Response::new(
            String::from("HTTP/1.1"),
            500,
            String::from("SERVER ERROR"),
            response_headers.map,
            FileLike::TextFile(error_body),
            false,
        );

        let stream_write_result = stream.write_all(response.build_as_string().as_bytes());

        match stream_write_result {
            Ok(_) => (),
            Err(e) => {
                Logger::error(&format!("{:?}", e));
            }
        }
    }
}

pub struct ConnectionError {
    message: String,
}

impl ConnectionError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
    pub fn get_error_as_json_string<'a>(&self) -> String {
        let json = serde_json::json!({ "error": &self.message });
        let json_str = json.to_string();
        json_str
    }
}
