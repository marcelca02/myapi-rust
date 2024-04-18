use std::net::TcpListener;
use std::io::Read;

use crate::routing::router::Router;
use crate::config;
use crate::routing::handler::Handler;
use crate::utils::parsers;

pub struct App {
    host: String,
    port: u16,
    router: Router,
}

#[allow(dead_code)]
impl App {
    pub fn new() -> Self {
        App {
            host: config::HOST.to_string(),
            port: config::PORT,
            router: Router::new()
        }
    }

    pub fn run(&mut self, host: String, port: u16) {
        self.host = host;
        self.port = port; 

        // Create a new TcpListener and bind it to the HOST and PORT
        let listener = TcpListener::bind(format!("{}:{}", config::HOST, config::PORT)).unwrap();
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
    pub fn get(&mut self, path: &str, action: Box<dyn Handler>) {
        self.router.get(path, action);
    }

    // Public method to create a new POST Route
    pub fn post(&mut self, path: &str, action: Box<dyn Handler>) {
        self.router.post(path, action);
    }

    // Public method to create a new PUT Route
    pub fn put(&mut self, path: &str, action: Box<dyn Handler>) {
        self.router.put(path, action);
    }
    
    // Public method to create a new DELETE Route
    pub fn delete(&mut self, path: &str, action: Box<dyn Handler>) {
        self.router.delete(path, action);
    }

    // Public method to create a new OPTIONS Route
    pub fn options(&mut self, path: &str, action: Box<dyn Handler>) {
        self.router.options(path, action);
    }

    // Public method to create a new PATCH Route
    pub fn patch(&mut self, path: &str, action: Box<dyn Handler>) {
        self.router.patch(path, action);
    }
}
