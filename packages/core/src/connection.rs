use crate::{Headers, Request, Response, StaticDirectoryManager};
use std::{
    collections::HashMap,
    fs,
    io::{prelude::*, BufReader},
    net::TcpStream,
};

pub struct ConnectionHandler;

impl ConnectionHandler {
    pub fn handle(
        mut stream: TcpStream,
        static_directory_manager_instance: StaticDirectoryManager,
    ) {
        // Convert the stream to a Buffer
        let buf_reader = BufReader::new(&mut stream);

        // Pass the Buffer to the Request::new constructor
        // To parse the Buffer into a Result<Request, String>
        let request_result = Request::new(buf_reader);

        // Handle each arm of request_result
        match request_result {
            // We have constructed a valid Request object
            Ok(request) => {
                let requests_gzipped_file: bool;
                let accept_encoding_request_header =
                    request.headers().get_header_by_key("Accept-Encoding");

                match accept_encoding_request_header {
                    Some(header_value) => {
                        if header_value.contains("gzip") {
                            requests_gzipped_file = true;
                        } else {
                            requests_gzipped_file = false;
                        }
                    }
                    None => {
                        requests_gzipped_file = false;
                    }
                }

                // Request the path field off of `request`
                let path = request.path();

                if requests_gzipped_file {
                    Self::handle_gzipped_file_request(
                        path,
                        static_directory_manager_instance,
                        &mut stream,
                    );
                } else {
                    Self::handle_simple_static_file_request(
                        path,
                        static_directory_manager_instance,
                        &mut stream,
                    );
                }
            }

            // We have failed to construct a Request object from the Buffer
            Err(e) => Self::handle_request_with_error(e, &mut stream),
        }
    }

    pub fn handle_simple_static_file_request(
        path: &String,
        static_directory_manager_instance: StaticDirectoryManager,
        stream: &mut TcpStream,
    ) {
        // Check that the path exists in our static directories that we're managing
        let requested_file_result =
            static_directory_manager_instance.search_for_file_path_in_approved_directories(path);

        // Try and read the Result<String, ()> that was requested from `path`
        let mut failed_to_read_file = false;
        let file = match requested_file_result {
            Ok(file_contents) => file_contents,
            Err(e) => {
                failed_to_read_file = true;
                fs::read_to_string(&static_directory_manager_instance.backup_file)
                    .unwrap_or_else(|_| format!("{{ \"error\": \"{:#?}\" }}", e))
            }
        };

        if failed_to_read_file {
            Self::handle_request_with_error(
                String::from("ConnectionHandler::SimpleException Failed to read requested file."),
                stream,
            );
            return;
        }

        let response_headers = Headers::new(vec![
            (String::from("Content-Length"), file.len().to_string()),
            (
                String::from("Content-Type"),
                String::from(Headers::format_content_type_header_based_on_request_path(
                    &path,
                )),
            ),
            (
                String::from("Cache-Control"),
                String::from("private, max-age=60, must-revalidate"),
            ),
        ]);

        let response = Response::new(
            String::from("HTTP/1.1"),
            200,
            String::from("OK"),
            response_headers.map,
            file,
        );

        stream
            .write_all(response.build_as_string().as_bytes())
            .unwrap();
    }

    pub fn handle_gzipped_file_request(
        path: &String,
        static_directory_manager_instance: StaticDirectoryManager,
        stream: &mut TcpStream,
    ) {
        // Check that the path exists in our static directories that we're managing
        let requested_file_result =
            static_directory_manager_instance.search_for_file_path_in_approved_directories(path);

        // Try and read the Result<String, ()> that was requested from `path`
        let mut failed_to_read_file = false;
        let file = match requested_file_result {
            Ok(file_contents) => file_contents,
            Err(e) => {
                failed_to_read_file = true;
                fs::read_to_string(&static_directory_manager_instance.backup_file)
                    .unwrap_or_else(|_| format!("{{ \"error\": \"{:#?}\" }}", e))
            }
        };

        if failed_to_read_file {
            Self::handle_request_with_error(
                String::from("ConnectionHandler::SimpleException Failed to read requested file."),
                stream,
            );
            return;
        }

        let octet_response_result = Response::respond_in_octet_stream(
            stream,
            Response::new(
                String::from("HTTP/1.1"),
                200,
                String::from("OK"),
                HashMap::new(),
                file,
            ),
            path,
        );

        match octet_response_result {
            Ok(_) => (),
            Err(e) => Self::handle_request_with_error(e, stream),
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
            error_body,
        );

        stream
            .write_all(response.build_as_string().as_bytes())
            .unwrap();
    }
}
