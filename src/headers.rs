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
