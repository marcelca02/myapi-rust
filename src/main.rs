use std::net::TcpListener;
use std::io::Write;

fn main() {
    // Uncomment this block to pass the first stage
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("accepted new connection");
                let response = "HTTP/1.1 200 OK\r\n\r\n";
                _stream
                    .write_all(response.as_bytes())
                    .expect("failed to write to tcp stream");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
