extern crate tokio;

use tokio::net::{TcpListener, TcpStream};
use std::net::SocketAddr;
use std::sync::Arc;

use crate::http::methods::HttpMethod;
use crate::routing::router::Router;
use crate::config;
use crate::http::{request::Request, response::Response};
use crate::http::status::HttpStatus;

pub struct App {
    address: SocketAddr,
    router: Router,
}

impl Clone for App {
    fn clone(&self) -> Self {
        App {
            address: self.address,
            router: self.router.clone(),
        }
    }
}

#[allow(dead_code)]
impl App {

    /// Method to run the server
    /// This method listens for incoming requests and spawns a new thread to handle each request
    ///
    /// # Example
    ///
    /// 
    /// let mut app = App::new("127.0.0.1", 8080);
    /// app.run().await;
    /// 
    pub async fn run(&mut self) {

        // Create a new TcpListener and bind it to the HOST and PORT
        let listener = TcpListener::bind(&self.address).await.expect("Failed to bind to address");
        println!("Listening on {}:{}", config::HOST, config::PORT);

        let arc = Arc::new(self.router.clone());

        // Event loop for receiving requests
        loop {
            let (stream, _) = listener.accept().await.expect("Failed to accept connection");
            let cloned = Arc::clone(&arc);
                
            // Spawn a new thread to handle the connection
            tokio::spawn(async move {
                App::handle_connection(cloned, stream);
            });
        }
    }


    /// Constructor for the App structure
    /// Takes the address and port as arguments
    ///
    /// # Example
    ///
    /// 
    /// let mut app = App::new("127.0.0.1", 8080);
    /// 
    pub fn new(address: &str, port: u16) -> Self {
        App {
            address: format!("{}:{}", address, port).parse().unwrap(),
            router: Router::new()
        }
    }

    fn handle_connection(cloned: Arc<Router>, stream: TcpStream) {
        let mut buffer = [0; 1024];

        // Try reading from the stream until it is successful
        while !stream.try_read(&mut buffer).is_ok() {}

        let request = String::from_utf8_lossy(&buffer[..]);

        // Create a new Request object
        let req = Request::new(&request);

        let mut res = Response::empty();
        
        match &req {
            Ok(req) => {
                // Resolve the request
                res = cloned.resolve(&req);
            },
            Err(_) => {
                // Set the status to BadRequest if the request is invalid
                res.set_status(HttpStatus::BadRequest)
            }
        }

        // Try writing to the stream until it is successful
        while !stream.try_write(res.to_string().as_bytes()).is_ok() {}

        // Print debug information
        println!("[{:} {:}] Number of bytes: {:?}",res.get_version(), res.get_status().to_string(), req.expect("0").to_string().len());

    }

    pub fn stop(&self) {

        

    }

    /// Method to store a GET route in the server
    /// Takes the path and a closure as arguments.
    /// The clousre should take a Request and a mutable Response as arguments and return a mutable Response
    ///
    /// This clousre will be called when the route is resolved
    ///
    /// # Example
    /// 
    /// app.get("/", |req, res| {
    /// res
    /// });
    /// 
    pub fn get<F>(&mut self, path: &str, action: F) 
        where F: for<'a> Fn(&'a Request, &'a mut Response) -> &'a mut Response + 'static 
    {
        self.router.store_route(HttpMethod::GET, path, action)
    }

    /// Method to store a POST route in the server
    /// Takes the path and a closure as arguments.
    /// The clousre should take a Request and a mutable Response as arguments and return a mutable Response
    ///
    /// This clousre will be called when the route is resolved
    ///
    /// # Example
    /// 
    /// app.post("/", |req, res| {
    /// res
    /// });
    /// 
    pub fn post<F>(&mut self, path: &str, action: F) 
        where F: for<'a> Fn(&'a Request, &'a mut Response) -> &'a mut Response + 'static 
    {
        self.router.store_route(HttpMethod::POST, path, action);
    }

    /// Method to store a PUT route in the server
    /// Takes the path and a closure as arguments.
    /// The clousre should take a Request and a mutable Response as arguments and return a mutable Response
    ///
    /// This clousre will be called when the route is resolved
    ///
    /// # Example
    /// 
    /// app.put("/", |req, res| {
    /// res
    /// });
    /// 
    pub fn put<F>(&mut self, path: &str, action: F) 
        where F: for<'a> Fn(&'a Request, &'a mut Response) -> &'a mut Response + 'static 
    {
        self.router.store_route(HttpMethod::PUT, path, action);
    }

    /// Method to store a DELETE route in the server
    /// Takes the path and a closure as arguments.
    /// The clousre should take a Request and a mutable Response as arguments and return a mutable Response
    ///
    /// This clousre will be called when the route is resolved
    ///
    /// # Example
    /// 
    /// app.delete("/", |req, res| {
    /// res
    /// });
    /// 
    pub fn delete<F>(&mut self, path: &str, action: F) 
        where F: for<'a> Fn(&'a Request, &'a mut Response) -> &'a mut Response + 'static 
    {
        self.router.store_route(HttpMethod::DELETE, path, action);
    }

    /// Method to store a OPTIONS route in the server
    /// Takes the path and a closure as arguments.
    /// The clousre should take a Request and a mutable Response as arguments and return a mutable Response
    ///
    /// This clousre will be called when the route is resolved
    ///
    /// # Example
    /// 
    /// app.options("/", |req, res| {
    /// res
    /// });
    /// 
    pub fn options<F>(&mut self, path: &str, action: F) 
        where F: for<'a> Fn(&'a Request, &'a mut Response) -> &'a mut Response + 'static 
    {
        self.router.store_route(HttpMethod::OPTIONS, path, action);
    }

    /// Method to store a PATCH route in the server
    /// Takes the path and a closure as arguments.
    /// The clousre should take a Request and a mutable Response as arguments and return a mutable Response
    ///
    /// This clousre will be called when the route is resolved
    ///
    /// # Example
    /// 
    /// app.patch("/", |req, res| {
    /// res
    /// });
    /// 
    pub fn patch<F>(&mut self, path: &str, action: F) 
        where F: for<'a> Fn(&'a Request, &'a mut Response) -> &'a mut Response + 'static 
    {
        self.router.store_route(HttpMethod::PATCH, path, action);
    }


}
