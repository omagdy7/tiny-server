use itertools::Itertools;
use std::collections::HashMap;

pub enum StatusCode {
    // 2xx Success
    Ok,
    Created,

    // 4xx Client Error
    BadRequest,
    NotFound,
}

type Route = String;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Method {
    Get(Route),
    Post(Route),
    Put(Route),
}

pub fn get(route: &str) -> Method {
    Method::Get(route.to_string())
}

pub fn post(route: &str) -> Method {
    Method::Post(route.to_string())
}

pub fn put(route: &str) -> Method {
    Method::Put(route.to_string())
}

impl From<StatusCode> for String {
    fn from(val: StatusCode) -> Self {
        use StatusCode::*;
        match val {
            Ok => "200 OK".to_string(),
            Created => "201 Created".to_string(),
            BadRequest => "400 Bad Request".to_string(),
            NotFound => "404 Not Found".to_string(),
        }
    }
}

impl From<Method> for String {
    fn from(val: Method) -> Self {
        use Method::*;
        match val {
            Get(route) => "GET ".to_string() + &route,
            Post(route) => "POST ".to_string() + &route,
            Put(route) => "PUT ".to_string() + &route,
        }
    }
}
impl From<&Method> for String {
    fn from(val: &Method) -> Self {
        use Method::*;
        match val {
            Get(route) => "GET ".to_string() + &route,
            Post(route) => "POST ".to_string() + &route,
            Put(route) => "PUT ".to_string() + &route,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Headers(pub HashMap<String, String>);

impl From<Vec<String>> for Headers {
    fn from(value: Vec<String>) -> Self {
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

impl From<String> for Method {
    fn from(val: String) -> Self {
        use Method::*;
        let request_line = val.split(' ').collect_vec();
        let (method, route) = (request_line[0], request_line[1]);
        match method {
            "GET" => Get(route.to_string()),
            "POST" => Post(route.to_string()),
            _ => {
                eprintln!("{method} Not Supported Yet");
                unreachable!()
            }
        }
    }
}
impl From<&str> for Method {
    fn from(val: &str) -> Self {
        use Method::*;
        let request_line = val.split(' ').collect_vec();
        let (method, route) = (request_line[0], request_line[1]);
        match method {
            "GET" => Get(route.to_string()),
            "POST" => Post(route.to_string()),
            _ => {
                eprintln!("{method} Not Supported Yet");
                unreachable!()
            }
        }
    }
}
