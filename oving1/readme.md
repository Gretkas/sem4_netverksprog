# Network programming Exercise 1 - Sigmund Granaas

The programs are written entirely in Rust, to compile them, you need Rustc, or Crates. A simple way to install rust is using rustup: <https://rustup.rs/>

## Binaries

To run the configured binaries, you can compile the source files, or run them with cargo, as specified in cargo.toml.

```toml
[lib]
name = "lib"
path = "src/lib/multithreaded_tcp_server/lib.rs"

[[bin]]
name = "tcp-web-server"
path = "src/bin/tcp-web-server.rs"

[[bin]]
name = "tcp-calculation-server"
path = "src/bin/tcp-calculator-server.rs"

[[bin]]
name = "tcp-client"
path = "src/bin/tcp-client.rs"
```

To run the bin with cargo type either of the following commands from the same directory as Cargo.toml.

```bash
cargo run --bin tcp-client
cargo run --bin tcp-calculation-server
cargo run --bin tcp-web-server

```

The tcp client and the tcp calculation server communicates with each other. First start the server, then the client. Currently the adress and ports are configured as a static string in `src/lib/multithreaded_tcp_server/lib.rs` defaulting to `127.0.0.1:6969` This is the adress and port everything will connect to. This means that the web server acn calculation server cannot be connected to at the same time, without changing one of their socket paths.

## Multithreading

Both server relies on the same library for utilizing several threads do do concurrent work. The amount of threads allocated is specified in the individual servers. Both the calculator and web server can handle concurrent users. This apporoach to handling web requests and simple calculation requests is overkill and a huge waste of resources. An asynchronous event loop would handle this a lot more efficiently, but I have no idea how to make one. Threads should be recycled once they are done computing.

## Stability and error handling

Rust offers exhaustive handling of errors, though Enums such as `Result<Ok, Err>`, in some cases I have dealt with errors that might occur, but for the most part I hace handled the possibility of an error by calling `.unwrap()` this wil crash the program if something goes wrong. Some Errors should crash the service, as there is no point to continue if you can't for example bind to the socket. Mostly the program should handle errors from user input, but errors with the connection will likely crash the thread handling the connection to the server as there is no "failover".

## Providing valid Calculator input

The calculator has been made to transfer JSON calculation object which can handle simple calulations with the + and operators. floats are not supported. If the server is provided with Invalid JSON syntax it will try to compute what is present, but is might not be what should be expected. Proper syntax would be `10 + 10 - 1 + 3` or `11234 - 33336`. Remember a whitespace between numbers and operators. The handling of calculation errors could have been handled better, but building a robust calculator is not the scope of this exercise.

This is my first attempt at programming with Rust, the code is pretty garbage as I'm still trying to grasp the Rust programming language.
