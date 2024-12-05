use itertools::Itertools;
use std::collections::HashMap;

pub enum StatusCode {
    // 1xx Informational
    Continue,           // 100
    SwitchingProtocols, // 101
    Processing,         // 102

    // 2xx Success
    Ok,                          // 200
    Created,                     // 201
    Accepted,                    // 202
    NonAuthoritativeInformation, // 203
    NoContent,                   // 204
    ResetContent,                // 205
    PartialContent,              // 206
    MultiStatus,                 // 207
    AlreadyReported,             // 208
    ImUsed,                      // 226

    // 3xx Redirection
    MultipleChoices,   // 300
    MovedPermanently,  // 301
    Found,             // 302
    SeeOther,          // 303
    NotModified,       // 304
    UseProxy,          // 305
    TemporaryRedirect, // 307
    PermanentRedirect, // 308

    // 4xx Client Error
    BadRequest,                  // 400
    Unauthorized,                // 401
    PaymentRequired,             // 402
    Forbidden,                   // 403
    NotFound,                    // 404
    MethodNotAllowed,            // 405
    NotAcceptable,               // 406
    ProxyAuthenticationRequired, // 407
    RequestTimeout,              // 408
    Conflict,                    // 409
    Gone,                        // 410
    LengthRequired,              // 411
    PreconditionFailed,          // 412
    PayloadTooLarge,             // 413
    UriTooLong,                  // 414
    UnsupportedMediaType,        // 415
    RangeNotSatisfiable,         // 416
    ExpectationFailed,           // 417
    ImaTeapot,                   // 418
    MisdirectedRequest,          // 421
    UnprocessableEntity,         // 422
    Locked,                      // 423
    FailedDependency,            // 424
    TooEarly,                    // 425
    UpgradeRequired,             // 426
    PreconditionRequired,        // 428
    TooManyRequests,             // 429
    RequestHeaderFieldsTooLarge, // 431
    UnavailableForLegalReasons,  // 451

    // 5xx Server Error
    InternalServerError,           // 500
    NotImplemented,                // 501
    BadGateway,                    // 502
    ServiceUnavailable,            // 503
    GatewayTimeout,                // 504
    HttpVersionNotSupported,       // 505
    VariantAlsoNegotiates,         // 506
    InsufficientStorage,           // 507
    LoopDetected,                  // 508
    NotExtended,                   // 510
    NetworkAuthenticationRequired, // 511
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
            // 1xx Informational
            Continue => "100 Continue".to_string(),
            SwitchingProtocols => "101 Switching Protocols".to_string(),
            Processing => "102 Processing".to_string(),

            // 2xx Success
            Ok => "200 OK".to_string(),
            Created => "201 Created".to_string(),
            Accepted => "202 Accepted".to_string(),
            NonAuthoritativeInformation => "203 Non-Authoritative Information".to_string(),
            NoContent => "204 No Content".to_string(),
            ResetContent => "205 Reset Content".to_string(),
            PartialContent => "206 Partial Content".to_string(),
            MultiStatus => "207 Multi-Status".to_string(),
            AlreadyReported => "208 Already Reported".to_string(),
            ImUsed => "226 IM Used".to_string(),

            // 3xx Redirection
            MultipleChoices => "300 Multiple Choices".to_string(),
            MovedPermanently => "301 Moved Permanently".to_string(),
            Found => "302 Found".to_string(),
            SeeOther => "303 See Other".to_string(),
            NotModified => "304 Not Modified".to_string(),
            UseProxy => "305 Use Proxy".to_string(),
            TemporaryRedirect => "307 Temporary Redirect".to_string(),
            PermanentRedirect => "308 Permanent Redirect".to_string(),

            // 4xx Client Error
            BadRequest => "400 Bad Request".to_string(),
            Unauthorized => "401 Unauthorized".to_string(),
            PaymentRequired => "402 Payment Required".to_string(),
            Forbidden => "403 Forbidden".to_string(),
            NotFound => "404 Not Found".to_string(),
            MethodNotAllowed => "405 Method Not Allowed".to_string(),
            NotAcceptable => "406 Not Acceptable".to_string(),
            ProxyAuthenticationRequired => "407 Proxy Authentication Required".to_string(),
            RequestTimeout => "408 Request Timeout".to_string(),
            Conflict => "409 Conflict".to_string(),
            Gone => "410 Gone".to_string(),
            LengthRequired => "411 Length Required".to_string(),
            PreconditionFailed => "412 Precondition Failed".to_string(),
            PayloadTooLarge => "413 Payload Too Large".to_string(),
            UriTooLong => "414 URI Too Long".to_string(),
            UnsupportedMediaType => "415 Unsupported Media Type".to_string(),
            RangeNotSatisfiable => "416 Range Not Satisfiable".to_string(),
            ExpectationFailed => "417 Expectation Failed".to_string(),
            ImaTeapot => "418 I'm a teapot".to_string(),
            MisdirectedRequest => "421 Misdirected Request".to_string(),
            UnprocessableEntity => "422 Unprocessable Entity".to_string(),
            Locked => "423 Locked".to_string(),
            FailedDependency => "424 Failed Dependency".to_string(),
            TooEarly => "425 Too Early".to_string(),
            UpgradeRequired => "426 Upgrade Required".to_string(),
            PreconditionRequired => "428 Precondition Required".to_string(),
            TooManyRequests => "429 Too Many Requests".to_string(),
            RequestHeaderFieldsTooLarge => "431 Request Header Fields Too Large".to_string(),
            UnavailableForLegalReasons => "451 Unavailable For Legal Reasons".to_string(),

            // 5xx Server Error
            InternalServerError => "500 Internal Server Error".to_string(),
            NotImplemented => "501 Not Implemented".to_string(),
            BadGateway => "502 Bad Gateway".to_string(),
            ServiceUnavailable => "503 Service Unavailable".to_string(),
            GatewayTimeout => "504 Gateway Timeout".to_string(),
            HttpVersionNotSupported => "505 HTTP Version Not Supported".to_string(),
            VariantAlsoNegotiates => "506 Variant Also Negotiates".to_string(),
            InsufficientStorage => "507 Insufficient Storage".to_string(),
            LoopDetected => "508 Loop Detected".to_string(),
            NotExtended => "510 Not Extended".to_string(),
            NetworkAuthenticationRequired => "511 Network Authentication Required".to_string(),
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
