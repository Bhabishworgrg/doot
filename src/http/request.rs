use std::io::Read;
use std::net::TcpStream;

use crate::constants::config;

pub fn read(mut stream: &TcpStream) -> String {
    println!("Reading a request...");
    
    let mut buffer: [u8; config::MAX_REQ_SIZE] = [0; config::MAX_REQ_SIZE];

    if let Err(_) = stream.read(&mut buffer) {
        eprintln!("Failed to read the request.");
    }

    let request = String::from_utf8_lossy(&buffer).to_string();
    println!("{request}\n");

    return request;
}
