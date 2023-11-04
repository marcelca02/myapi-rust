use std::net::TcpListener;
use std::io::Write;
use std::io::Read;

mod parsers;

fn main() {
    // Uncomment this block to pass the first stage
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    println!("listening to 127.0.0.1 4221");

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("accepted new connection");
                let mut req = [0u8; 1024];
                _stream
                    .read(&mut req)
                    .unwrap();
                let req = String::from_utf8(req.to_vec()).unwrap();
                let res;
                println!("request received: {}", req);

                // Check path
                match req.as_str() {
                    "" => {
                        println!("empty request");
                        res = "HTTP/1.1 404 NOT FOUND\r\n\r\n".to_string();
                        _stream
                            .write(res.as_bytes())
                            .expect("failed to write OK response");
                    },
                    _ => {
                        let path = parsers::http::get_path(req).unwrap();
                        println!("path: {}", path);
                    }                    
                }
            },
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
