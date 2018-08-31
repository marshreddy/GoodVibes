use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs;


fn main() {
    //This listener is setup to listen for Alexa commands from the Happy Sat invocation 
    //and from the offerzen satellite itself.
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

    let get = b"GET / HTTP/1.1\r\n";

     
    if buffer.starts_with(get) {
        //This is the message that is sent to Alexa        
        let contents = fs::read_to_string("/Users/Marsh/Documents/GitHub/GoodVibes/hello/src/response.json").unwrap();

        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
    
        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();

        post_to_offerzen();

    } else {

        let contents = fs::read_to_string("/Users/Marsh/Documents/GitHub/GoodVibes/hello/src/blank.json").unwrap();

        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

fn post_to_offerzen() {}

