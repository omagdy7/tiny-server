#![allow(unused)]
use http_server_starter_rust::router::Router;
use itertools::Itertools;
use nom::AsBytes;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::str::Utf8Error;
use std::sync::{Arc, Mutex};
use std::{str, thread, usize};

use http_server_starter_rust::request::*;
use http_server_starter_rust::response::*;
use http_server_starter_rust::server::*;
use http_server_starter_rust::utils::*;
use http_server_starter_rust::{extractor, http_types::*};

fn handle_echo(request: &Request, ctx: Option<&HashMap<String, String>>) -> Response {
    let mut headers = HashMap::new();
    let mut echo_string = "".to_string();
    let route = match request.method() {
        Method::Get(route) | Method::Post(route) | Method::Put(route) => route,
    };

    if let Some(encoding) = request.get_tag("Accept-Encoding") {
        if encoding.as_str() == "gzip" {
            headers.insert("Content-Encoding".to_string(), "gzip".to_string());
        }
    }

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
    if let Some(user_agent) = request.get_tag("User-Agent") {
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
    } else {
        println!("User-Agent isn't present in headers");
        Response::new("1.1".to_string(), StatusCode::BadRequest, None, None)
    }
}

fn handle_success(request: &Request, ctx: Option<&HashMap<String, String>>) -> Response {
    Response::new("1.1".to_string(), StatusCode::Ok, None, None).into()
}

fn handle_not_found(request: Request, ctx: Option<&HashMap<String, String>>) -> Response {
    Response::new("1.1".to_string(), StatusCode::NotFound, None, None).into()
}

fn main() -> io::Result<()> {
    // Collect the command-line arguments
    let args: Vec<String> = std::env::args().collect();

    let mut dir = "".to_string();

    // Check if the correct number of arguments are provided
    if args.len() == 3 {
        // Parse the arguments
        if args[1] == "--directory" {
            dir += &args[2];
            println!("Directory: {}", dir);
        } else {
            eprintln!("Unknown argument: {}", args[1]);
            eprintln!("Usage: {} --directory <path>", args[0]);
        }
    } else {
        eprintln!("Usage: {} --directory <path>", args[0]);
    }

    let mut router: Router = Router::new();

    let mut ctx = HashMap::new();
    dbg!(&dir);
    ctx.insert("dir".to_string(), dir);

    router
        .route(get("/"), handle_success)
        .route(get("/echo/:var/"), handle_echo)
        .route(get("/user-agent/"), handle_user_agent)
        .route(get("/files/:file/"), handle_files)
        .route(post("/files/:file/"), handle_post_files);

    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 4221);
    let app = Server::new(socket);
    app.serve(&router, Some(&ctx))
}
