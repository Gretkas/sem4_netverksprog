extern crate lib;
use lib::Calculator::CalculationRequest;
use lib::ThreadPool::ThreadPool;
use lib::SOCKET_PATH;
use serde::de::Deserialize;
use std::error::Error;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};

fn main() {
    let thread_pool: ThreadPool = ThreadPool::new(10);

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
    let bufread = BufReader::new(&stream);
    for line in bufread.lines() {
        println!("{}", line.unwrap());
    }
    let request: CalculationRequest = read_calculation_from_stream(&stream).unwrap();

    let calculation = match serde_json::to_string(&handle_calculation(request)) {
        Err(_) => {
            stream
                .write(String::from("Unable to calculate object").as_bytes())
                .unwrap();
            stream.flush().unwrap();
            return;
        }
        Ok(calculation) => calculation,
    };
    println!("sending calculation back");
    stream.write(calculation.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn read_calculation_from_stream(
    tcp_stream: &TcpStream,
) -> Result<CalculationRequest, Box<dyn Error>> {
    let mut deserialized_stream = serde_json::Deserializer::from_reader(tcp_stream);
    let result = CalculationRequest::deserialize(&mut deserialized_stream)?;

    Ok(result)
}

fn handle_calculation(req: CalculationRequest) -> CalculationRequest {
    req.calculate()
}
