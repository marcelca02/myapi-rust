// Request structure for handling request in the server api

use std::collections::HashMap;

use crate::http::methods::HttpMethod;
use crate::routing::route::Route;

#[allow(dead_code)]
pub struct Request {
    uri: String,
    route: Route,
    method: HttpMethod,
    body_data: Vec<u8>,
    headers: HashMap<String, String>
    //TODO: parameters and files
}

#[allow(dead_code)]
impl Request {
    pub fn new(uri: &str, route: Route, method: HttpMethod, data: Vec<u8>, headers: HashMap<String, String>) -> Self {
        Request {
            uri: uri.to_string(), 
            route, 
            method,
            body_data: data,
            headers
        }
    }

    pub fn get_header_field(&self, key: &str) -> Option<&str> {
        if key.is_empty() {
            return None;
        }
        self.headers.get(key).map(|v| v.as_str())
    }
}
