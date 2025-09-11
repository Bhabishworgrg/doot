use std::net::TcpListener;
use std::process::exit;

use crate::connection;
use crate::constants::config;

pub fn start() {
    let listener: TcpListener = TcpListener::bind((config::HOST, config::PORT))
        .unwrap_or_else(|_| {
            eprintln!("Failed to create a TCP listener.");
            exit(1);
        });

    for incoming in listener.incoming() {
        match incoming {
            Ok(stream) => connection::manager::handle_connection(stream),
            Err(_) => eprintln!("Failed to accept a new connection.")
        };
    }
}
