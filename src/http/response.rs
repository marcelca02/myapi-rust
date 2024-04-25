// File contains the structures for handling request and response in the server api

use std::collections::HashMap;
use std::fmt;

use crate::http::status::HttpStatus;

// Response structure for handling response in the server api

#[allow(dead_code)]
pub struct Response {
    version: String,
    status: HttpStatus,
    headers: HashMap<String, u8>,
    response_body: Option<Vec<u8>>,
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

