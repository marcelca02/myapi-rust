// File contains the structures for handling request and response in the server api

use std::collections::HashMap;
use std::fmt;

use crate::http::status::HttpStatus;
use crate::utils::formatter;

// Response structure for handling response in the server api

pub struct Response {
    version: String,
    status: HttpStatus,
    headers: HashMap<String, String>,
    response_body: Option<Vec<u8>>,
}



impl Response {

    /// Set the response contnet type to plain text
    /// Takes a string as the body of the response
    ///
    /// # Example
    ///
    /// 
    /// let mut res = Response::empty();
    /// res.send("Hello, World!");
    /// 
    pub fn send(&mut self, body: &str) -> &mut Self {
        self.headers.insert("Content-Type".to_string(), "text/plain".to_string());
        self.set_body(body.into())
    }

    /// Set the response content type to JSON
    /// Takes a string as the body of the response
    ///
    /// # Example
    ///
    /// 
    /// let mut res = Response::empty();
    /// res.json(r#"{"message": "Hello, World!"}"#);
    /// 
    pub fn json(&mut self, body: &str) -> &mut Self {
        self.headers.insert("Content-Type".to_string(), "application/json".to_string());
        self.response_body = Some(body.into());
        self
    }

    /// Render an HTML template
    /// Takes a file path and a HashMap of parameters that will be replaced in the template
    ///
    /// This parameters must exist in the template file and be surrounded by double curly braces
    /// like this:  {{param}} ```
    ///
    /// # Example
    ///
    /// 
    /// let params = HashMap::new();
    /// params.insert("name".to_string(), "John".to_string());
    /// res.render_template("example.html", params);
    /// 
    pub fn render_template(&mut self, file_path: &str, params: HashMap<String, String>) -> &mut Self {
        self.headers.insert("Content-Type".to_string(), "text/html".to_string());

        match formatter::format_html(file_path, params) {
            Ok(body) => {
                self.response_body = Some(body.into());
                return self;
            },
            Err(e) => {
                self.status = HttpStatus::InternalServerError;
                println!("Error: {}", e);
                return self;
            }
        }
    }

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

    pub fn get_body(&self) -> Option<&Vec<u8>> {
        self.response_body.as_ref()
    }

    pub fn set_status(&mut self, status: HttpStatus) {
        self.status = status;
    }

    pub fn set_body(&mut self, body: Vec<u8>) -> &mut Self {
        self.response_body = Some(body);
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




