use std::fmt::{self, Debug};

#[derive(Debug)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    OPTIONS,
    PATCH,
}

impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
    
