use std::{
    io::{prelude::*, BufReader},
    net::TcpStream,
};

use crate::headers::Headers;
use crate::logger::Logger;

#[derive(Debug, Clone)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    OPTIONS,
}

#[derive(Debug, Clone)]
pub struct Request {
    path: String,
    protocol: String,
    headers: Headers,
    method: HttpMethod,
}

impl Request {
    pub fn new(buffer: BufReader<&mut TcpStream>) -> Result<Self, String> {
        let http_request: Vec<_> = buffer
            .lines() // BufReader implements the std::io::BufRead trait, which provides the lines method. The lines method returns an iterator of Result<String, std::io::Error> by splitting the stream of data whenever it sees a newline byte.
            .map(|result| match result {
                Ok(line) => line,
                Err(err) => {
                    Logger::warn(&format!(
                        "Request::new() Exception: Buffer yielded invalid byte line.\nError: {:#?}",
                        err
                    ));
                    String::new()
                }
            }) // To get each String, we map and unwrap each Result. The Result might be an error if the data isn’t valid UTF-8 or if there was a problem reading from the stream.
            .take_while(|line| !line.is_empty()) // The browser signals the end of an HTTP request by sending two newline characters in a row, so to get one request from the stream, we take lines until we get a line that is the empty string.
            .collect();

        if http_request.len() == 0 {
            return Err(String::from(
                "Request::new() Exception: HttpRequest String is corrupted.",
            ));
        }

        let request_line = &http_request[0];
        let request_line_split_on_whitespace: Vec<&str> = request_line[..].split(" ").collect();

        if request_line_split_on_whitespace.len() != 3 {
            // handle misformatted request line, with a 500 response.
            return Err(String::from(
                "Request::new() Exception: HttpRequest String is corrupted. Request line is misformatted.",
            ));
        }

        let method = *(request_line_split_on_whitespace
            .get(0)
            .expect("Proved request_line_split_on_whitespace.len() == 3"));
        let path = *(request_line_split_on_whitespace
            .get(1)
            .expect("Proved request_line_split_on_whitespace.len() == 3"));
        let protocol = *(request_line_split_on_whitespace
            .get(2)
            .expect("Proved request_line_split_on_whitespace.len() == 3"));

        let raw_headers: Vec<_> = http_request.clone()[1..]
            .iter()
            .map(|item| item.clone())
            .collect();

        Ok(Request {
            path: String::from(path),
            protocol: String::from(protocol),
            method: Request::get_enumerated_method_from_string(method),
            headers: Headers::new(Headers::convert_raw_headers(raw_headers)),
        })
    }

    pub fn get_enumerated_method_from_string(method_as_str: &str) -> HttpMethod {
        if method_as_str.to_lowercase() == "get" {
            return HttpMethod::GET;
        }

        if method_as_str.to_lowercase() == "post" {
            return HttpMethod::POST;
        }

        if method_as_str.to_lowercase() == "put" {
            return HttpMethod::PUT;
        }

        if method_as_str.to_lowercase() == "delete" {
            return HttpMethod::DELETE;
        }

        if method_as_str.to_lowercase() == "options" {
            return HttpMethod::OPTIONS;
        }

        HttpMethod::GET
    }

    pub fn method(&self) -> &HttpMethod {
        &self.method
    }

    pub fn path(&self) -> &String {
        &self.path
    }

    pub fn protocol(&self) -> &String {
        &self.protocol
    }

    pub fn headers(&self) -> &Headers {
        &self.headers
    }
}
