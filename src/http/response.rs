use std::{io::Write, net::TcpStream};

use crate::constants::status_code;

pub fn write(mut stream: &TcpStream, resource_path: &str) {
    println!("Formulating response...\n");

    let mut response = String::from("HTTP/1.1 ");
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
