use crate::http_types::*;

#[derive(Debug)]
pub struct Request {
    pub method: HTTPMethod,
    pub headers: Option<Headers>,
    body: Option<String>,
}

impl Request {
    fn new(method: HTTPMethod, headers: Headers, body: String) -> Self {
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
}

impl From<&str> for Request {
    fn from(val: &str) -> Self {
        let request: Vec<&str> = val.split("\r\n").collect();
        match &request[..] {
            [request_line, headers @ .., body] => {
                let (method, headers, body) = (
                    HTTPMethod::from(*request_line),
                    Headers::from(headers),
                    body.to_string(),
                );
                Request::new(method, headers, body)
            }
            _ => {
                unreachable!();
            }
        }
    }
}
