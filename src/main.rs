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
            Ok(_) => println!("Accepted a new connection"),
            Err(_) => println!("Error accepting a connection")
        }
    }
}
