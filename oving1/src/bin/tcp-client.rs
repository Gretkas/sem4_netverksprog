extern crate lib;
use lib::Calculator::{Calculation, CalculationRequest};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

fn main() {
    match TcpStream::connect(lib::SOCKET_PATH) {
        Ok(mut stream) => {
            println!("Successfully connected to {}", lib::SOCKET_PATH);

            let calculation_vec: Vec<Calculation> = [
                Calculation::Number(1),
                Calculation::Operator(String::from("-")),
                Calculation::Number(1),
                Calculation::Operator(String::from("-")),
                Calculation::Number(1),
                Calculation::Operator(String::from("-")),
                Calculation::Number(1),
            ]
            .to_vec();

            let calculation_req: CalculationRequest = CalculationRequest {
                calculation: calculation_vec,
                result: 0,
            };

            let calculation_req = match serde_json::to_string(&calculation_req) {
                Err(_) => {
                    panic!("Invalid calculation object");
                }
                Ok(calculation) => calculation,
            };

            println!("{}", &calculation_req);

            stream.write(calculation_req.as_bytes()).unwrap();
            println!("Sent calculation, awaiting reply...");

            let mut data = [0 as u8; 1024]; // using 1024 byte buffer
            match stream.read(&mut data) {
                Ok(_) => {
                    let text = from_utf8(&data).unwrap();
                    println!("reply: {}", text);
                }
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}
