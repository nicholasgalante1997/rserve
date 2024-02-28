use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Headers {
    pub map: HashMap<String, String>,
}

impl Headers {
    pub fn new(headers_as_string_vec: Vec<(String, String)>) -> Self {
        let mut map: HashMap<String, String> = HashMap::new();
        for (key, value) in headers_as_string_vec {
            map.insert(key, value);
        }
        Headers { map }
    }
    pub fn convert_raw_headers(raw_headers: Vec<String>) -> Vec<(String, String)> {
        let mut kv_header_vec: Vec<(String, String)> = Vec::new();
        for raw_header in raw_headers {
            let split_headers: Vec<_> = raw_header.split(": ").collect();
            if split_headers.len() > 1 {
                let key = *(split_headers.get(0).expect("Proved split_headers len > 1"));
                let value = *(split_headers.get(1).expect("Proved split_headers len > 1"));
                kv_header_vec.push((String::from(key), String::from(value)));
            }
        }
        kv_header_vec
    }
    pub fn get_header_by_key(&self, key: &str) -> Option<&String> {
        let header = self.map.get(&String::from(key));
        match header {
            Some(header_value) => Some(header_value),
            None => None,
        }
    }
    pub fn format_content_type_header_based_on_request_path(path: &str) -> String {
        let path_split_on_delimiter: Vec<_> = path.split(".").collect();
        let last_index = path_split_on_delimiter.len() - 1;
        let file_ending = *(path_split_on_delimiter
            .get(last_index)
            .expect("Proved using valid index"));

        let mut known_content_types: HashMap<&str, &str> = HashMap::new();

        known_content_types.insert("htm", "text/html; charset=utf-8");
        known_content_types.insert("html", "text/html; charset=utf-8");
        known_content_types.insert("xhtml", "application/xhtml+xml");
        known_content_types.insert("xml", "application/xml");

        known_content_types.insert("css", "text/css");

        known_content_types.insert("js", "text/javascript");
        known_content_types.insert("mjs", "text/javscript");
        known_content_types.insert("cjs", "text/javascript");
        known_content_types.insert("json", "application/json");
        known_content_types.insert("jsonld", "application/ld+json");
        known_content_types.insert("csv", "text/csv");

        known_content_types.insert("php", "application/x-httpd-php");

        known_content_types.insert("gz", "application/gzip");
        known_content_types.insert("pdf", "application/pdf");
        known_content_types.insert("tar", "application/x-tar");
        known_content_types.insert("zip", "application/zip");

        known_content_types.insert("avif", "image/avif");
        known_content_types.insert("gif", "image/gif");
        known_content_types.insert("ico", "image/vnd.microsoft.icon");
        known_content_types.insert("jpg", "image/jpeg");
        known_content_types.insert("jpeg", "image/jpeg");
        known_content_types.insert("mp3", "video/mpeg");
        known_content_types.insert("mp4", "video/mp4");
        known_content_types.insert("mpeg", "videp/mpeg");
        known_content_types.insert("png", "image/png");
        known_content_types.insert("svg", "image/svg+xml");
        known_content_types.insert("webp", "image/webp");

        known_content_types.insert("wav", "audio/wav");
        known_content_types.insert("weba", "audio/webm");
        known_content_types.insert("webm", "audio/webm");

        known_content_types.insert("otf", "font/otf");
        known_content_types.insert("ttf", "font/ttf");
        known_content_types.insert("rtf", "application/rtf");
        known_content_types.insert("woff", "font/woff");
        known_content_types.insert("woff2", "font/woff2");

        known_content_types.insert("bin", "application/octet-stream");
        known_content_types.insert("txt", "text/plain");

        let content_type = known_content_types.get(file_ending);
        match content_type {
            Some(value) => String::from(*value),
            None => String::from("application/octet-stream"),
        }
    }
}

impl Headers {
    pub fn get_accept_encoding_header_if_exists(
        headers: &HashMap<String, String>,
    ) -> Option<String> {
        let header = headers.get(&String::from("Accept-Encoding"));
        match header {
            Some(header) => Some(header.clone()),
            _ => None,
        }
    }
}

impl Headers {
    pub fn get_accept_content_header_if_exists(
        headers: &HashMap<String, String>,
    ) -> Option<String> {
        let header = headers.get(&String::from("Accept"));
        match header {
            Some(header) => Some(header.clone()),
            _ => None,
        }
    }
}
