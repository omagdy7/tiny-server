use std::collections::HashMap;

use crate::http_types::*;

#[derive(Debug, Clone)]
pub struct Request {
    pub method: Method,
    pub headers: Option<Headers>,
    body: Option<String>,
}

impl Request {
    fn new(method: Method, headers: Headers, body: String) -> Self {
        let headers = if headers.0.len() == 0 {
            None
        } else {
            Some(headers)
        };
        let body = if body.is_empty() { None } else { Some(body) };
        Request {
            method,
            headers,
            body,
        }
    }

    pub fn get_tag(&self, key: &str) -> Option<&String> {
        self.headers.as_ref().unwrap().0.get(&key.to_string())
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn headers(&self) -> &Option<Headers> {
        &self.headers
    }

    pub fn body(&self) -> &Option<String> {
        &self.body
    }
}

impl From<Vec<&str>> for Request {
    fn from(value: Vec<&str>) -> Self {
        match &value[..] {
            [request_line, headers @ .., body] => {
                let (method, headers, body) =
                    (Method::from(*request_line), Headers::from(headers), body);
                if let Some(content_length) = headers.0.get("Content-Length") {
                    let content_length = content_length
                        .parse::<usize>()
                        .expect("Content-Length should be parsable to usize");
                    Request::new(method, headers, (body[0..content_length]).to_string())
                } else {
                    Request::new(method, headers, (*body).to_string())
                }
            }
            _ => {
                unreachable!();
            }
        }
    }
}

impl<'a> Into<String> for Request {
    fn into(self) -> String {
        let method = String::from(self.method);
        let (method, endpoint) = method.split_once(" ").unwrap();
        let status_line = format!("{} {} HTTP/1.1", method, endpoint);
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

impl Into<String> for &Request {
    fn into(self) -> String {
        let method = String::from(self.method.clone());
        let (method, endpoint) = method.split_once(" ").unwrap();
        let status_line = format!("{} {} HTTP/1.1", method, endpoint);
        let headers = self
            .headers()
            .clone()
            .unwrap_or(Headers(HashMap::new()))
            .0
            .iter()
            .map(|(key, value)| format!("{key}: {value}\r\n"))
            .collect::<String>();
        let body = self.body.clone().unwrap_or("".to_string());
        format!("{status_line}\r\n{headers}\r\n{body}")
    }
}
