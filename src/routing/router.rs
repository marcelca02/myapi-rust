// Router structure for handling routes in the server api

use std::collections::HashMap;

use crate::routing::route::Route;
use crate::http::{methods::HttpMethod,request::Request,response::Response, status::HttpStatus};

// TODO: Implement my own HashMap
pub struct Router {
    routes: HashMap<String, Vec<Route>>,
    num_routes: usize,
    path: String
}

unsafe impl Send for Router {}
unsafe impl Sync for Router {}

impl std::fmt::Debug for Router {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Router {{ routes: {:?}, num_routes: {} }}", self.routes, self.num_routes)
    }
}

impl Clone for Router {
    fn clone(&self) -> Self {
        Router {
            routes: self.routes.clone(),
            num_routes: self.num_routes,
            path: self.path.clone(),
        }
    }
}



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
            path: "".to_string()
        }
    }
    
    // Private method to create a new Route
    fn register_route(&mut self, method: HttpMethod, path: &str, action: Box<dyn for<'a> Fn(&'a Request, &'a mut Response) -> &'a mut Response + 'static>) {
        let method = method.to_string();
        let formated_path = format!("{}{}", self.path, path);
        println!("Registering route: {} {}", method, formated_path);
        let route = Route::new(&formated_path, action);
        self.routes.entry(method).or_insert(Vec::new()).push(route);
    }

    //Resolvers
    pub fn resolve_route(&self, req: &Request) -> Option<&Route> { 
        let method = req.get_method().to_string();
        let routes = self.routes.get(&method).unwrap();
        let route = routes.iter().find(|r| r.get_uri() == req.get_uri());
        match route {
            Some(r) => Some(r),
            None => None,
        }
    }

    //    
    //  This method will resolve the request and return a response, by checking
    //  the existance of the route in the routes HashMap of the Aplication server.        //
    //     -->  It will return an empty response if the route is not found.
    //
    pub fn resolve(&self, req: &Request) -> Response { 

        let route = self.resolve_route(req);
        match route {
            Some(r) => {
                let mut res = Response::empty();
                res.set_status(HttpStatus::Ok);
                let action = r.get_action();
                let action = action.lock().unwrap();
                action(req, &mut res);
                res
            },
            None => Response::empty(),
        }
    }

    pub fn mount(&mut self, path: &str) {
        self.path = path.to_string();
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }

    pub fn set_path(&mut self, path: &str) {
        self.path = path.to_string();
    }
    
    // PUBLIC API METHODS

    pub fn store_route<F>(&mut self, method: HttpMethod, path: &str, action: F) 
        where F: for<'a> Fn(&'a Request, &'a mut Response) -> &'a mut Response + 'static 
    {
        self.register_route(method, path, Box::new(action));
    }

    pub fn get<F>(&mut self, path: &str, action: F) 
        where F: for<'a> Fn(&'a Request, &'a mut Response) -> &'a mut Response + 'static 
    {
        self.register_route(HttpMethod::GET, path, Box::new(action));
    }

    pub fn post<F>(&mut self, path: &str, action: F) 
        where F: for<'a> Fn(&'a Request, &'a mut Response) -> &'a mut Response + 'static 
    {
        self.register_route(HttpMethod::POST, path, Box::new(action));
    }

    pub fn put<F>(&mut self, path: &str, action: F) 
        where F: for<'a> Fn(&'a Request, &'a mut Response) -> &'a mut Response + 'static 
    {
        self.register_route(HttpMethod::PUT, path, Box::new(action));
    }

    pub fn delete<F>(&mut self, path: &str, action: F) 
        where F: for<'a> Fn(&'a Request, &'a mut Response) -> &'a mut Response + 'static 
    {
        self.register_route(HttpMethod::DELETE, path, Box::new(action));
    }

    pub fn options<F>(&mut self, path: &str, action: F) 
        where F: for<'a> Fn(&'a Request, &'a mut Response) -> &'a mut Response + 'static 
    {
        self.register_route(HttpMethod::OPTIONS, path, Box::new(action));
    }

    pub fn patch<F>(&mut self, path: &str, action: F) 
        where F: for<'a> Fn(&'a Request, &'a mut Response) -> &'a mut Response + 'static 
    {
        self.register_route(HttpMethod::PATCH, path, Box::new(action));
    }

}


#[cfg(test)]
mod test_router {

    #[test]
    fn store_route() {
        use crate::http::methods::HttpMethod;
        use crate::http::request::Request;
        use crate::routing::router::Router;

        let mut router = Router::new();
        router.store_route(HttpMethod::GET, "/hello", |_req, res| {
            res
        });


        let req_str = "GET /hello HTTP/1.1\r\nHost: localhost:4221\r\nUser-Agent: curl/7.68.0\r\nAccept: */*\r\n\r\n";
        let req = Request::new(req_str).unwrap();
        let res = router.resolve(&req);
        assert_eq!(res.get_status().to_string(), "200 OK");
    }

    #[test]
    fn not_found() {
        use crate::http::request::Request;
        use crate::routing::router::Router;

        let router = Router::new();

        let req_str = "GET /world HTTP/1.1\r\nHost: localhost:4221\r\nUser-Agent: curl/7.68.0\r\nAccept: */*\r\n\r\n";
        let req = Request::new(req_str).unwrap();
        let res = router.resolve(&req);
        assert_eq!(res.get_status().to_string(), "404 Not Found");
    }

    #[test]
    fn resolve_ok() {
        use crate::http::request::Request;
        use crate::routing::router::Router;

        let mut router = Router::new();
        router.store_route(crate::http::methods::HttpMethod::GET, "/hello", |_req, res| {
            res
        });
        let req = Request::new("GET /hello HTTP/1.1\r\nHost: localhost:4221\r\nUser-Agent: curl/7.68.0\r\nAccept: */*\r\n\r\n").unwrap();
        let res = router.resolve(&req);
        assert_eq!(res.get_status().to_string(), "200 OK");
    }

    #[test]
    fn resolve_not_found() {
        use crate::http::request::Request;
        use crate::routing::router::Router;

        let router = Router::new();
        let req = Request::new("GET /world HTTP/1.1\r\nHost: localhost:4221\r\nUser-Agent: curl/7.68.0\r\nAccept: */*\r\n\r\n").unwrap();
        let res = router.resolve(&req);
        assert_eq!(res.get_status().to_string(), "404 Not Found");
    }



}
