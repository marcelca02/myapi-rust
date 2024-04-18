extern crate httparse;

use std::io::Error;
use std::collections::HashMap;

use crate::http::methods::HttpMethod;

#[allow(dead_code)]
pub fn parse_request(_req: String) -> Result<(HttpMethod, String, HashMap<String,u8>), Error> {
    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = httparse::Request::new(&mut headers);
    let res = req.parse(_req.as_bytes());
    match res {
        Ok(_) => {
            let method = HttpMethod::from_str(req.method.unwrap());
            let path = req.path.unwrap();
            let mut headers = HashMap::new();
            for header in req.headers {
                headers.insert(header.name.to_string(), header.value.len() as u8);
            }
            Ok((method, path.to_string(), headers))
            
        },
        Err(_) => Err(Error::new(std::io::ErrorKind::Other, "Invalid request"))
    }

}

#[allow(dead_code)]
pub fn parse_form(_headers: HashMap<String, u8>) -> Result<Vec<(String, String)>, Error> {
    Ok(vec![("".to_string(), "".to_string())])
}

#[allow(dead_code)]
fn get_path(req: String) -> Result<String, Error> {
    let first_line = req.split("\r\n").nth(0).unwrap();
    let path = first_line.split(" ").nth(1).unwrap();
    match path.chars().nth(0) {
        Some('/') => {
            if path.len() > 1 {
                Ok(path.to_string())
            } else {
                Err(Error::new(std::io::ErrorKind::Other, "empty path"))
            }
        },
        _ => Err(Error::new(std::io::ErrorKind::Other, "invalid path"))
    }
}

