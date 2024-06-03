use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str;

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
}

impl From<StatusCode> for String {
    fn from(val: StatusCode) -> Self {
        use StatusCode::*;
        match val {
            Ok => "200 OK".to_string(),
            BadRequest => "400 Bad Request".to_string(),
        }
    }
}

struct HTTPResponse {
    version: String,
    status: StatusCode,
    headers: Option<String>,
    body: Option<String>,
}

impl HTTPResponse {
    fn new(
        version: String,
        status: StatusCode,
        headers: Option<String>,
        body: Option<String>,
    ) -> Self {
        HTTPResponse {
            version,
            status,
            headers,
            body,
        }
    }
}

impl Into<String> for HTTPResponse {
    fn into(self) -> String {
        format!(
            "HTTP/{} {}\r\n{}\r\n{}",
            self.version,
            String::from(self.status),
            self.headers.unwrap_or("".to_string()),
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
            }

            // Prepare a response
            // let response = b"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nHello, world!";
            let response: String =
                HTTPResponse::new("1.1".to_string(), StatusCode::Ok, None, None).into();
            let response = response.as_bytes();

            // Write response to the stream
            match stream.write(response) {
                Ok(_) => println!("Response sent successfully"),
                Err(e) => eprintln!("Failed to send response: {}", e),
            }
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
