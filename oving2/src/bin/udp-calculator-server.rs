extern crate lib;
use lib::Calculator::CalculationRequest;
use std::net::UdpSocket;

//Multithreading is useless in cases like this. Nothing holds on to the connection.
fn main() {
    match UdpSocket::bind(lib::SOCKET_PATH2) {
        Ok(socket) => loop {
            let mut buf = [0; 1024]; // I am capping the buffer at 1024 bytes, any message longer than this will crash the server.
            let (number_of_bytes, src_addr) = socket.recv_from(&mut buf).expect("no data received");
            let result = std::str::from_utf8(&buf)
                .unwrap()
                .trim_matches(char::from(0));

            println!("Received {:?} bytes from: {:?} ", number_of_bytes, src_addr);

            println!("JSON request is: {:?}", result);

            let result_calculation: CalculationRequest = serde_json::from_str(result).unwrap();
            let result = result_calculation.calculate();

            println!("answer is: {:?}, returning answer", &result.result);
            match socket.send_to(serde_json::to_string(&result).unwrap().as_bytes(), src_addr) {
                Ok(number_of_bytes) => println!("I sent {:?} bytes ", number_of_bytes),
                Err(fail) => println!("failed sending {:?}", fail),
            }
        },
        Err(error) => {
            println!("Failed to connect to socket: {}", error);
        }
    }
}
