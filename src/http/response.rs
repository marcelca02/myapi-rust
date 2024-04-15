// Response structure for handling response in the server api

use std::collections::HashMap;

#[allow(dead_code)]
pub struct Response {
    status: u16,
    headers: HashMap<String, String>,
    response_body: Vec<u8>,
}

#[allow(dead_code)]
impl Response {
    
    pub fn new(&mut self, status_code: u16, headers: HashMap<String, String>, body: Vec<u8>) -> Self {
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
