use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str;

use itertools::Itertools;

enum StatusCode {
    // 2xx Success
    Ok,
    // Created = 201,
    // Accepted = 202,
    // NonAuthoritative = 203,
    // NoContent = 204,
    // ResetContent = 205,
    // PartialContent = 206,

    // 4xx Client Error
    BadRequest,
    NotFound,
}

#[derive(Debug)]
enum HTTPMethod {
    Get(String),
    Post(String),
    Put(String),
}

impl From<StatusCode> for String {
    fn from(val: StatusCode) -> Self {
        use StatusCode::*;
        match val {
            Ok => "200 OK".to_string(),
            BadRequest => "400 Bad Request".to_string(),
            NotFound => "404 Not Found".to_string(),
        }
    }
}

impl From<HTTPMethod> for String {
    fn from(val: HTTPMethod) -> Self {
        use HTTPMethod::*;
        match val {
            Get(target) => "GET".to_string() + &target,
            Post(target) => "POST".to_string() + &target,
            Put(target) => "PUT".to_string() + &target,
        }
    }
}

#[derive(Debug)]
struct Headers(HashMap<String, String>);

impl From<&[&str]> for Headers {
    fn from(value: &[&str]) -> Self {
        let mut header_map = HashMap::new();
        for header in value.iter().filter(|val| !val.is_empty()) {
            let (key, val) = header.split_once(':').expect("Should be splitable by :");
            header_map.insert(key.to_string(), val.to_string());
        }
        Headers(header_map)
    }
}

impl From<&str> for HTTPMethod {
    fn from(val: &str) -> Self {
        use HTTPMethod::*;
        let request_line = val.split(' ').collect_vec();
        let (method, target) = (request_line[0], request_line[1]);
        match method {
            "GET" => Get(target.to_string()),
            "POST" => Post(target.to_string()),
            _ => {
                eprintln!("{method} Not Supported Yet");
                unreachable!()
            }
        }
    }
}

#[derive(Debug)]
struct Request {
    method: HTTPMethod,
    headers: Option<Headers>,
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

struct Response {
    version: String,
    status: StatusCode,
    body: Option<String>,
}

impl Response {
    fn new(version: String, status: StatusCode, body: Option<String>) -> Self {
        Response {
            version,
            status,
            body,
        }
    }
}

impl Into<String> for Response {
    fn into(self) -> String {
        format!(
            "HTTP/{} {}\r\n{}\r\n",
            self.version,
            String::from(self.status),
            self.body.unwrap_or("".to_string())
        )
    }
}

fn handle_client(mut stream: TcpStream) {
    // Buffer to store the data received from the client
    let mut buffer = [0; 512];

    // Read data from the stream
    match stream.read(&mut buffer) {
        Ok(_) => {
            // Convert buffer to a string and print the received data
            if let Ok(request) = str::from_utf8(&buffer) {
                println!("Received request: {}", request);
                let request = Request::from(request);
                println!("Request after parsing: {:?}", request);
                let succses: String = Response::new("1.1".to_string(), StatusCode::Ok, None).into();
                let succses = succses.as_bytes();

                let not_found: String =
                    Response::new("1.1".to_string(), StatusCode::NotFound, None).into();
                let not_found = not_found.as_bytes();

                match request.method {
                    HTTPMethod::Get(target) => match target.as_str() {
                        "/" => match stream.write(succses) {
                            Ok(_) => println!("Response sent successfully"),
                            Err(e) => eprintln!("Failed to send response: {}", e),
                        },
                        _ => match stream.write(not_found) {
                            Ok(_) => println!("Response sent successfully"),
                            Err(e) => eprintln!("Failed to send response: {}", e),
                        },
                    },
                    HTTPMethod::Post(target) => todo!(),
                    HTTPMethod::Put(target) => todo!(),
                }
            }

            // Prepare a response
            // let response = b"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nHello, world!";
            //
        }
        Err(e) => {
            eprintln!("Failed to read from stream: {}", e);
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // println!("accepted new connection");
                handle_client(stream)
            }
            Err(e) => {
                eprintln!("error: {}", e);
            }
        }
    }
}
