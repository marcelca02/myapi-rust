use std::net::TcpListener;
use std::io::Read;

mod parsers;
mod config;


fn main() {
    // Create a new TcpListener and bind it to the HOST and PORT
    let listener = TcpListener::bind(format!("{}:{}", config::HOST, config::PORT)).unwrap();
    println!("Listening on {}:{}", config::HOST, config::PORT);

    for stream in listener.incoming() {
        match stream {
            // New connection
            Ok(mut _stream) => {
                println!("Accepted new connection, reading request...");

                let mut req = [0u8; 1024];
                _stream.read(&mut req).unwrap();
                let req = String::from_utf8(req.to_vec()).unwrap();

                println!("Request received: {}", req);

                let(method, path, headers) = parsers::http::parse_request(req).unwrap();
                let form = parsers::http::parse_form(headers).unwrap();

                println!("Method: {}", method);
                println!("Path: {}", path);
                println!("Form: {:?}", form);

            },
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
