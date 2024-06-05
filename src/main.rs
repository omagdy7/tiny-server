#![allow(unused)]
use http_server_starter_rust::router::Router;
use itertools::Itertools;
use nom::AsBytes;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str::Utf8Error;
use std::sync::{Arc, Mutex};
use std::{str, thread, usize};

use http_server_starter_rust::request::*;
use http_server_starter_rust::response::*;
use http_server_starter_rust::{extractor, http_types::*};

fn save_bytes_to_file(bytes: &[u8], file_path: &str) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(bytes)?;
    Ok(())
}

fn read_file_as_bytes(path: &str) -> io::Result<Vec<u8>> {
    // Open the file in read-only mode
    let mut file = File::open(path)?;

    // Create a buffer to hold the file contents
    let mut buffer = Vec::new();

    // Read the file contents into the buffer
    file.read_to_end(&mut buffer)?;

    // Return the buffer
    Ok(buffer)
}

fn handle_echo(request: &Request, ctx: Option<&HashMap<String, String>>) -> Response {
    let mut headers = HashMap::new();
    // Extract the route regardless of the variant
    let mut echo_string = "".to_string();
    let route = match request.method() {
        Method::Get(route) | Method::Post(route) | Method::Put(route) => route,
    };

    for ch in route.chars().skip(1).skip_while(|&ch| ch != '/').skip(1) {
        echo_string.push(ch);
    }
    if echo_string.chars().last().unwrap() == '/' {
        echo_string.pop();
    }
    let len = echo_string.len().to_string();
    headers.insert("Content-Type".to_string(), "text/plain".to_string());
    headers.insert("Content-Length".to_string(), len);
    let body = echo_string;
    Response::new(
        "1.1".to_string(),
        StatusCode::Ok,
        Some(Headers(headers)),
        Some(body),
    )
}

fn handle_post_files(request: &Request, ctx: Option<&HashMap<String, String>>) -> Response {
    // Extract the route regardless of the variant
    let mut file = "".to_string();
    let route = match request.method() {
        Method::Get(route) | Method::Post(route) | Method::Put(route) => route,
    };

    let mut directory = ctx.unwrap().get(&"dir".to_string()).unwrap().clone();
    directory.pop(); // remove last slash

    for ch in route.chars().skip(1).skip_while(|&ch| ch != '/') {
        file.push(ch);
    }
    if file.chars().last().unwrap() == '/' {
        file.pop();
    }
    let len = file.len().to_string();

    let full_path = &(directory + &file);
    println!("post_files");
    dbg!(full_path);
    let bytes = request.body().as_ref().unwrap();
    let body = bytes.as_bytes();

    match save_bytes_to_file(body, full_path) {
        Ok(bytes) => Response::new("1.1".to_string(), StatusCode::Created, None, None),
        Err(err) => {
            println!("Error: {err}");
            Response::new("1.1".to_string(), StatusCode::NotFound, None, None)
        }
    }
}

fn handle_files(request: &Request, ctx: Option<&HashMap<String, String>>) -> Response {
    // Extract the route regardless of the variant
    let mut file = "".to_string();
    let route = match request.method() {
        Method::Get(route) | Method::Post(route) | Method::Put(route) => route,
    };

    let mut directory = ctx.unwrap().get(&"dir".to_string()).unwrap().clone();
    directory.pop(); // remove last slash

    for ch in route.chars().skip(1).skip_while(|&ch| ch != '/') {
        file.push(ch);
    }
    if file.chars().last().unwrap() == '/' {
        file.pop();
    }
    let len = file.len().to_string();

    let full_path = &(directory + &file);
    println!("handle_files");
    dbg!(full_path);

    match read_file_as_bytes(full_path) {
        Ok(bytes) => {
            let mut headers = HashMap::new();
            headers.insert(
                "Content-Type".to_string(),
                "application/octet-stream".to_string(),
            );
            headers.insert("Content-Length".to_string(), bytes.len().to_string());
            let body = String::from_utf8(bytes).unwrap();
            Response::new(
                "1.1".to_string(),
                StatusCode::Ok,
                Some(Headers(headers)),
                Some(body),
            )
        }
        Err(_) => Response::new("1.1".to_string(), StatusCode::NotFound, None, None),
    }
}

