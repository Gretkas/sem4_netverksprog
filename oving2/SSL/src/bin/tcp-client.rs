extern crate lib;
use lib::Calculator::{Calculation, CalculationRequest};
use openssl::ssl::SslConnector;
use openssl::ssl::{SslMethod, SslStream};
use serde::de::Deserialize;
use std::error::Error;
use std::io::{stdin, Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

fn main() {
    match TcpStream::connect(lib::SOCKET_PATH) {
        Ok(stream) => {
            let connector = SslConnector::builder(SslMethod::tls()).unwrap().build();
            let mut stream = connector.connect("localhost", stream).unwrap();
            println!("Successfully connected to {}", lib::SOCKET_PATH);

            println!("Please type out your calculation in this format 10 + 10 + 10 - 10, remember space inbetween number and operators");

            println!(
                "Only + and - are supported. Press enter or invalid request on a new line to Exit"
            );

            // Continually looping over user input, this way we need to pass a reference to the TCPstream
            // I could Create a new stream on every loop, that way it would go out of scope on every loop. Not really needed here.
            loop {
                let mut input = String::new();

                stdin().read_line(&mut input).expect("Error reading input");

                let input: Vec<&str> = input.split_whitespace().collect();
                if input.is_empty() || input.len() < 2 {
                    break; // breaks loop and exits if input is empty or insufficient to create a calculation
                }

                let request = convert_string_to_calculation(&input);

                let request_json = convert_calculation_to_json(&request);

                stream.write_all(request_json.as_bytes()).unwrap();
                stream.flush().unwrap();
                //println!("{}", &request_json); // remove this comment if you wish to see the object being sent.
                let mut deserialized_stream = serde_json::Deserializer::from_reader(&mut stream);
                let result = CalculationRequest::deserialize(&mut deserialized_stream).unwrap();
                println!("{:?}", result.result);
            }
        }
        Err(error) => {
            println!("Failed to connect to server: {}", error);
        }
    }
    println!("Terminated.");
}

// writing a anything that can be converted to a string reference to the stream and flushing it.
fn write_to_server(mut stream: SslStream<TcpStream>, message: &str) {
    stream.write_all(message.as_bytes()).unwrap();
    stream.flush().unwrap();
    println!("Sent calculation, awaiting reply...");
}

// reading string from the stream through a buffer, is useful for reading plain messages.
fn read_from_server(mut stream: SslStream<&TcpStream>) {
    let mut data = [0 as u8; 1024]; // using 1024 byte buffer
    match stream.read(&mut data) {
        Ok(_) => {
            let text = from_utf8(&data).unwrap();
            println!("reply: {}", text);
        }
        Err(e) => {
            println!("Failed to load data: {}", e);
        }
    }
}

// Converting a vector of strings into a calculation struct that can be translated to JSON.
fn convert_string_to_calculation(calculation_req: &Vec<&str>) -> CalculationRequest {
    let mut calculation: Vec<Calculation> = Vec::new();

    for n in calculation_req.iter() {
        match n {
            &"-" => calculation.push(Calculation::Operator(String::from("-"))),
            &"+" => calculation.push(Calculation::Operator(String::from("+"))),
            _ => {
                let chars_are_numeric: Vec<bool> = n.chars().map(|c| c.is_numeric()).collect();
                let b = !chars_are_numeric.contains(&false);
                if b {
                    calculation.push(Calculation::Number(n.trim().parse::<i32>().unwrap()))
                }
            }
        }
    }
    CalculationRequest {
        calculation: calculation,
        result: 0,
    }
}

//Converting a struct into an Owned string using the JSON format. This is done using Serde Json
fn convert_calculation_to_json(calculation: &CalculationRequest) -> String {
    match serde_json::to_string(calculation) {
        Err(_) => {
            panic!("Invalid calculation object");
        }
        Ok(calculation) => calculation,
    }
}
//Reading and deserializing potential JSON objects directly from stream instead of parsing from a string.
fn read_calculation_from_stream(
    tcp_stream: SslStream<TcpStream>,
) -> Result<CalculationRequest, Box<dyn Error>> {
    let mut deserialized_stream = serde_json::Deserializer::from_reader(tcp_stream);
    let result = CalculationRequest::deserialize(&mut deserialized_stream)?;

    Ok(result)
}
