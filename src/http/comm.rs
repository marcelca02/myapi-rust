// File contains the structures for handling request and response in the server api

use std::collections::HashMap;
use std::fmt;

use crate::http::{methods::HttpMethod, status::HttpStatus}; 
use crate::utils::parsers;

// Request structure for handling request in the server api

#[allow(dead_code)]
pub struct Request {
    version: String,
    uri: String,
    method: HttpMethod,
    body_data: Option<Vec<u8>>,
    headers: HashMap<String, u8>
    //TODO: parameters and files
}

// Response structure for handling response in the server api

#[allow(dead_code)]
pub struct Response {
    version: String,
    status: HttpStatus,
    headers: HashMap<String, u8>,
    response_body: Option<Vec<u8>>,
}

#[allow(dead_code)]
impl Request {

    pub fn empty() -> Self {
        Request {
            version: "HTTP/1.1".to_string(),
            uri: "".to_string(),
            method: HttpMethod::GET,
            body_data: None,
            headers: HashMap::new()
        }
    }

    pub fn create(uri: &str,  method: HttpMethod, data: Vec<u8>, headers: HashMap<String, u8>) -> Self {
        Request {
            version: "HTTP/1.1".to_string(),
            uri: uri.to_string(), 
            method,
            body_data: Some(data),
            headers
        }
    }

    pub fn new(request: &str) -> Self {
        let mut req = Request::empty();
        parsers::parse_request(request.to_string(), &mut req).unwrap();
        req
    }

    pub fn set_version(&mut self, version: &str) {
        self.version = version.to_string();
    }


    pub fn get_uri(&self) -> &str {
        &self.uri
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

    pub fn get_body(&self) -> Option<&Vec<u8>> {
        self.body_data.as_ref()
    }

    pub fn set_body(&mut self, data: Vec<u8>) {
        self.body_data = Some(data);
    }

    pub fn set_headers(&mut self, headers: HashMap<String, u8>) {
        self.headers = headers;
    }

    pub fn set_method(&mut self, method: HttpMethod) {
        self.method = method;
    }

    pub fn set_uri(&mut self, uri: &str) {
        self.uri = uri.to_string();
    }

    pub fn set_header_field(&mut self, key: &str, value: u8) {
        self.headers.insert(key.to_string(), value);
    }
}


#[allow(dead_code)]
impl Response {

    // Empty constructor
    pub fn empty() -> Self {
        Response {
            version: "HTTP/1.1".to_string(),
            status: HttpStatus::Ok,
            headers: HashMap::new(),
            response_body: Some(Vec::new())
        }
    }
    
    pub fn new(&mut self, status_code: HttpStatus, headers: HashMap<String, u8>, body: Vec<u8>) -> Self {
        Response {
            version: "HTTP/1.1".to_string(),
            status: status_code,
            headers, 
            response_body: Some(body)
        }
    }

    pub fn get_status(&self) -> &HttpStatus {
        &self.status
    }

    pub fn set_status(&mut self, status: HttpStatus) {
        self.status = status;
    }

    pub fn set_body(&mut self, body: Vec<u8>) {
        self.response_body = Some(body);
    }

}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}\r\n",self.version, self.status)?;
        for (key, value) in &self.headers {
            write!(f, "{}: {}\r\n", key, value)?;
        }
        write!(f, "\r\n")?;
        match &self.response_body {
            Some(body) => {
                let body = String::from_utf8_lossy(&body);
                write!(f, "{}", body)
            },
            None => write!(f, "")
        }
    }
}

