// File contains the structures for handling request and response in the server api

use std::collections::HashMap;
use std::fmt;

use crate::http::status::HttpStatus;

// Response structure for handling response in the server api

#[allow(dead_code)]
pub struct Response {
    version: String,
    status: HttpStatus,
    headers: HashMap<String, String>,
    response_body: Option<Vec<u8>>,
}



#[allow(dead_code)]
impl Response {

    // Empty constructor
    pub fn empty() -> Self {
        Response {
            version: "HTTP/1.1".to_string(),
            status: HttpStatus::NotFound,
            headers: HashMap::new(),
            response_body: Some(Vec::new())
        }
    }
    
    pub fn new(&mut self, status_code: HttpStatus, headers: HashMap<String, String>, body: Vec<u8>) -> Self {
        Response {
            version: "HTTP/1.1".to_string(),
            status: status_code,
            headers,
            response_body: Some(body)
        }
    }

    pub fn get_version(&self) -> &str {
        &self.version
    }

    pub fn get_status(&self) -> &HttpStatus {
        &self.status
    }

    pub fn set_status(&mut self, status: HttpStatus) {
        self.status = status;
    }

    pub fn set_body(&mut self, body: Vec<u8>) -> &mut Self {
        self.response_body = Some(body);
        self
    }

    /// Set the response contnet type to plain text
    /// Takes a string as the body of the response
    ///
    /// # Example
    ///
    /// ```
    /// let mut res = Response::empty();
    /// res.send("Hello, World!");
    /// ```
    pub fn send(&mut self, body: &str) -> &mut Self {
        self.headers.insert("Content-Type".to_string(), "text/plain".to_string());
        self.set_body(body.into())
    }

    /// Set the response content type to JSON
    /// Takes a string as the body of the response
    ///
    /// # Example
    ///
    /// ```
    /// let mut res = Response::empty();
    /// res.json(r#"{"message": "Hello, World!"}"#);
    /// ```
    pub fn json(&mut self, body: &str) -> &mut Self {
        self.headers.insert("Content-Type".to_string(), "application/json".to_string());
        self.response_body = Some(body.into());
        self
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

