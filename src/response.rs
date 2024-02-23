use std::collections::HashMap;

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
        body: String,
        headers: HashMap<String, String>,
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
            headers_as_string.push_str(format!("{key}: {value}").as_str());
        }
        headers_as_string
    }
}
