use crate::filelike::FileLike;
use crate::headers::Headers;
use crate::request::Request;
use crate::response::Response;
use crate::static_directory_manager::StaticDirectoryManager;

use std::{
    io::{prelude::*, BufReader},
    net::TcpStream,
};

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
                            .search_for_file_path_in_approved_directories(
                                static_directory_manager_instance.backup_file.as_str(),
                            );

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

                let null_accept_encoding_header = String::new();
                let derefd_accept_encoding_header = request
                    .headers()
                    .get_header_by_key("Accept-Encoding")
                    .unwrap_or_else(|| &null_accept_encoding_header);

                let compressed = derefd_accept_encoding_header.contains("gzip");
                let mut headers = Headers::new(vec![
                    (
                        String::from("Content-Type"),
                        String::from(Headers::format_content_type_header_based_on_request_path(
                            &path,
                        )),
                    ),
                    (
                        String::from("Cache-Control"),
                        String::from("private, max-age=60, must-revalidate"), // Get from command line arg
                    ),
                ]);

                if compressed {
                    headers
                        .map
                        .insert(String::from("Content-Encoding"), String::from("gzip"));
                    headers
                        .map
                        .insert(String::from("Connection"), String::from("close"));
                } else {
                    headers
                        .map
                        .insert(String::from("Content-Length"), file.len().to_string());
                }

                let response = Response::new(
                    String::from("HTTP/1.1"),
                    200,
                    String::from("OK"),
                    headers.map,
                    file,
                    compressed,
                );

                response.respond(&mut stream);
            }
            Err(e) => Self::handle_request_with_error(e, &mut stream),
        }
    }

    pub fn handle_request_with_error(e: String, stream: &mut TcpStream) {
        let error_body = format!("{{ \"error\": \"{}\" }}", e);

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

        stream
            .write_all(response.build_as_string().as_bytes())
            .unwrap();
    }
}
