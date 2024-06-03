use itertools::Itertools;
use std::collections::HashMap;

pub enum StatusCode {
    // 2xx Success
    Ok,

    // 4xx Client Error
    BadRequest,
    NotFound,
}

type Endpoint = String;
type Target = String;

#[derive(Debug)]
pub enum HTTPMethod {
    Get((Endpoint, Target)),
    Post((Endpoint, Target)),
    Put((Endpoint, Target)),
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
            Get((endpoint, target)) => "GET".to_string() + &endpoint + &target,
            Post((endpoint, target)) => "POST".to_string() + &endpoint + &target,
            Put((endpoint, target)) => "PUT".to_string() + &endpoint + &target,
        }
    }
}

#[derive(Debug)]
pub struct Headers(pub HashMap<String, String>);

impl From<&[&str]> for Headers {
    fn from(value: &[&str]) -> Self {
        let mut header_map = HashMap::new();
        for header in value.iter().filter(|val| !val.is_empty()) {
            let (key, val) = header
                .split_once(": ")
                .expect("Should be splitable by :<space>");
            header_map.insert(key.to_string(), val.to_string());
        }
        Headers(header_map)
    }
}

impl From<&str> for HTTPMethod {
    fn from(val: &str) -> Self {
        use HTTPMethod::*;
        let request_line = val.split(' ').collect_vec();
        let (method, info) = (request_line[0], request_line[1]);
        let info = info.chars().skip(1).collect::<String>() + &"/";
        let (endpoint, target) = info.split_once("/").expect("Should be splitable by /");
        match method {
            "GET" => Get((endpoint.to_string(), target.to_string())),
            "POST" => Post((endpoint.to_string(), target.to_string())),
            _ => {
                eprintln!("{method} Not Supported Yet");
                unreachable!()
            }
        }
    }
}
