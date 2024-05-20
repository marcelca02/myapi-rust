extern crate httparse;
extern crate regex;

use std::io::Error;
use std::collections::HashMap;

use crate::http::methods::HttpMethod;
use crate::http::request::Request;

#[allow(dead_code)]
pub fn parse_request<'a>(request: String, my_req: &'a mut Request) -> Result<(), Error> {

    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = httparse::Request::new(&mut headers);
    let res = req.parse(request.as_bytes());

    // Check if the request is valid
    match res {
        Ok(_) => {
            let method = HttpMethod::from_str(req.method.unwrap());
            let path = req.path.unwrap();
            let mut headers = HashMap::new();
            // Insert headers into a HashMap
            for header in &mut *req.headers {
                headers.insert(header.name.to_string(), header.value.len() as u8);
            }

            // Get the body of the request
            let body_start = req.headers.len() + 1;
            let body = &request.split("\r\n").nth(body_start).unwrap_or_else(|| "Error reading body");
            my_req.set_body(body.as_bytes().to_vec());

            
            // Set the request data
            my_req.set_uri(path);
            my_req.set_method(method);
            my_req.set_headers(headers);
           

            Ok(())        
        },
        Err(err) => Err(Error::new(std::io::ErrorKind::Other, err))
    }
}
