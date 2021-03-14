#![warn(rust_2018_idioms)]
use http::{Request, Response};
use sha1::{Digest, Sha1};
use std::collections::HashMap;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::Interest;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{ReadHalf, WriteHalf};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, Mutex};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:6969").await?;
    let state = Arc::new(Mutex::new(SharedClients::new()));

    loop {
        let (stream, client_adress) = listener.accept().await?;
        let state = Arc::clone(&state);
        tokio::spawn(async move {
            println!("Accepted connection from {}", &client_adress);

            if let Err(e) = handle_connection(stream, state, client_adress).await {
                println!("an error occurred; error = {:?}", e);
            }
        });
    }
}

async fn handle_connection(
    stream: TcpStream,
    clients: Arc<Mutex<SharedClients>>,
    address: SocketAddr,
) -> Result<(), Box<dyn Error>> {
    let mut ws = match WebSocket::new(stream, clients, address).await {
        Ok(websocket) => websocket,
        Err(error) => {
            println!(
                "Encountered error trying to initiate websocket connection: {}",
                &error
            );
            return Err(error);
        }
    };

    let (mut reader, mut writer) = ws.stream.split();
    loop {
        let mut buffer = [0 as u8; 1024];
        tokio::select! {
            Some(message) = ws.rx.recv() => {
                send_message(&mut writer, message).await?;
            }
            result = reader.read(&mut buffer) => match result {
                // A message was received from the current user, we should
                // broadcast this message to the other users.
                Ok(msg) => {
                    println!("msg: {:?}", msg);
                }
                // An error occurred.
                Err(e) => {
                    break;
                }

            },
        }
    }

    // {
    //     let mut clients = clients.lock().await;
    //     clients.clients.remove(&ws.client_adress);

    //     let msg = format!("{} has left the chat", ws.adress);
    //     clients.broadcast(&msg).await;
    // }

    Ok(())
}
type Tx = mpsc::UnboundedSender<String>;

type Rx = mpsc::UnboundedReceiver<String>;

struct SharedClients {
    clients: HashMap<SocketAddr, Tx>,
}
impl SharedClients {
    pub fn new() -> SharedClients {
        SharedClients {
            clients: HashMap::new(),
        }
    }

    async fn broadcast(&mut self, message: &str) {
        for peer in self.clients.iter_mut() {
            peer.1.send(message.into()).unwrap();
        }
    }
}

struct WebSocket {
    adress: String,
    stream: TcpStream,
    websocket_key: String,
    websocket_key_encoded: String,
    client_adress: SocketAddr,
    clients: Arc<Mutex<SharedClients>>,
    rx: Rx,
}

impl WebSocket {
    pub async fn new(
        mut stream: TcpStream,
        clients: Arc<Mutex<SharedClients>>,
        address: SocketAddr,
    ) -> Result<WebSocket, Box<dyn Error>> {
        let mut buffer = [0; 1024];
        stream.readable().await?;
        stream.read(&mut buffer).await?;

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
        stream.writable().await?;
        stream
            .write_all(WebSocket::init_websocket_response(&websocket_key_encoded).as_bytes())
            .await
            .unwrap();

        let (tx, rx) = mpsc::unbounded_channel();

        // Add an entry for this `Peer` in the shared state map.
        clients.lock().await.clients.insert(address.clone(), tx);

        {
            let mut state = clients.lock().await;
            let msg = format!("{} has joined the chat", &address);
            state.broadcast(&msg).await;
        }

        Ok(WebSocket {
            adress: "127.0.0.1:6969".to_owned().to_string(),
            stream,
            websocket_key: websocket_key.to_owned(),
            websocket_key_encoded,
            rx,
            client_adress: address,
            clients,
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

    fn encode_key(key: &str) -> Result<String, Box<dyn Error>> {
        let sec_websocket_key_response =
            format!("{}{}", key, "258EAFA5-E914-47DA-95CA-C5AB0DC85B11");
        let mut hasher = Sha1::new();
        hasher.update(sec_websocket_key_response.as_bytes());
        let result = hasher.finalize();

        Ok(base64::encode(result))
    }

    pub async fn send_message(&mut self, message: String) -> Result<(), Box<dyn Error>> {
        let mut byte_message: Vec<u8> = Vec::new();

        byte_message.push(129);
        byte_message.push(message.len() as u8);

        for byte in message.as_bytes().into_iter() {
            byte_message.push(byte.to_owned());
        }

        self.stream.writable().await?;
        self.stream.write_all(&byte_message).await?;
        Ok(())
    }

    pub async fn receive_message(&mut self) -> Result<String, Box<dyn Error>> {
        let ready = self
            .stream
            .ready(Interest::READABLE | Interest::WRITABLE)
            .await?;
        let mut buffer = [0; 1024];
        if ready.is_readable() {
            // Try to read data, this may still fail with `WouldBlock`
            // if the readiness event is a false positive.
            match self.stream.read(&mut buffer).await {
                Ok(n) => {
                    println!("read {} bytes", n);
                }
                Err(e) => {
                    return Err(e.into());
                }
            }
        }
        //self.stream.try_read(&mut buffer).unwrap();

        println!("{:?}", &buffer[..]);
        println!("{:?}", &buffer[..].len());

        println!(
            "{}",
            String::from_utf8_lossy(&buffer[..])
                .to_string()
                .trim_matches(char::from(0))
                .to_owned()
        );

        let message = WebSocket::decode_websocket_message(&buffer.to_vec()).await?;
        println!("{}", &message);
        Ok(message)
    }

    async fn decode_websocket_message(buffer: &Vec<u8>) -> Result<String, &'static str> {
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

async fn send_message(stream: &mut WriteHalf<'_>, message: String) -> Result<(), Box<dyn Error>> {
    let mut byte_message: Vec<u8> = Vec::new();

    byte_message.push(129);
    byte_message.push(message.len() as u8);

    for byte in message.as_bytes().into_iter() {
        byte_message.push(byte.to_owned());
    }

    stream.write_all(&byte_message).await?;
    Ok(())
}

async fn receive_message(stream: &mut ReadHalf<'_>) -> Result<String, Box<dyn Error>> {
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer).await {
        Ok(n) => {
            println!("read {} bytes", n);
        }
        Err(e) => {
            return Err(e.into());
        }
    }
    //self.stream.try_read(&mut buffer).unwrap();

    println!("{:?}", &buffer[..]);
    println!("{:?}", &buffer[..].len());

    println!(
        "{}",
        String::from_utf8_lossy(&buffer[..])
            .to_string()
            .trim_matches(char::from(0))
            .to_owned()
    );

    let message = WebSocket::decode_websocket_message(&buffer.to_vec()).await?;
    println!("{}", &message);
    Ok(message)
}
