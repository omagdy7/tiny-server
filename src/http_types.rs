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
    GET,
    POST,
    PUT,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Endpoint {
    pub method: Method,
    pub route: Route,
}

impl Endpoint {
    fn get(route: &str) -> Self {
        Self {
            method: Method::GET,
            route: route.to_string(),
        }
    }
    fn post(route: &str) -> Self {
        Self {
            method: Method::POST,
            route: route.to_string(),
        }
    }
    fn put(route: &str) -> Self {
        Self {
            method: Method::PUT,
            route: route.to_string(),
        }
    }
}

pub fn get(route: &str) -> Endpoint {
    Endpoint::get(route)
}

pub fn post(route: &str) -> Endpoint {
    Endpoint::post(route)
}

pub fn put(route: &str) -> Endpoint {
    Endpoint::put(route)
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

impl From<Endpoint> for String {
    fn from(val: Endpoint) -> Self {
        use Method as M;
        match (val.method, val.route) {
            (M::GET, route) => "GET ".to_string() + &route,
            (M::POST, route) => "GET ".to_string() + &route,
            (M::PUT, route) => "GET ".to_string() + &route,
        }
    }
}
impl From<&Endpoint> for String {
    fn from(val: &Endpoint) -> Self {
        use Method as M;
        match (&val.method, &val.route) {
            (M::GET, route) => "GET ".to_string() + &route,
            (M::POST, route) => "GET ".to_string() + &route,
            (M::PUT, route) => "GET ".to_string() + &route,
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

impl From<String> for Endpoint {
    fn from(val: String) -> Self {
        let request_line = val.split(' ').collect_vec();
        let (method, route) = (request_line[0], request_line[1]);
        match method {
            "GET" => Endpoint::get(route),
            "POST" => Endpoint::post(route),
            _ => {
                eprintln!("{method} Not Supported Yet");
                unreachable!()
            }
        }
    }
}
impl From<&str> for Endpoint {
    fn from(val: &str) -> Self {
        let request_line = val.split(' ').collect_vec();
        let (method, route) = (request_line[0], request_line[1]);
        match method {
            "GET" => Endpoint::get(route),
            "POST" => Endpoint::post(route),
            _ => {
                eprintln!("{method} Not Supported Yet");
                unreachable!()
            }
        }
    }
}
