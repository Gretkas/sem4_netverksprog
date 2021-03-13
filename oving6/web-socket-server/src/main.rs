use http::{Request, Response};
use sha1::{Digest, Sha1};
use std::io::prelude::*;
use std::io::Error;
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6969").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(stream: TcpStream) {
    let mut ws = match WebSocket::new(stream) {
        Ok(websocket) => websocket,
        Err(error) => {
            println!(
                "Encountered error trying to initiate websocket connection: {}",
                error
            );
            return;
        }
    };
    ws.send_message().unwrap();

    loop {
        println!("Message from client!: {}", ws.receive_message().unwrap());
    }
}

struct WebSocket {
    adress: String,
    stream: TcpStream,
    websocket_key: String,
    websocket_key_encoded: String,
}

impl WebSocket {
    pub fn new(mut stream: TcpStream) -> Result<WebSocket, Box<Error>> {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer)?;

        let mut headers = [httparse::EMPTY_HEADER; 16];
        let mut parsed_request = httparse::Request::new(&mut headers);

        parsed_request.parse(&buffer).unwrap();

        let mut request_builder = Request::builder();

        for header in parsed_request.headers {
            request_builder =
                request_builder.header(header.name, String::from_utf8_lossy(header.value).as_ref());
        }

        let request = request_builder.body(()).unwrap();

        let websocket_key = request
            .headers()
            .get("Sec-WebSocket-Key")
            .unwrap()
            .to_str()
            .unwrap();

        let websocket_key_encoded = WebSocket::encode_key(websocket_key)?;

        stream.write(WebSocket::init_websocket_response(&websocket_key_encoded).as_bytes())?;
        stream.flush()?;

        Ok(WebSocket {
            adress: "127.0.0.1:6969".to_owned().to_string(),
            stream,
            websocket_key: websocket_key.to_owned(),
            websocket_key_encoded,
        })
    }

    fn init_websocket_response(websocket_key_encoded: &str) -> String {
        let response = Response::builder()
            .header("Upgrade", "websocket")
            .header("Connection", "Upgrade")
            .header("Sec-WebSocket-Accept", websocket_key_encoded)
            .body(())
            .unwrap();

        let http_response = format!(
            "HTTP/1.1 101 Switching Protocols\r\n\
             Upgrade: websocket\r\n\
             Connection: Upgrade\r\n\
             Sec-WebSocket-Accept: {}\r\n\n",
            websocket_key_encoded,
        );

        return http_response;
    }

    fn encode_key(key: &str) -> Result<String, Box<Error>> {
        let sec_websocket_key_response =
            format!("{}{}", key, "258EAFA5-E914-47DA-95CA-C5AB0DC85B11");
        let mut hasher = Sha1::new();
        hasher.update(sec_websocket_key_response.as_bytes());
        let result = hasher.finalize();

        Ok(base64::encode(result))
    }

    pub fn send_message(&mut self) -> Result<(), Box<Error>> {
        let message = "Hello websocket";

        let mut byte_message: Vec<u8> = Vec::new();

        byte_message.push(129);
        byte_message.push(message.len() as u8);

        for byte in message.as_bytes().into_iter() {
            byte_message.push(byte.to_owned());
        }

        self.stream.write(&byte_message)?;
        self.stream.flush()?;
        Ok(())
    }

    pub fn receive_message(&mut self) -> Result<String, &'static str> {
        let mut buffer = [0; 128];
        self.stream.read(&mut buffer).unwrap();
        println!("{:?}", &buffer[..]);
        println!("{:?}", &buffer[..].len());

        println!(
            "{}",
            String::from_utf8_lossy(&buffer[..])
                .to_string()
                .trim_matches(char::from(0))
                .to_owned()
        );

        let message = WebSocket::decode_websocket_message(&buffer.to_vec())?;
        Ok(message)
    }

    fn decode_websocket_message(buffer: &Vec<u8>) -> Result<String, &'static str> {
        let first_byte: Result<u8, &'static str> = match buffer.get(0) {
            Some(129) => Ok(129),
            _ => return Err("message is not text"),
        };

        let length_of_message = buffer.get(1).unwrap().to_owned() - 128;
        if length_of_message > 125 && first_byte? == 129 {
            return Err("Unable to decode!");
        }

        let mut encoded_data_bytes: Vec<u8> = Vec::new();
        let mut key_data_bytes: Vec<u8> = Vec::new();
        let mut decoded_data_bytes: Vec<u8> = Vec::new();

        println!("length of message: {}", length_of_message);

        for i in 0..4 {
            key_data_bytes.push(buffer.get((i + 2) as usize).unwrap().to_owned());
        }

        println!("{:?}", key_data_bytes);

        for i in 0..length_of_message {
            encoded_data_bytes.push(buffer.get((i + 6) as usize).unwrap().to_owned());
            decoded_data_bytes
                .push(encoded_data_bytes[i as usize] ^ key_data_bytes[(i % 4) as usize]);
        }
        println!("{:?}", encoded_data_bytes);

        Ok(String::from_utf8_lossy(&decoded_data_bytes)
            .to_owned()
            .to_string())
    }
}

// Plan - Accept the connection, analyse the text
// Abort if it is not a valid WS connection
// Continue the connection and respond to the client

// should the stream be handled within the WebSocket?? Maybe

// byte[] response = ("HTTP/1.1 101 Switching Protocols\r\n"
// 						+ "Connection: Upgrade\r\n"
// 						+ "Upgrade: websocket\r\n"
// 						+ "Sec-WebSocket-Accept: "
// 						+ Base64.getEncoder().encodeToString(MessageDigest.getInstance("SHA-1").digest((match.group(1) + "258EAFA5-E914-47DA-95CA-C5AB0DC85B11").getBytes("UTF-8")))
// 						+ "\r\n\r\n").getBytes("UTF-8");
// 					out.write(response, 0, response.length);
