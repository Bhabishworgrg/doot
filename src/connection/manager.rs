use std::net::TcpStream;

use crate::http::{request, response};

pub fn handle_connection(stream: TcpStream) {
    let request: String = request::read(&stream);
    
    let request: Vec<&str> = request.split_whitespace().collect();
    let resource_path: &str = request.get(1).unwrap();

    response::write(&stream, resource_path);
}
