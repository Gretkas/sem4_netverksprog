extern crate lib;
use lib::Calculator::{Calculation, CalculationRequest};
use std::io::stdin;
use std::net::UdpSocket;

fn main() {
    match UdpSocket::bind(lib::SOCKET_PATH1) {
        Ok(socket) => {
            println!("Please type out your calculation in this format 10 + 10 + 10 - 10, remember space inbetween number and operators");

            println!(
                "Only + and - are supported. Press enter or invalid request on a new line to Exit"
            );
            loop {
                let mut input = String::new(); // mutable empty string for equations

                stdin().read_line(&mut input).expect("Error reading input"); // reading input

                let input: Vec<&str> = input.split_whitespace().collect();
                if input.is_empty() || input.len() < 2 {
                    break; // breaks loop and exits if input is empty or insufficient to create a calculation
                }

                let request = convert_string_to_calculation(&input);

                let request_json = convert_calculation_to_json(&request);

                match socket.send_to(&request_json.as_bytes(), lib::SOCKET_PATH2) {
                    Ok(number_of_bytes) => println!("I sent {:?} bytes ", number_of_bytes),
                    Err(fail) => println!("failed sending {:?}", fail),
                };

                let mut buf = [0; 1024];
                let (number_of_bytes, src_addr) =
                    socket.recv_from(&mut buf).expect("no data received");
                let result = std::str::from_utf8(&buf)
                    .unwrap()
                    .trim_matches(char::from(0));
                println!("Received {:?} bytes from: {:?} ", number_of_bytes, src_addr);

                println!("JSON request is: {:?}", result);

                let result_calculation: CalculationRequest = serde_json::from_str(result).unwrap();
                let result = result_calculation.calculate();

                println!("answer is: {:?}", &result.result);
            }
        }
        Err(error) => {
            println!("Failed to connect to socket: {}", error);
        }
    }
    println!("Terminated.");
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
