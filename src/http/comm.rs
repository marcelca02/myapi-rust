// File contains the structures for handling request and response in the server api

// Request structure for handling request in the server api

use std::collections::HashMap;

use crate::http::methods::HttpMethod;
use crate::routing::route::Route;

#[allow(dead_code)]
pub struct Request {
    uri: String,
    route: Route,
    method: HttpMethod,
    body_data: Option<Vec<u8>>,
    headers: HashMap<String, u8>
    //TODO: parameters and files
}

#[allow(dead_code)]
impl Request {
    pub fn new(uri: &str, route: Route, method: HttpMethod, data: Vec<u8>, headers: HashMap<String, u8>) -> Self {
        Request {
            uri: uri.to_string(), 
            route, 
            method,
            body_data: Some(data),
            headers
        }
    }

    pub fn get_uri(&self) -> &str {
        &self.uri
    }

    pub fn get_route(&self) -> &Route {
        &self.route
    }

    pub fn get_method(&self) -> &HttpMethod {
        &self.method
    }

    pub fn get_header_field(&self, key: &str) -> Option<&u8> {
        if key.is_empty() {
            return None;
        }
        self.headers.get(key)
    }
}

// Response structure for handling response in the server api

#[allow(dead_code)]
pub struct Response {
    status: u16,
    headers: HashMap<String, u8>,
    response_body: Vec<u8>,
}

#[allow(dead_code)]
impl Response {

    // Empty constructor
    pub fn empty() -> Self {
        Response {
            status: 404,
            headers: HashMap::new(),
            response_body: Vec::new()
        }
    }
    
    pub fn new(&mut self, status_code: u16, headers: HashMap<String, u8>, body: Vec<u8>) -> Self {
        Response {
            status: status_code,
            headers, 
            response_body: body
        }
    }

    pub fn get_status(&self) -> u16 {
        self.status
    }

}
