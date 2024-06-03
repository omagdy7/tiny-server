#![allow(unused)]
use itertools::Itertools;
use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::{str, thread};

use http_server_starter_rust::http_types::*;
use http_server_starter_rust::request::*;
use http_server_starter_rust::response::*;

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
                let succses: String =
                    Response::new("1.1".to_string(), StatusCode::Ok, None, None).into();
                let succses = succses.as_bytes();

                let not_found: String =
                    Response::new("1.1".to_string(), StatusCode::NotFound, None, None).into();
                let not_found = not_found.as_bytes();

                match request.method {
                    HTTPMethod::Get((endpoint, target)) => {
                        match (endpoint.as_str(), target.as_str()) {
                            ("", "") => match stream.write(succses) {
                                Ok(_) => {
                                    println!("Response sent successfully");
                                }
                                Err(e) => eprintln!("Failed to send response: {}", e),
                            },
                            ("echo", target) => {
                                let mut headers = HashMap::new();
                                headers
                                    .insert("Content-Type".to_string(), "text/plain".to_string());
                                headers.insert(
                                    "Content-Length".to_string(),
                                    (target.len() - 1).to_string(),
                                );
                                let body = target[0..target.len() - 1].to_string();
                                let response: String = Response::new(
                                    "1.1".to_string(),
                                    StatusCode::Ok,
                                    Some(Headers(headers)),
                                    Some(body),
                                )
                                .into();
                                dbg!(&response);
                                let response = response.as_bytes();

                                match stream.write(response) {
                                    Ok(_) => {
                                        println!("Response sent successfully");
                                        println!("Hello echo");
                                    }
                                    Err(e) => eprintln!("Failed to send response: {}", e),
                                }
                            }
                            ("user-agent", _) => {
                                let mut headers = HashMap::new();
                                let user_agent: String =
                                    request.headers.unwrap().0.get("User-Agent").unwrap().into();
                                headers
                                    .insert("Content-Type".to_string(), "text/plain".to_string());
                                headers.insert(
                                    "Content-Length".to_string(),
                                    user_agent.len().to_string(),
                                );
                                let body = user_agent;
                                let response: String = Response::new(
                                    "1.1".to_string(),
                                    StatusCode::Ok,
                                    Some(Headers(headers)),
                                    Some(body),
                                )
                                .into();
                                dbg!(&response);
                                let response = response.as_bytes();

                                match stream.write(response) {
                                    Ok(_) => {
                                        println!("Response sent successfully");
                                        println!("Hello user-agent");
                                    }
                                    Err(e) => eprintln!("Failed to send response: {}", e),
                                }
                            }
                            _ => match stream.write(not_found) {
                                Ok(_) => println!("Response sent successfully"),
                                Err(e) => eprintln!("Failed to send response: {}", e),
                            },
                        }
                    }
                    HTTPMethod::Post(_target) => todo!(),
                    HTTPMethod::Put(_target) => todo!(),
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to read from stream: {}", e);
        }
    }
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || handle_client(stream));
            }
            Err(e) => {
                eprintln!("error: {}", e);
            }
        }
    }

    Ok(())
}
