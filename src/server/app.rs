extern crate tokio;

use tokio::net::{TcpListener, TcpStream};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::thread::current;

use crate::routing::router::Router;
use crate::config;
use crate::http::comm::{Request, Response};

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

    pub async fn run(&mut self) {

        // Create a new TcpListener and bind it to the HOST and PORT
        let listener = TcpListener::bind(&self.address).await.expect("Failed to bind to address");
        println!("Listening on {}:{}", config::HOST, config::PORT);

        let arc = Arc::new(Mutex::new(self.clone()));

        // Event loop for receiving requests
        loop {
            let (socket, _) = listener.accept().await.expect("Failed to accept connection");
            let cloned = Arc::clone(&arc);
                
            // Spawn a new thread to handle the connection
            tokio::spawn(async move {
                let app = cloned.lock().unwrap();
                app.handle_connection(socket);
            });
        }
    }


    pub fn new(address: &str, port: u16) -> Self {
        App {
            address: format!("{}:{}", address, port).parse().unwrap(),
            router: Router::new()
        }
    }

    fn handle_connection(&self, stream: TcpStream) {

        let mut buffer = [0; 1024];

        while !stream.try_read(&mut buffer).is_ok() {}
        
        let request = String::from_utf8_lossy(&buffer[..]);

        // Create a new Request object
        let req = Request::new(&request);

        // Create a new Response object
        let res = self.router.resolve(&req);

        println!("{}", res);
    }

    // Public method to create a new GET Route
    pub fn get<F>(&mut self, path: &str, action: F) 
        where F: for<'a> Fn(&'a Request, &'a mut Response) -> &'a mut Response + 'static 
    {
        self.router.get(path, action);
    }

    // Public method to create a new POST Route
    pub fn post<F>(&mut self, path: &str, action: F) 
        where F: for<'a> Fn(&'a Request, &'a mut Response) -> &'a mut Response + 'static 
    {
        self.router.post(path, action);
    }

    // Public method to create a new PUT Route
    pub fn put<F>(&mut self, path: &str, action: F) 
        where F: for<'a> Fn(&'a Request, &'a mut Response) -> &'a mut Response + 'static 
    {
        self.router.put(path, action);
    }

    // Public method to create a new DELETE Route
    pub fn delete<F>(&mut self, path: &str, action: F) 
        where F: for<'a> Fn(&'a Request, &'a mut Response) -> &'a mut Response + 'static 
    {
        self.router.delete(path, action);
    }

    // Public method to create a new OPTIONS Route
    pub fn options<F>(&mut self, path: &str, action: F) 
        where F: for<'a> Fn(&'a Request, &'a mut Response) -> &'a mut Response + 'static 
    {
        self.router.options(path, action);
    }

    // Public method to create a new PATCH Route
    pub fn patch<F>(&mut self, path: &str, action: F) 
        where F: for<'a> Fn(&'a Request, &'a mut Response) -> &'a mut Response + 'static 
    {
        self.router.patch(path, action);
    }
}
