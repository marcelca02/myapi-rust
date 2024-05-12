use std::fmt;
use std::cmp;

#[derive(Debug)]
pub enum HttpStatus {
    Ok,
    BadRequest,
    NotFound,
    InternalServerError,
}

impl HttpStatus{
    fn as_str(&self) -> &'static str {
        match self {
            HttpStatus::Ok => "200 OK",
            HttpStatus::BadRequest => "400 Bad Request",
            HttpStatus::NotFound => "404 Not Found",
            HttpStatus::InternalServerError => "500 Internal Server Error",
        }
    }
}

impl fmt::Display for HttpStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl cmp::PartialEq for HttpStatus {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
