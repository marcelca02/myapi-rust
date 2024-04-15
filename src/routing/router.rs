// Router structure for handling routes in the server api

use std::collections::HashMap;

use crate::routing::route::Route;
use crate::http::{methods::HttpMethod,request::Request, response::Response};



// TODO: Implement my own HashMap
#[allow(dead_code)]
pub struct Router {
    routes: HashMap<String, Vec<Route>>,
    num_routes: usize,
}


#[allow(dead_code)]
impl Router {

    // Constructor
    pub fn new() -> Self {
        let mut methods = HashMap::new();
        methods.insert("GET".to_string(), Vec::<Route>::new());
        methods.insert("POST".to_string(), Vec::<Route>::new());
        methods.insert("PUT".to_string(), Vec::<Route>::new());
        methods.insert("DELETE".to_string(), Vec::<Route>::new());
        methods.insert("OPTIONS".to_string(), Vec::<Route>::new());
        methods.insert("PATCH".to_string(), Vec::<Route>::new());

        Router {
            routes: methods,
            num_routes: 0,
        }
    }

    // Private method to create a new Route
    fn register_route(&mut self, method: HttpMethod, path: &str, action: Box<dyn Fn()>) {
        let method = method.to_string();
        let route = Route::new(path, action);
        self.routes.entry(method).or_insert(Vec::new()).push(route);
    }

    //Resolvers
    pub fn resolve_route(&self, req: &Request) -> &Route {
        let method = req.get_method().to_string();
        let routes = self.routes.get(&method).unwrap();
        let mut route = routes.iter().find(|r| r.get_uri() == req.get_uri());
        match route {
            Some(r) => r,
            None => panic!("404 Route not found")
        }
    }

    // pub fn resolve(&self, req: &Request) -> Response {
    //
    // }


    
    // PUBLIC API METHODS

    // Public method to create a new GET Route
    pub fn get(&mut self, path: &str, action: Box<dyn Fn()>) {
        self.register_route(HttpMethod::GET, path, action);
    }

    // Public method to create a new POST Route
    pub fn post(&mut self, path: &str, action: Box<dyn Fn()>) {
        self.register_route(HttpMethod::POST, path, action);
    }

    // Public method to create a new PUT Route
    pub fn put(&mut self, path: &str, action: Box<dyn Fn()>) {
        self.register_route(HttpMethod::PUT, path, action);
    }

    // Public method to create a new DELETE Route
    pub fn delete(&mut self, path: &str, action: Box<dyn Fn()>) {
        self.register_route(HttpMethod::DELETE, path, action);
    }

    // Public method to create a new OPTIONS Route
    pub fn options(&mut self, path: &str, action: Box<dyn Fn()>) {
        self.register_route(HttpMethod::OPTIONS, path, action);
    }

    // Public method to create a new PATCH Route
    pub fn patch(&mut self, path: &str, action: Box<dyn Fn()>) {
        self.register_route(HttpMethod::PATCH, path, action);
    }
}