fn handle_user_agent(request: &Request, ctx: Option<&HashMap<String, String>>) -> Response {
    let mut headers = HashMap::new();
    let user_agent = request.get_tag("User-Agent".to_string());
    let len = user_agent.len().to_string();
    headers.insert("Content-Type".to_string(), "text/plain".to_string());
    headers.insert("Content-Length".to_string(), len);
    let body = user_agent.to_string();
    Response::new(
        "1.1".to_string(),
        StatusCode::Ok,
        Some(Headers(headers)),
        Some(body),
    )
    .into()
}

fn handle_success(request: &Request, ctx: Option<&HashMap<String, String>>) -> Response {
    Response::new("1.1".to_string(), StatusCode::Ok, None, None).into()
}

fn handle_not_found(request: Request, ctx: Option<&HashMap<String, String>>) -> Response {
    Response::new("1.1".to_string(), StatusCode::NotFound, None, None).into()
}

fn serve(
    mut stream: TcpStream,
    router: Arc<Mutex<Router>>,
    ctx: Arc<Mutex<HashMap<String, String>>>,
) -> io::Result<usize> {
    // Buffer to store the data received from the client
    let mut buffer = [0; 512];

    // Read data from the stream
    match stream.read(&mut buffer) {
        Ok(_) => {
            // Convert buffer to a string and print the received data
            match str::from_utf8(&buffer) {
                Ok(request) => {
                    use Method::*;
                    println!("Received request:\n{}", request);
                    let request_lines: Vec<&str> = request.split("\r\n").collect();
                    dbg!(&request_lines);
                    let request = Request::from(request_lines);
                    let request_string: String = (&request).into();
                    println!("body:\n{:?}", request.body());

                    let response: String = {
                        let router = router.lock().unwrap();
                        let ctx = ctx.lock().unwrap();
                        router.handle(&request, Some(&ctx)).into()
                    };
                    stream.write(response.as_bytes())
                }
                Err(_) => todo!(),
            }
        }
        Err(_) => todo!(),
    }
}

fn main() -> io::Result<()> {
    // Collect the command-line arguments
    let args: Vec<String> = std::env::args().collect();

    let mut dir = "".to_string();

    let ctx = Arc::new(Mutex::new(HashMap::new()));

    // Check if the correct number of arguments are provided
    if args.len() == 3 {
        // Parse the arguments
        if args[1] == "--directory" {
            dir += &args[2];
            println!("Directory: {}", dir);
        } else {
            eprintln!("Unknown argument: {}", args[1]);
            eprintln!("Usage: {} --directory <path>", args[0]);
            return Ok(());
        }
    } else {
        eprintln!("Usage: {} --directory <path>", args[0]);
    }

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    let router = Arc::new(Mutex::new(Router::new()));
    ctx.lock().unwrap().insert("dir".to_string(), dir);

    {
        let mut router = router.lock().unwrap();
        router
            .route(get("/"), handle_success)
            .route(get("/echo/:var/"), handle_echo)
            .route(get("/user-agent/"), handle_user_agent)
            .route(get("/files/:file/"), handle_files)
            .route(post("/files/:file/"), handle_post_files);
    }

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let router = Arc::clone(&router);
                let ctx = Arc::clone(&ctx);
                thread::spawn(move || {
                    if let Err(e) = serve(stream, router, ctx) {
                        eprintln!("Failed to serve connection: {}", e);
                    }
                });
            }
            Err(e) => {
                eprintln!("error: {}", e);
            }
        }
    }

    Ok(())
}
