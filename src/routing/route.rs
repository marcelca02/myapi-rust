// Route structure for handling routes in the server api
use std::sync::{Arc, Mutex};
use regex::Regex;

use crate::http::{request::Request, response::Response};
use crate::utils::formatter;

pub struct Route {
    uri: String,
    action: Arc<Mutex<Box<dyn for<'a> Fn(&'a Request, &'a mut Response) -> &'a mut Response>>>,
    parameters: Vec<String>,
    regex: Regex,
}


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
            regex: self.regex.clone(),
        }
    }
}

impl Route {
    pub fn new(uri: &str, action: Box<dyn for<'a> Fn(&'a Request, &'a mut Response) -> &'a mut Response>) -> Self {
        let mut params = Vec::new();
        let regex_str = formatter::format_regex(uri, &mut params);
        let regex = Regex::new(&regex_str).expect("Error creating regex");
        Route {
            uri: uri.to_string(),
            action: Arc::new(Mutex::new(action)),
            parameters: params,
            regex,
            // middlewares: Vec::new(),
        }
    }

    pub fn get_uri(&self) -> &str {
        &self.uri
    }

    pub fn get_regex(&self) -> &Regex {
        &self.regex
    }

    pub fn get_action(&self) -> Arc<Mutex<Box<dyn for<'a> Fn(&'a Request, &'a mut Response) -> &'a mut Response>>> {
        self.action.clone()
    }

    #[allow(dead_code)]
    pub fn has_parameters(&self) -> bool {
        !self.parameters.is_empty()
    }

    #[allow(dead_code)]
    pub fn get_parameters(&self) -> &Vec<String> {
        &self.parameters
    }
}
