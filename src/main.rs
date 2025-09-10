use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let listener: TcpListener = match TcpListener::bind("127.0.0.1:6969") {
        Ok(listener) => listener,
        Err(_) => {
            println!("Failed to create a TCP listener");
            return;
        }
    };

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Accepted a new connection");

                let mut request: [u8; 8192] = [0; 8192];
                match stream.read(&mut request) {
                    Ok(_) => println!("Reading request..."),
                    Err(_) => println!("Failed to read request")
                };

                let request: String = String::from_utf8_lossy(&request).to_string();
                println!("{}", request);

                let request: Vec<&str> = request.split_whitespace().collect::<Vec<&str>>();
                let url_path: &str = request.get(1).unwrap();

                let mut response: String = String::from("HTTP/1.1 ");
                if url_path == "/" {
                    response.push_str("200 OK\r\n\r\n");
                } else {
                    response.push_str("404 Not Found\r\n\r\n");
                }

                match stream.write(response.as_bytes()) {
                    Ok(bytes) => bytes,
                    Err(_) => {
                        println!("Failed to response to request");
                        return;
                    }
                };
            },
            Err(_) => println!("Failed to accept a new connection")
        }
    }
}
