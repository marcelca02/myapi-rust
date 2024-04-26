// Router structure for handling routes in the server api

use std::collections::HashMap;

use crate::routing::route::Route;
use crate::http::{methods::HttpMethod,request::Request,response::Response, status::HttpStatus};

// TODO: Implement my own HashMap
pub struct Router {
    routes: HashMap<String, Vec<Route>>,
    num_routes: usize,
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
        }
    }
    
    // Private method to create a new Route
    fn register_route(&mut self, method: HttpMethod, path: &str, action: Box<dyn for<'a> Fn(&'a Request, &'a mut Response) -> &'a mut Response + 'static>) {
        let method = method.to_string();
        let route = Route::new(path, action);
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
    
    // PUBLIC API METHODS

    pub fn store_route<F>(&mut self, method: HttpMethod, path: &str, action: F) 
        where F: for<'a> Fn(&'a Request, &'a mut Response) -> &'a mut Response + 'static 
    {
        self.register_route(method, path, Box::new(action));
    }

}
