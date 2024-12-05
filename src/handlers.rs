use crate::http::*;
use crate::request::*;
use crate::response::*;
use crate::utils::*;
use std::collections::HashMap;

pub fn handle_echo(request: &Request, ctx: Option<&HashMap<String, String>>) -> Response {
    let mut headers = HashMap::new();
    let mut echo_string = "".to_string();
    let route = request.endpoint.route.clone();

    let mut body = vec![];

    for ch in route.chars().skip(1).skip_while(|&ch| ch != '/').skip(1) {
        echo_string.push(ch);
    }
    if echo_string.chars().last().unwrap() == '/' {
        echo_string.pop();
    }

    if let Some(encoding) = request.get_tag("Accept-Encoding") {
        if encoding.contains("gzip") {
            headers.insert("Content-Encoding".to_string(), "gzip".to_string());
            match encode_gzip_string(echo_string.as_str()) {
                Ok(encoded_bytes) => {
                    println!("In succses");
                    let len = encoded_bytes.len();
                    body = encoded_bytes;
                    headers.insert("Content-Length".to_string(), len.to_string());
                }
                Err(err) => {
                    println!("In error {}", &echo_string);
                    println!("Error: {err}");
                }
            }
        } else {
            let len = echo_string.len();
            headers.insert("Content-Length".to_string(), len.to_string());
            body = echo_string.as_bytes().to_owned();
        }
    } else {
        let len = echo_string.len();
        headers.insert("Content-Length".to_string(), len.to_string());
        body = echo_string.as_bytes().to_owned();
    }

    headers.insert("Content-Type".to_string(), "text/plain".to_string());

    Response::new(
        "1.1".to_string(),
        StatusCode::Ok,
        Some(Headers(headers)),
        Some(body),
    )
}

pub fn handle_post_files(request: &Request, ctx: Option<&HashMap<String, String>>) -> Response {
    // Extract the route regardless of the variant
    let mut file = "".to_string();
    let route = request.endpoint.route.clone();

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

pub fn handle_files(request: &Request, ctx: Option<&HashMap<String, String>>) -> Response {
    // Extract the route regardless of the variant
    let mut file = "".to_string();
    let route = request.endpoint.route.clone();

    let mut directory = ctx.unwrap().get(&"dir".to_string()).unwrap().clone();
    directory.pop(); // remove last slash

    for ch in route.chars().skip(1).skip_while(|&ch| ch != '/') {
        file.push(ch);
    }
    if file.chars().last().unwrap() == '/' {
        file.pop();
    }

    let full_path = &(directory + &file);

    match read_file_as_bytes(full_path) {
        Ok(bytes) => {
            let mut headers = HashMap::new();
            headers.insert("Content-Type".to_string(), "text/html".to_string());
            // headers.insert(
            //     "Content-Type".to_string(),
            //     "application/octet-stream".to_string(),
            // );
            headers.insert("Content-Length".to_string(), bytes.len().to_string());
            let body = bytes;
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

pub fn handle_user_agent(request: &Request, ctx: Option<&HashMap<String, String>>) -> Response {
    let mut headers = HashMap::new();
    if let Some(user_agent) = request.get_tag("User-Agent") {
        let len = user_agent.len().to_string();
        headers.insert("Content-Type".to_string(), "text/plain".to_string());
        headers.insert("Content-Length".to_string(), len);
        let body = user_agent.as_bytes().to_owned();
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

pub fn handle_success(request: &Request, ctx: Option<&HashMap<String, String>>) -> Response {
    Response::new("1.1".to_string(), StatusCode::Ok, None, None).into()
}

pub fn handle_not_found(request: Request, ctx: Option<&HashMap<String, String>>) -> Response {
    Response::new("1.1".to_string(), StatusCode::NotFound, None, None).into()
}
