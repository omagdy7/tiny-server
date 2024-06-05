use crate::{
    extractor::build_regex_from_path,
    http_types::{Method, StatusCode},
    request::Request,
    response::Response,
};
use regex::Regex;
use std::collections::HashMap;

type Handler = fn(&Request, Option<&HashMap<String, String>>) -> Response;
type Routes = HashMap<Method, Handler>;

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
    pub fn route(&mut self, method: Method, handler: Handler) -> &mut Self {
        use Method::*;

        let method_string = match &method {
            Get(s) | Post(s) | Put(s) => s,
        };

        let re = build_regex_from_path(method_string.as_str());
        let meth = Get(re.to_string());
        dbg!(&meth);
        self.routes.insert(meth, handler);
        self
    }

    // Handle incoming requests
    pub fn handle(&self, request: &Request, ctx: Option<&HashMap<String, String>>) -> Response {
        use Method::*;
        let method_string = match &request.method {
            Get(s) | Post(s) | Put(s) => s,
        };
        for (method, handler) in self.routes() {
            let route_method = match method {
                Get(s) | Post(s) | Put(s) => s.as_str(),
            };
            let re = Regex::new(route_method).unwrap();
            dbg!(&re, method_string);
            if re.is_match(method_string) {
                return handler(request, ctx);
            }
        }
        Response::new("1.1".to_string(), StatusCode::NotFound, None, None).into()
    }
}
