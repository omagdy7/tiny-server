use crate::{
    extractor::build_regex_from_path,
    http::{get, post, put, Endpoint, Method, StatusCode},
    request::Request,
    response::Response,
};
use regex::Regex;
use std::collections::HashMap;

type Handler = fn(&Request, Option<&HashMap<String, String>>) -> Response;
type Routes = HashMap<Endpoint, Handler>;

pub struct Router {
    routes: Routes,
}

impl Router {
    // Create a new Router
    pub fn new() -> Self {
        Router {
            routes: HashMap::new(),
        }
    }

    pub fn routes(&self) -> &Routes {
        &self.routes
    }

    // Add a route to the router
    pub fn route(&mut self, endpoint: Endpoint, handler: Handler) -> &mut Self {
        use Method as M;
        match (endpoint.method, endpoint.route) {
            (M::GET, route) => {
                let re = build_regex_from_path(&route);
                let epoint = get(re.as_str());
                self.routes.insert(epoint, handler);
            }
            (M::POST, route) => {
                let re = build_regex_from_path(&route);
                let epoint = post(re.as_str());
                self.routes.insert(epoint, handler);
            }
            (M::PUT, route) => {
                let re = build_regex_from_path(&route);
                let meth = put(re.as_str());
                self.routes.insert(meth, handler);
            }
        }
        self
    }

    // Handle incoming requests
    pub fn handle(&self, request: &Request, ctx: Option<&HashMap<String, String>>) -> Response {
        use Method as M;
        for (endpoint, handler) in self.routes() {
            let repoint = &request.endpoint;
            match (
                &repoint.method,
                &repoint.route,
                &endpoint.method,
                &endpoint.route,
            ) {
                (M::GET, request_route, M::GET, endpoint_regex) => {
                    let re = Regex::new(&endpoint_regex).unwrap();
                    // dbg!(&re, request_method);
                    if re.is_match(&request_route) {
                        return handler(request, ctx);
                    }
                }
                (M::POST, request_route, M::POST, endpoint_regex) => {
                    let re = Regex::new(&endpoint_regex).unwrap();
                    // dbg!(&re, request_method);
                    if re.is_match(&request_route) {
                        return handler(request, ctx);
                    }
                }
                (M::PUT, request_route, M::PUT, endpoint_regex) => {
                    let re = Regex::new(&endpoint_regex).unwrap();
                    // dbg!(&re, request_method);
                    if re.is_match(&request_route) {
                        return handler(request, ctx);
                    }
                }
                _ => {}
            }
        }
        Response::new("1.1".to_string(), StatusCode::NotFound, None, None)
    }
}
