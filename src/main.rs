use std::net::TcpListener;
use std::io::Write;
use std::io::Read;

fn main() {
    // Uncomment this block to pass the first stage
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("accepted new connection");
                let mut buf = [0u8; 1024];
                _stream
                    .read(&mut buf).unwrap();
                let res = String::from_utf8(buf.to_vec()).unwrap();
                println!("request received: {}", res);
                let first_line = res.split("\r\n").next().expect("no first line");
                let path = first_line.split(" ").nth(1).unwrap();
                println!("path: {}", path);
                let response;
                // Check path
                match path {
                    "/" => {
                        response = "HTTP/1.1 200 OK\r\n\r\n";
                        _stream
                            .write(response.as_bytes())
                            .expect("failed to write OK response");
                    },
                    _ => {
                        response = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
                        _stream
                            .write(response.as_bytes())
                            .expect("failed to write NOT FOUND response");
                    }
                }

            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
