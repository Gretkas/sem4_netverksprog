use http::{Request, Response, StatusCode};
use httparse::Header;
use sha1::{Digest, Sha1};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6969").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let ws = WebSocket::new(stream);
}

struct WebSocket {
    adress: String,
    stream: TcpStream,
}

impl WebSocket {
    pub fn new(mut stream: TcpStream) -> WebSocket {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();

        let mut headers = [httparse::EMPTY_HEADER; 16];
        let mut parsed_request = httparse::Request::new(&mut headers);

        parsed_request.parse(&buffer).unwrap();

        let mut request_builder = Request::builder();

        for header in parsed_request.headers {
            request_builder =
                request_builder.header(header.name, String::from_utf8_lossy(header.value).as_ref());
            println!("{}", header.name);
            println!("{}", String::from_utf8_lossy(header.value));
        }

        let request = request_builder.body(()).unwrap();

        let sec_websocket_key = request.headers().get("Sec-WebSocket-Key").unwrap();

        let sec_websocket_key_response = format!(
            "{}{}",
            sec_websocket_key.to_str().unwrap(),
            "258EAFA5-E914-47DA-95CA-C5AB0DC85B11"
        );
        let mut hasher = Sha1::new();
        hasher.update(sec_websocket_key_response.as_bytes());
        let result = hasher.finalize();

        let encoded_result = base64::encode(result);

        let response = Response::builder()
            .header("Upgrade", "websocket")
            .header("Connection", "Upgrade")
            .header("Sec-WebSocket-Accept", &encoded_result)
            .body(())
            .unwrap();

        let http_response = format!(
            "HTTP/1.1 101 Switching Protocols\r\n\
             Upgrade: websocket\r\n\
             Connection: Upgrade\r\n\
             Sec-WebSocket-Accept: {}\r\n\n",
            &encoded_result,
        );

        stream.write(http_response.as_bytes()).unwrap();
        stream.flush().unwrap();

        while true {}
        return WebSocket {
            adress: "127.0.0.1:6969".to_owned().to_string(),
            stream,
        };
    }

    fn init_web_Socket(&self, stream: TcpStream) -> bool {
        let get = b"GET / HTTP/1.1\r\n";

        todo!();
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
