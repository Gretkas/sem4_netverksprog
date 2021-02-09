# Network programming Exercise 2 - Sigmund Granaas

The programs are written entirely in Rust, to compile them, you need Rustc, or Crates. A simple way to install rust is using rustup: <https://rustup.rs/>

## Binaries

To run the configured binaries, you can compile the source files, or run them with cargo, as specified in cargo.toml.

```toml
[lib]
name = "lib"
path = "src/lib/udp_server/lib.rs"

[[bin]]
name = "udp-calculation-server"
path = "src/bin/udp-calculator-server.rs"

[[bin]]
name = "udp-client"
path = "src/bin/udp-client.rs"
```

To run the bin with cargo type either of the following commands from the same directory as Cargo.toml.

```bash
cargo run --bin udp-client
cargo run --bin udp-calculation-server

```

The udp client and the udp calculation server communicates with each other. First start the server, then the client. Currently the adresses and ports are configured as a static string in `src/lib/udp_server/lib.rs` defaulting to `127.0.0.1:6969` and `127.0.0.1:7979`. The client and server will connect to either one of them and send messages to the other.

## Multithreading

No multithreading is really needed here, as the requests are handled without keeping a connection alive. This does not block anything, and removed the need for a multithreaded solution.

## Stability and error handling

Rust offers exhaustive handling of errors, though Enums such as `Result<Ok, Err>`, in some cases I have dealt with errors that might occur, but for the most part I hace handled the possibility of an error by calling `.unwrap()` this wil crash the program if something goes wrong. Some Errors should crash the service, as there is no point to continue if you can't for example bind to the socket. Mostly the program should handle errors from user input, but errors with the connection will likely crash the thread handling the connection to the server as there is no "failover".

## Providing valid Calculator input

The calculator has been made to transfer JSON calculation object which can handle simple calulations with the + and operators. floats are not supported. If the server is provided with Invalid JSON syntax it will try to compute what is present, but is might not be what should be expected. Proper syntax would be `10 + 10 - 1 + 3` or `11234 - 33336`. Remember a whitespace between numbers and operators. The handling of calculation errors could have been handled better, but building a robust calculator is not the scope of this exercise.

This is my first attempt at programming with Rust, the code is pretty garbage as I'm still trying to grasp fundamentals of the Rust programming language.

## Changes from the TCP calculator

There are no changes made to the calculator, as I already solved it by creating DTO's. The communication between server and client has been simplified a bit and almost everything is handled inside the main function. The implementation of using UDP in this case makes the entire program a lot easier to implement, as you can just fire off requests left and right without worrying about missed messages. If you need this totally awesome calculator to be reliable, you are in the wrong spot bud.


## SSL

For testing SSL functionality, I ditched the java example and rewrote the code in the first exercise to handle connections using SSL by implementing the example from the openSSL Module from the rust docs <https://docs.rs/openssl/0.9.19/openssl/ssl/index.html>. The code is not that much different and can be found under the SSL folder. 
