// Route structure for handling routes in the server api
use std::sync::{Arc, Mutex};

use crate::http::comm::{Request, Response};

pub struct Route {
    uri: String,
    action: Arc<Mutex<Box<dyn for<'a> Fn(&'a Request, &'a mut Response) -> &'a mut Response>>>,
    parameters: Vec<String>,
    // TODO: Implement regex for uri and middlewares
}

unsafe impl Send for Route {}
unsafe impl Sync for Route {}

impl std::fmt::Debug for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Route {{ uri: {}, parameters: {:?} }}", self.uri, self.parameters)
    }
}

impl Clone for Route {
    fn clone(&self) -> Self {
        Route {
            uri: self.uri.clone(),
            action: self.action.clone(),
            parameters: self.parameters.clone(),
        }
    }
}

#[allow(dead_code)]
impl Route {
    pub fn new(uri: &str, action: Box<dyn for<'a> Fn(&'a Request, &'a mut Response) -> &'a mut Response>) -> Self {
        Route {
            uri: uri.to_string(),
            action: Arc::new(Mutex::new(action)),
            parameters: Vec::new(),
            // regex: Regex::new(uri).unwrap(),
            // middlewares: Vec::new(),
        }
    }

    pub fn get_uri(&self) -> &str {
        &self.uri
    }

    pub fn get_action(&self) -> Arc<Mutex<Box<dyn for<'a> Fn(&'a Request, &'a mut Response) -> &'a mut Response>>> {
        self.action.clone()
    }

    pub fn has_parameters(&self) -> bool {
        !self.parameters.is_empty()
    }

    pub fn get_parameters(&self) -> &Vec<String> {
        &self.parameters
    }
}
