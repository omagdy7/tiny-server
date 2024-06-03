use crate::http_types::*;
use std::collections::HashMap;

pub struct Response {
    version: String,
    status: StatusCode,
    headers: Option<Headers>,
    body: Option<String>,
}

impl Response {
    pub fn new(
        version: String,
        status: StatusCode,
        headers: Option<Headers>,
        body: Option<String>,
    ) -> Self {
        Response {
            version,
            headers,
            status,
            body,
        }
    }
}

impl Into<String> for Response {
    fn into(self) -> String {
        let status_line = format!("HTTP/{} {}", self.version, String::from(self.status));
        let headers = self
            .headers
            .unwrap_or(Headers(HashMap::new()))
            .0
            .iter()
            .map(|(key, value)| format!("{key}: {value}\r\n"))
            .collect::<String>();
        let body = self.body.unwrap_or("".to_string());
        format!("{status_line}\r\n{headers}\r\n{body}")
    }
}
