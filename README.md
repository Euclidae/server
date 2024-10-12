# Rust TCP Server

This project implements a basic TCP server using Rust, capable of parsing simple HTTP requests. 

## Project Overview

This server is a learning project that demonstrates:
- Basic TCP server implementation in Rust
- Parsing of HTTP requests
- Rust's ownership model and error handling
- Modular code organization

## Project Structure

The project consists of the following files:

- `main.rs`: Entry point of the application
- `server.rs`: Contains the `Server` struct and its implementation
- `http/mod.rs`: Module declarations for HTTP-related structs
- `http/method.rs`: Defines the `Method` enum for HTTP methods
- `http/request.rs`: Implements the `Request` struct for parsing HTTP requests

## Features

- TCP server listening on a specified address
- Parsing of incoming HTTP requests
- Support for common HTTP methods (GET, POST, etc.)
- Basic error handling and logging

## Usage

To run the server:

1. Ensure you have Rust installed on your system.
2. Clone this repository.
3. Navigate to the project directory.
4. Run the following command:

   ```
   cargo run
   ```

The server will start and listen on `127.0.0.1:8080`.

## Implementation Details

- The `Server` struct in `server.rs` manages the TCP listener and incoming connections.
- HTTP requests are parsed into `Request` objects in `request.rs`.
- The `Method` enum in `method.rs` represents different HTTP methods.
- Error handling is implemented using Rust's `Result` type.

## Learning Points

This project demonstrates several key Rust concepts:
- Trait implementations (`TryFrom`, `FromStr`)
- Error handling with `Result`
- String and byte slice manipulation
- Modular code organization with Rust's module system

## Current Limitations

- The server currently only parses requests and doesn't generate responses.
- Error handling is basic and could be improved.
- The server doesn't handle concurrent connections.

## Todo

- Implement HTTP response generation
- Add routing for different endpoints
- Improve error handling and logging
- Implement concurrent connection handling

## Contributing

As this is a learning project, contributions that enhance its educational value are welcome. Please feel free to submit a Pull Request or open an Issue for discussion.

## Acknowledgements

This project is based on the Udemy course "Learn Rust by Building Real Applications". Special thanks to the course instructor for the guidance and structure provided.

## License

This project is open source and available under the [MIT License](https://opensource.org/licenses/MIT).
