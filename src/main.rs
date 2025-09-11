mod constants;

use std::io::{Read, Write};
use std::net::TcpListener;
use std::process::exit;

use constants::config;
use constants::status_code;

fn main() {
    let listener: TcpListener = TcpListener::bind((config::HOST, config::PORT))
        .unwrap_or_else(|_| {
            eprintln!("Failed to create a TCP listener");
            exit(1);
        }
    );

    for incoming in listener.incoming() {
        match incoming {
            Ok(mut stream) => {
                println!("Accepted a new connection");

                let mut buffer: [u8; config::MAX_REQ_SIZE] = [0; config::MAX_REQ_SIZE];
                match stream.read(&mut buffer) {
                    Ok(_) => println!("Reading request..."),
                    Err(_) => {
                        eprintln!("Failed to read request");
                        continue;
                    }
                };

                let request: String = String::from_utf8_lossy(&buffer).to_string();
                println!("{request}");

                let request: Vec<&str> = request.split_whitespace().collect();
                let resource_path: &str = request.get(1).unwrap();

                let mut response: String = String::from("HTTP/1.1 ");
                if resource_path == "/" {
                    response.push_str(status_code::OK);
                } else {
                    response.push_str(status_code::NOT_FOUND);
                }
                response.push_str("\r\n\r\n");

                if let Err(_) = stream.write(response.as_bytes()) {
                    eprintln!("Failed to respond to the request");
                    continue;
                };
            },
            Err(_) => eprintln!("Failed to accept a new connection")
        }
    }
}
