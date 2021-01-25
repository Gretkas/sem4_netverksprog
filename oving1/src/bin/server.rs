extern crate lib;
use lib::SOCKET_PATH;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::fs;

fn main() {
    let socket = SOCKET_PATH;

    // Bind to socket
    let stream = match TcpListener::bind(&socket) {
        Err(_) => panic!("failed to bind socket"),
        Ok(stream) => stream,
    };

    println!("Server started, waiting for clients");

    // Iterate over clients, blocks if no client available
    for client_stream in stream.incoming() {
        let client_stream = client_stream.unwrap();
        
        handle_connection(client_stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);

    println!("Request: {}", request);

    let html_contents = fs::read_to_string("index.html").unwrap();

    let response = response_html(&request.to_string());

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn response_html(request: &String) -> String {
    let html_contents = fs::read_to_string("index.html").unwrap();
    
    format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        html_contents.len(),
        html_contents
    )
}