use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:5432").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
        post_to_offerzen();
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn post_to_offerzen() {}

