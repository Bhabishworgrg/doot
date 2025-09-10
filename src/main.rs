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
                println!("{}", String::from_utf8_lossy(&request));

                let response: &str = "HTTP/1.1 200 OK\r\n\r\n";
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
