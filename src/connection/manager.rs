use std::io::{Read, Write};
use std::net::TcpStream;

use crate::constants::{config, status_code};

pub fn handle_connection(mut stream: TcpStream) {
    println!("Reading a request...");

    let mut buffer: [u8; config::MAX_REQ_SIZE] = [0; config::MAX_REQ_SIZE];

    if let Err(_) = stream.read(&mut buffer) {
        eprintln!("Failed to read the request.");
    }

    let request: String = String::from_utf8_lossy(&buffer).to_string();

    println!("{request}\n");
    
    let request: Vec<&str> =request.split_whitespace().collect();
    let resource_path: &str = request.get(1).unwrap();

    println!("Formulating response...\n");

    let mut response: String = String::from("HTTP/1.1 ");
    response.push_str(
        if resource_path == "/" {
            status_code::OK
        } else {
            status_code::NOT_FOUND
        }
    );
    response.push_str("\r\n\r\n");

    println!("Sending response...");

    if let Err(_) = stream.write(response.as_bytes()) {
        eprintln!("Failed to send response.");
    };

    println!("{response}");
}
