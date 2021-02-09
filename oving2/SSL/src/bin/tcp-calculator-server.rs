extern crate lib;
use lib::Calculator::CalculationRequest;
use lib::ThreadPool::ThreadPool;
use lib::SOCKET_PATH;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod, SslStream};
use serde::de::Deserialize;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;

fn main() {
    // The server is allocated 3 workers/threads
    let thread_pool: ThreadPool = ThreadPool::new(3);

    let mut acceptor = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    acceptor
        .set_private_key_file("localhost.key", SslFiletype::PEM)
        .unwrap();
    acceptor
        .set_certificate_chain_file("localhost.crt")
        .unwrap();
    acceptor.check_private_key().unwrap();
    let acceptor = Arc::new(acceptor.build());

    // Bind to socket
    let stream = match TcpListener::bind(SOCKET_PATH) {
        Err(_) => panic!("failed to bind socket"),
        Ok(stream) => stream,
    };

    println!("Server started, waiting for clients");

    // Iterate over clients, blocks if no client available
    for client_stream in stream.incoming() {
        match client_stream {
            Ok(client_stream) => {
                let acceptor = acceptor.clone();

                // Handing over the ownership of the incoming stream to a worker thread which will execute the function.
                thread_pool.excecute(move || {
                    let client_stream = acceptor.accept(client_stream).unwrap();
                    handle_connection(client_stream);
                });
            }
            Err(error) => { /* connection failed */ }
        }
    }
    println!("Shutting down");
}
//this function is handled by individual threads
fn handle_connection(mut stream: SslStream<TcpStream>) {
    // looping over the socket. When the function returns, the TCPstream will be out of scope and will be destroyed.
    // The Stream is owned by this function.

    loop {
        let mut deserialized_stream = serde_json::Deserializer::from_reader(&mut stream);
        let result = CalculationRequest::deserialize(&mut deserialized_stream).unwrap();

        let calculation = match serde_json::to_string(&handle_calculation(result)) {
            Err(_) => {
                stream
                    .write(String::from("Unable to calculate object").as_bytes())
                    .unwrap();
                stream.flush().unwrap();
                return; // if this function does not return, thread will never close, and it will eventually break the system.
            }
            Ok(calculation) => calculation,
        };
        println!("sending calculation back");
        stream.write(calculation.as_bytes()).unwrap();
        stream.flush().unwrap(); // Always flushing the stream to make sure the message is sent properly.
    }
}

// Reading JSON objects directly from stream instead of parsing string from a buffer, it's more prone to failure but a lot more simple.
// fn read_calculation_from_stream(
//     mut tcp_stream: &SslStream<TcpStream>,
// ) -> Result<CalculationRequest, Box<dyn Error>> {
//     let mut deserialized_stream = serde_json::Deserializer::from_reader(&mut tcp_stream);
//     let result = CalculationRequest::deserialize(&mut deserialized_stream)?;

//     Ok(result)
// }

//Unecessary, but that's fine I guess.
fn handle_calculation(req: CalculationRequest) -> CalculationRequest {
    req.calculate()
}
