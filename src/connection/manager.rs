use std::io::Write;
use std::net::TcpStream;

use crate::constants::status_code;
use crate::http::request;

pub fn handle_connection(mut stream: TcpStream) {
    let request: String = request::read(&mut stream);
    
    let request: Vec<&str> = request.split_whitespace().collect();
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
