
use std::collections::HashMap;

use crate::http::methods::HttpMethod; 
use crate::utils::parsers;

// Request structure for handling request in the server api

#[derive(Clone)]
pub struct Request {
    version: String,
    uri: String,
    method: HttpMethod,
    body_data: Option<Vec<u8>>,
    headers: HashMap<String, u8>
    //TODO: parameters and files
}

impl std::fmt::Debug for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ {}\r\n{}\r\n{:}\r\n{:?}\r\n}}", self.version, self.uri, self.method, self.headers)
    }
}

impl std::fmt::Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}\r\n{:?}\r\n{:?}", self.method, self.uri, self.headers, self.body_data)
    }
}


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

    pub fn new(request: &str) -> Result<Self, std::io::Error> {
        let mut req = Request::empty();
        match parsers::parse_request(request.to_string(), &mut req) {
            Ok(_) => Ok(req),
            Err(err) => Err(err)
        }
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
