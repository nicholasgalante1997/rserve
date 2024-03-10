use std::collections::HashMap;
use std::io::prelude::*;
use std::net::TcpStream;

use crate::filelike::FileLike;
use crate::gzip::Gzip;
use crate::logger::Logger;

pub struct Response {
    compress: bool,
    protocol: String,
    status: u16,
    status_text: String,
    headers: HashMap<String, String>,
    body: FileLike,
}

impl Response {
    pub fn new(
        protocol: String,
        status: u16,
        status_text: String,
        headers: HashMap<String, String>,
        body: FileLike,
        compress: bool,
    ) -> Self {
        Self {
            compress,
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
    pub fn respond(&self, stream: &mut TcpStream) {
        match &self.body {
            FileLike::TextFile(_) => {
                if self.compress {
                    match Gzip::compress(&self.body) {
                        Ok(compressed_file) => {
                            let response_header = format!(
                                "{} {} {}\r\nContent-Length: {}\r\n{}\r\n",
                                &self.protocol,
                                &self.status,
                                &self.status_text,
                                &compressed_file.len(),
                                self.headers_as_string(),
                            );

                            let header_stream_write_result =
                                stream.write_all(response_header.as_bytes());

                            match header_stream_write_result {
                                Ok(_) => (),
                                Err(e) => {
                                    Logger::error(&format!("{:?}", e));
                                    return;
                                }
                            }

                            let body_stream_write_result = stream.write_all(&compressed_file);

                            match body_stream_write_result {
                                Ok(_) => (),
                                Err(e) => {
                                    Logger::error(&format!("{:?}", e));
                                    return;
                                }
                            }

                            let flush_stream_result = stream.flush();

                            match flush_stream_result {
                                Ok(_) => (),
                                Err(e) => {
                                    Logger::error(&format!("{:?}", e));
                                }
                            }
                        }
                        Err(e) => {
                            // Fallback to trying to respond with a plain text file
                            Logger::error(e.to_string().as_str());
                            match stream.write_all(&self.build_as_string().as_bytes()) {
                                Ok(_) => (),
                                Err(e) => {
                                    Logger::error(e.to_string().as_str());
                                }
                            }
                        }
                    }
                } else {
                    match stream.write_all(&self.build_as_string().as_bytes()) {
                        Ok(_) => (),
                        Err(e) => {
                            Logger::error(&e.to_string());
                        }
                    }
                }
            }
            FileLike::ImageFile(image_file) => {
                if self.compress {
                    match Gzip::compress(&self.body) {
                        Ok(compressed_image_file) => {
                            let response_header = format!(
                                "{} {} {}\r\nContent-Length: {}\r\n{}\r\n",
                                &self.protocol,
                                &self.status,
                                &self.status_text,
                                &compressed_image_file.len(),
                                self.headers_as_string(),
                            );

                            let header_stream_write_result =
                                stream.write_all(response_header.as_bytes());

                            match header_stream_write_result {
                                Ok(_) => (),
                                Err(e) => {
                                    Logger::error(&format!("{:?}", e));
                                    return;
                                }
                            }

                            let body_stream_write_result = stream.write_all(&compressed_image_file);

                            match body_stream_write_result {
                                Ok(_) => (),
                                Err(e) => {
                                    Logger::error(&format!("{:?}", e));
                                }
                            }

                            let stream_flush_result = stream.flush();

                            match stream_flush_result {
                                Ok(_) => (),
                                Err(e) => {
                                    Logger::error(&format!("{:?}", e));
                                }
                            }
                        }
                        Err(e) => {
                            Logger::error(&format!(
                                "Error thrown during file compression - {:?}",
                                e
                            ));
                            // Fallback to standard image file serving
                            let response_header = format!(
                                "{} {} {}\r\n{}\r\n",
                                &self.protocol,
                                &self.status,
                                &self.status_text,
                                &self.headers_as_string()
                            );

                            let header_stream_write_result =
                                stream.write_all(response_header.as_bytes());

                            match header_stream_write_result {
                                Ok(_) => (),
                                Err(e) => {
                                    Logger::error(&format!("{:?}", e));
                                }
                            };

                            let body_stream_write_result = stream.write_all(&image_file);

                            match body_stream_write_result {
                                Ok(_) => (),
                                Err(e) => {
                                    Logger::error(&format!("{:?}", e));
                                }
                            };

                            match stream.flush() {
                                Ok(_) => (),
                                Err(e) => {
                                    Logger::error(e.to_string().as_str());
                                }
                            }
                        }
                    }
                } else {
                    let response_header = format!(
                        "{} {} {}\r\n{}\r\n",
                        &self.protocol,
                        &self.status,
                        &self.status_text,
                        &self.headers_as_string()
                    );

                    let header_stream_write_result = stream.write_all(response_header.as_bytes());

                    match header_stream_write_result {
                        Ok(_) => (),
                        Err(e) => {
                            Logger::error(&format!("{:?}", e));
                            return;
                        }
                    };

                    let body_stream_write_result = stream.write_all(&image_file);

                    match body_stream_write_result {
                        Ok(_) => (),
                        Err(e) => {
                            Logger::error(&format!("{:?}", e));
                        }
                    }

                    match stream.flush() {
                        Ok(_) => (),
                        Err(e) => {
                            Logger::error(e.to_string().as_str());
                        }
                    }
                }
            }
            FileLike::ProxyFile(file) => {
                if self.compress {
                    match Gzip::compress(&self.body) {
                        Ok(compressed_file) => {
                            let response_header = format!(
                                "{} {} {}\r\nContent-Length: {}\r\n{}\r\n",
                                &self.protocol,
                                &self.status,
                                &self.status_text,
                                &compressed_file.len(),
                                self.headers_as_string(),
                            );

                            let header_stream_write_result =
                                stream.write_all(response_header.as_bytes());

                            match header_stream_write_result {
                                Ok(_) => (),
                                Err(e) => {
                                    Logger::error(e.to_string().as_str());
                                }
                            }

                            let body_stream_write_result = stream.write_all(&compressed_file);

                            match body_stream_write_result {
                                Ok(_) => (),
                                Err(e) => {
                                    Logger::error(e.to_string().as_str());
                                }
                            }

                            match stream.flush() {
                                Ok(_) => (),
                                Err(e) => {
                                    Logger::error(e.to_string().as_str());
                                }
                            }
                        }
                        Err(e) => {
                            Logger::error(&format!(
                                "Error thrown during file compression - {:?}",
                                e
                            ));
                            // Fallback to standard image file serving
                            let response_header = format!(
                                "{} {} {}\r\n{}\r\n",
                                &self.protocol,
                                &self.status,
                                &self.status_text,
                                &self.headers_as_string()
                            );

                            let header_stream_write_result =
                                stream.write_all(response_header.as_bytes());

                            match header_stream_write_result {
                                Ok(_) => (),
                                Err(e) => {
                                    Logger::error(e.to_string().as_str());
                                }
                            }

                            let body_stream_write_result = stream.write_all(&file);

                            match body_stream_write_result {
                                Ok(_) => (),
                                Err(e) => {
                                    Logger::error(e.to_string().as_str());
                                }
                            }

                            let stream_flush_result = stream.flush();

                            match stream_flush_result {
                                Ok(_) => (),
                                Err(e) => {
                                    Logger::error(e.to_string().as_str());
                                }
                            }
                        }
                    }
                } else {
                    let response_header = format!(
                        "{} {} {}\r\n{}\r\n",
                        &self.protocol,
                        &self.status,
                        &self.status_text,
                        &self.headers_as_string()
                    );

                    let header_stream_write_result = stream.write_all(response_header.as_bytes());

                    match header_stream_write_result {
                        Ok(_) => (),
                        Err(e) => {
                            Logger::error(e.to_string().as_str());
                        }
                    }

                    let body_stream_write_result = stream.write_all(&file);

                    match body_stream_write_result {
                        Ok(_) => (),
                        Err(e) => {
                            Logger::error(e.to_string().as_str());
                        }
                    }

                    match stream.flush() {
                        Ok(_) => (),
                        Err(e) => {
                            Logger::error(e.to_string().as_str());
                        }
                    }
                }
            }
        }
    }
}
