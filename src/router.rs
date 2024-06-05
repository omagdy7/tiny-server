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
        match method {
            Get(route) => {
                let re = build_regex_from_path(&route);
                let meth = Get(re.to_string());
                dbg!(&meth);
                self.routes.insert(meth, handler);
            }
            Post(route) => {
                let re = build_regex_from_path(&route);
                let meth = Post(re.to_string());
                dbg!(&meth);
                self.routes.insert(meth, handler);
            }
            Put(_) => todo!(),
        }
        self
    }

    // Handle incoming requests
    pub fn handle(&self, request: &Request, ctx: Option<&HashMap<String, String>>) -> Response {
        use Method::*;
        match &request.method {
            Get(request_method) => {
                for (method, handler) in self.routes() {
                    if let Get(method_string) = method {
                        let re = Regex::new(method_string).unwrap();
                        dbg!(&re, request_method);
                        if re.is_match(request_method) {
                            return handler(request, ctx);
                        }
                    }
                }
                Response::new("1.1".to_string(), StatusCode::NotFound, None, None).into()
            }
            Post(request_method) => {
                for (method, handler) in self.routes() {
                    if let Post(method_string) = method {
                        let re = Regex::new(method_string).unwrap();
                        dbg!(&re, request_method);
                        if re.is_match(request_method) {
                            return handler(request, ctx);
                        }
                    }
                }
                Response::new("1.1".to_string(), StatusCode::NotFound, None, None).into()
            }
            Put(_) => todo!(),
        }
    }
}
