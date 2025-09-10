use std::io::Write;
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
                match stream.write(b"HTTP/1.1 200 OK\r\n\r\n") {
                    Ok(bytes) => bytes,
                    Err(_) => {
                        println!("Failed to responed to request");
                        return;
                    }
                };
            },
            Err(_) => println!("Error accepting a connection")
        }
    }
}
