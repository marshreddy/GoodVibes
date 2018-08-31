use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:443").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
        post_to_offerzen();
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    let contents = fs::read_to_string("/Users/Marsh/Documents/GitHub/GoodVibes/hello/src/response.json").unwrap();

    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
    

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn post_to_offerzen() {}

