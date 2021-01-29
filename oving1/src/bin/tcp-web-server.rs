extern crate lib;
use lib::ThreadPool::ThreadPool;
use lib::SOCKET_PATH;
use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    let thread_pool: ThreadPool = ThreadPool::new(4);

    // Bind to socket
    let stream = match TcpListener::bind(SOCKET_PATH) {
        Err(_) => panic!("failed to bind socket"),
        Ok(stream) => stream,
    };

    println!("Server started, waiting for clients");

    // Iterate over clients, blocks if no client available
    for client_stream in stream.incoming() {
        let client_stream = client_stream.unwrap();

        thread_pool.excecute(|| {
            handle_connection(client_stream);
        })
    }
    println!("Shutting down")
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);

    //println!("Request: {}", request);

    //let html_contents = fs::read_to_string("index.html").unwrap();

    let response = response_html(&request.to_string());

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn response_html(request: &String) -> String {
    let mut html_contents = fs::read_to_string("start.html").unwrap();

    let html_header_as_list = add_html_header_as_string(request);

    html_contents += &html_header_as_list;

    html_contents += &fs::read_to_string("end.html").unwrap();
    format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        html_contents.len(),
        html_contents
    )
}

fn add_html_header_as_string(header: &String) -> String {
    let mut header_as_list: String = "<ul>".to_string();

    for header_line in header.lines() {
        header_as_list += "<li>";
        header_as_list += header_line;
        header_as_list += "</li>";
    }

    header_as_list += "</ul>";

    return header_as_list;
}
