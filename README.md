# Rust TCP Server

This project implements a basic TCP server using Rust. It provides a simple framework for handling HTTP-like requests.

## Features

- TCP server implementation
- Basic HTTP request parsing
- Customizable request handling

## Project Structure

The project consists of the following files:

- `main.rs`: Entry point of the application
- `server.rs`: Contains the `Server` struct and its implementation
- `http/mod.rs`: Module declarations for HTTP-related structs
- `http/method.rs`: Defines the `Method` enum for HTTP methods
- `http/request.rs`: Implements the `Request` struct for parsing HTTP requests

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

- The `Server` struct is responsible for creating and managing the TCP listener.
- HTTP requests are parsed into `Request` objects.
- The `Method` enum represents different HTTP methods (GET, POST, etc.).

## Todo

- Implement proper error handling
- Add request routing
- Implement response generation
- Add support for different content types
- Implement logging

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is open source and available under the [MIT License](https://opensource.org/licenses/MIT).
