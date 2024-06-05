use crate::http_types::*;
use std::collections::HashMap;

pub struct Response {
    version: String,
    status: StatusCode,
    headers: Option<Headers>,
    body: Option<Vec<u8>>,
}

impl Response {
    pub fn new(
        version: String,
        status: StatusCode,
        headers: Option<Headers>,
        body: Option<Vec<u8>>,
    ) -> Self {
        Response {
            version,
            headers,
            status,
            body,
        }
    }
}

impl Into<Vec<u8>> for Response {
    fn into(self) -> Vec<u8> {
        let status_line = format!("HTTP/{} {}", self.version, String::from(self.status))
            .as_bytes()
            .to_owned();
        let headers = self
            .headers
            .unwrap_or(Headers(HashMap::new()))
            .0
            .iter()
            .map(|(key, value)| format!("{key}: {value}\r\n"))
            .collect::<String>()
            .as_bytes()
            .to_owned();
        let body = self.body.unwrap_or(vec![]);
        let crlf = "\r\n".as_bytes().to_owned();
        vec![status_line, crlf.clone(), headers, crlf.clone(), body]
            .concat()
            .to_owned()
    }
}

impl Into<Response> for &str {
    fn into(self) -> Response {
        let mut lines = self.lines();

        // Parse the status line
        let status_line = lines.next().unwrap_or_default();
        let mut status_line_parts = status_line.split_whitespace();

        let version = status_line_parts.next().unwrap_or_default().to_string();
        let status_code = status_line_parts.next().unwrap_or_default();

        let status = match status_code {
            "200" => StatusCode::Ok,
            "404" => StatusCode::NotFound,
            _ => StatusCode::Ok,
        };

        // Parse headers
        let mut headers_map = HashMap::new();
        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }
            let mut parts = line.splitn(2, ':');
            let key = parts.next().unwrap_or_default().trim();
            let value = parts.next().unwrap_or_default().trim();
            headers_map.insert(key.to_string(), value.to_string());
        }
        let headers = if headers_map.is_empty() {
            None
        } else {
            Some(Headers(headers_map))
        };

        // Parse body
        let body: Option<Vec<u8>> = {
            let remaining_lines: Vec<&str> = lines.collect();
            if remaining_lines.is_empty() {
                None
            } else {
                Some(remaining_lines.join("\n").as_bytes().to_owned())
            }
        };

        Response {
            version,
            status,
            headers,
            body,
        }
    }
}
