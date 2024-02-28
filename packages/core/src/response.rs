use std::collections::HashMap;
use std::io::prelude::*;
use std::net::TcpStream;

use crate::gzip::Gzip;
use crate::headers::Headers;

pub struct Response {
    protocol: String,
    status: u16,
    status_text: String,
    body: String,
    headers: HashMap<String, String>,
}

impl Response {
    pub fn new(
        protocol: String,
        status: u16,
        status_text: String,
        headers: HashMap<String, String>,
        body: String,
    ) -> Self {
        Self {
            body,
            headers,
            protocol,
            status,
            status_text,
        }
    }
}

impl Response {
    pub fn build_as_string(&self) -> String {
        let status_line = format!("{} {} {}", self.protocol, self.status, self.status_text);
        let headers_as_string = self.headers_as_string();
        format!("{status_line}\r\n{headers_as_string}\r\n\r\n{}", &self.body)
    }
}

impl Response {
    fn headers_as_string(&self) -> String {
        let mut headers_as_string = String::new();
        for (key, value) in &self.headers {
            headers_as_string.push_str(format!("{key}: {value}\r\n").as_str());
        }
        headers_as_string
    }
}

impl Response {
    pub fn respond_in_octet_stream(
        stream: &mut TcpStream,
        response: Response,
        path: &str,
    ) -> Result<(), String> {
        let compression_result = Gzip::compress(&response.body);
        match compression_result {
            Ok(compressed_data) => {
                let header_response = format!(
                    "{} {} {}\r\n\
                    Content-Type: {}\r\n\
                    Content-Encoding: gzip\r\n\
                    Content-Length: {}\r\n\
                    Connection: close\r\n\r\n",
                    response.protocol,
                    response.status,
                    response.status_text,
                    Headers::format_content_type_header_based_on_request_path(path),
                    compressed_data.len()
                );

                stream
                    .write_all(header_response.as_bytes())
                    .expect("Failed to send response header");

                stream
                    .write_all(&compressed_data)
                    .expect("Failed to send compressed data");

                stream.flush().expect("Failed to write stream.");

                Ok(())
            }
            Err(e) => Err(format!("Error: {:#?}", e)),
        }
    }
}
