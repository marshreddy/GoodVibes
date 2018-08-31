extern crate slack_hook;
extern crate hyper;
use slack_hook::{Slack, PayloadBuilder};
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs;
use hyper::service::service_fn_ok;
use std::io::{self, Write};
use hyper::Client;
use hyper::rt::{self, Future, Stream};
use hyper::{Method, Request};

fn main() {
    //This listener is setup to listen for Alexa commands from the Happy Sat invocation 
    //and from the offerzen satellite itself.
    let listener = TcpListener::bind("127.0.0.1:443").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
       
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

     
    if buffer.starts_with(get) {
        //This is the message that is sent to Alexa and to the offerzen satelite        
        let contents = fs::read_to_string("/Users/Marsh/Documents/GitHub/GoodVibes/hello/src/response.json").unwrap();

        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
    
        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();

        post_to_offerzen_sat();

    } else {
        // This reads the response from the offerzen satellite, and then posts to slack.    
        let contents = fs::read_to_string("/Users/Marsh/Documents/GitHub/GoodVibes/hello/src/blank.json").unwrap();

        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();

        let slack = Slack::new("https://hooks.slack.com/services/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").unwrap();
        let p = PayloadBuilder::new()
      .text("Good Vibes are coming!!!")
      .build()
      .unwrap();

        let res = slack.send(&p);
    match res {
        Ok(()) => println!("ok"),
        Err(x) => println!("ERR: {:?}",x)

    }
}

    fn post_to_offerzen_sat() {

        let json = r#"{"library":"hyper"}"#;
        let uri: hyper::Uri = "https://make.offerzen.com/satellite/beam.json".parse().unwrap();
        let mut req = Request::new(Body::from(json));
        *req.method_mut() = Method::POST;
        *req.uri_mut() = uri.clone();
        req.headers_mut().insert(
            hyper::header::CONTENT_TYPE,
            HeaderValue::from_static("application/json")
        );

        let post = client.request(req).and_then(|res| {
            println!("POST: {}", res.status());

            res.into_body().concat2()
});


    
    }




}

