extern crate lib;
use lib::Calculator::{Calculation, CalculationRequest};
use serde::de::Deserialize;
use std::error::Error;
use std::io::{stdin, Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

fn main() {
    match TcpStream::connect(lib::SOCKET_PATH) {
        Ok(mut stream) => {
            println!("Successfully connected to {}", lib::SOCKET_PATH);

            println!("Please type out your calculation in this format 10 + 10 + 10 - 10, remember space inbetween number and operators");

            println!(
                "Only + and - are supported. Press enter or invalid request on a new line to Exit"
            );

            loop {
                let mut input = String::new();

                stdin().read_line(&mut input).expect("Error reading input");
                if input.is_empty() {
                    break;
                }
                let input: Vec<&str> = input.split_whitespace().collect();
                if input.is_empty() || input.len() < 2 {
                    break;
                }

                let request = convert_string_to_calculation(&input);

                let request_json = convert_calculation_to_json(&request);

                write_to_server(&mut stream, &request_json);
                //println!("{}", &request_json);
                read_from_server(&mut stream);
                // match read_calculation_from_stream(&stream) {
                //     Ok(result) => {
                //         println!("Answer is: {}", result.result);
                //     }
                //     Err(error) => println!("{:?}", error),
                // };
            }
        }
        Err(error) => {
            println!("Failed to connect to server: {}", error);
        }
    }
    println!("Terminated.");
}

fn write_to_server(mut stream: &TcpStream, message: &str) {
    stream.write(message.as_bytes()).unwrap();
    stream.flush().unwrap();
    println!("Sent calculation, awaiting reply...");
}

fn read_from_server(mut stream: &TcpStream) {
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

fn convert_calculation_to_json(calculation: &CalculationRequest) -> String {
    match serde_json::to_string(calculation) {
        Err(_) => {
            panic!("Invalid calculation object");
        }
        Ok(calculation) => calculation,
    }
}

fn read_calculation_from_stream(
    tcp_stream: &TcpStream,
) -> Result<CalculationRequest, Box<dyn Error>> {
    let mut deserialized_stream = serde_json::Deserializer::from_reader(tcp_stream);
    let result = CalculationRequest::deserialize(&mut deserialized_stream)?;

    Ok(result)
}
