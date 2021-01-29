extern crate lib;
use lib::ThreadPool::ThreadPool;
use lib::SOCKET_PATH;
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
    println!("Shutting down");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);

    stream.write("hello".as_bytes()).unwrap();
    stream.flush().unwrap();
}
