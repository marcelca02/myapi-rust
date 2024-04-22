use std::net::TcpListener;
use std::net::SocketAddr;
use std::io::Read;

use crate::routing::router::Router;
use crate::config;
use crate::utils::parsers;
use crate::http::comm::{Request, Response};

pub struct App {
    address: SocketAddr,
    router: Router,
}

#[allow(dead_code)]
impl App {
    pub fn new(address: &str, port: u16) -> Self {
        App {
            address: format!("{}:{}", address, port).parse().unwrap(),
            router: Router::new()
        }
    }

    pub fn run(&mut self) {

        // Create a new TcpListener and bind it to the HOST and PORT
        let listener = TcpListener::bind(self.address).unwrap();
        println!("Listening on {}:{}", config::HOST, config::PORT);

        // Event loop for receiving requests
        for stream in listener.incoming() {
            match stream {
                // New connection
                Ok(mut _stream) => {
                    println!("Accepted new connection, reading request...");

                    let mut req = [0u8; 1024];
                    _stream.read(&mut req).unwrap();
                    let req = String::from_utf8_lossy(&req);

                    let(method, path, _headers) = parsers::parse_request(req.to_string()).unwrap(); 
                    // let _form = utils::parsers::parse_form(headers).unwrap();

                    println!("Method: {}", method);
                    println!("Path: {}", path);
                    // println!("Form: {:?}", form);

                },
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
    }

    // Public method to create a new GET Route
    pub fn get<F>(&mut self, path: &str, action: F) 
        where F: Fn(&Request, &mut Response) -> Response + 'static
    {
        self.router.get(path, action);
    }

    // Public method to create a new POST Route
    pub fn post<F>(&mut self, path: &str, action: F) 
        where F: Fn(&Request, &mut Response) -> Response + 'static
    {
        self.router.post(path, action);
    }

    // Public method to create a new PUT Route
    pub fn put<F>(&mut self, path: &str, action: F) 
        where F: Fn(&Request, &mut Response) -> Response + 'static
    {
        self.router.put(path, action);
    }

    // Public method to create a new DELETE Route
    pub fn delete<F>(&mut self, path: &str, action: F) 
        where F: Fn(&Request, &mut Response) -> Response + 'static
    {
        self.router.delete(path, action);
    }

    // Public method to create a new OPTIONS Route
    pub fn options<F>(&mut self, path: &str, action: F) 
        where F: Fn(&Request, &mut Response) -> Response + 'static
    {
        self.router.options(path, action);
    }

    // Public method to create a new PATCH Route
    pub fn patch<F>(&mut self, path: &str, action: F) 
        where F: Fn(&Request, &mut Response) -> Response + 'static
    {
        self.router.patch(path, action);
    }
}
