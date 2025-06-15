# Rust HTTP Server

This project implements a complete HTTP server using Rust, featuring request parsing, response generation, and static file serving capabilities.

## Project Overview

This server is a comprehensive learning project that demonstrates:
- Full HTTP server implementation in Rust
- Complete HTTP request/response cycle
- Static file serving with security features
- Directory traversal attack prevention
- Rust's ownership model and error handling
- Modular code organization with trait-based architecture

## Project Structure

The project consists of the following files:
- `main.rs`: Entry point of the application
- `server.rs`: Contains the `Server` struct and `Handler` trait
- `website_handler.rs`: Implements static file serving with security features
- `http/mod.rs`: Module declarations for HTTP-related structs
- `http/method.rs`: Defines the `Method` enum for HTTP methods
- `http/request.rs`: Implements the `Request` struct for parsing HTTP requests
- `http/response.rs`: Implements the `Response` struct for generating HTTP responses
- `http/status_code.rs`: Defines HTTP status codes
- `http/query_string.rs`: Handles URL query string parsing

## Features

- **Complete HTTP Server**: Full request-response cycle implementation
- **Static File Serving**: Serves HTML, CSS, JS, and other static files
- **Security**: Directory traversal attack prevention
- **Multiple HTTP Methods**: Support for GET, POST, and other HTTP methods
- **Query String Parsing**: Handles URL parameters
- **Error Handling**: Comprehensive error handling with proper HTTP status codes
- **Modular Architecture**: Clean separation of concerns with trait-based design

## Usage

### Running the Server

1. Ensure you have Rust installed on your system
2. Clone this repository
3. Navigate to the project directory
4. Create a `public` directory and add your HTML files (e.g., `index.html`, `hello.html`)
5. Run the server:
   ```bash
   cargo run
   ```

The server will start and listen on `127.0.0.1:8080`.

### Testing the Server

#### Using curl (Recommended)

**Basic GET request:**
```bash
curl http://127.0.0.1:8080
```

**Request specific pages:**
```bash
curl http://127.0.0.1:8080/hello
curl http://127.0.0.1:8080/index.html
```

**Test with verbose output:**
```bash
curl -v http://127.0.0.1:8080
```

**POST request:**
```bash
curl -X POST http://127.0.0.1:8080 -d "test data"
```

#### Using PowerShell

```powershell
Invoke-WebRequest -Uri "http://127.0.0.1:8080" -Method GET
Invoke-WebRequest -Uri "http://127.0.0.1:8080/hello" -Method GET
```

#### Using a Web Browser

Simply navigate to:
- `http://127.0.0.1:8080` - Serves index.html
- `http://127.0.0.1:8080/hello` - Serves hello.html
- `http://127.0.0.1:8080/any-file.html` - Serves any file from the public directory

#### Using telnet (Advanced)

```bash
telnet 127.0.0.1 8080
```

Then type:
```
GET / HTTP/1.1
Host: 127.0.0.1:8080

```
(Press Enter twice after the Host line)

## Implementation Details

### Server Architecture
- **`Server` struct**: Manages the TCP listener and incoming connections
- **`Handler` trait**: Defines the interface for request handling
- **`WebsiteHandler`**: Implements static file serving with security features

### HTTP Processing
- **Request Parsing**: HTTP requests are parsed into `Request` objects
- **Response Generation**: `Response` objects generate proper HTTP responses
- **Method Support**: The `Method` enum represents different HTTP methods
- **Status Codes**: Proper HTTP status code handling

### Security Features
- **Directory Traversal Prevention**: Prevents access to files outside the public directory
- **Path Canonicalization**: Ensures secure file path resolution

### Error Handling
- Comprehensive error handling using Rust's `Result` type
- Proper HTTP status codes for different error conditions
- Graceful handling of malformed requests

## Example File Structure

```
project-root/
├── src/
│   ├── main.rs
│   ├── server.rs
│   ├── website_handler.rs
│   └── http/
│       ├── mod.rs
│       ├── method.rs
│       ├── request.rs
│       ├── response.rs
│       ├── status_code.rs
│       └── query_string.rs
├── public/
│   ├── index.html
│   ├── hello.html
│   └── style.css
└── Cargo.toml
```

## Learning Points

This project demonstrates several key Rust concepts:
- **Trait Implementations**: `TryFrom`, `FromStr`, custom `Handler` trait
- **Error Handling**: Comprehensive use of `Result` and custom error types
- **String Manipulation**: Working with HTTP headers and body content
- **File System Operations**: Safe file reading with security considerations
- **Network Programming**: TCP listener and stream handling
- **Modular Design**: Clean module organization and separation of concerns

## Current Features

✅ **Complete HTTP request parsing**  
✅ **HTTP response generation**  
✅ **Static file serving**  
✅ **Security against directory traversal attacks**  
✅ **Multiple HTTP method support**  
✅ **Query string parsing**  
✅ **Proper error handling with HTTP status codes**

## Planned Improvements

- [ ] Concurrent connection handling with threading
- [ ] Enhanced logging and metrics
- [ ] Configuration file support
- [ ] HTTPS/TLS support
- [ ] Request routing with pattern matching
- [ ] Middleware support
- [ ] Template engine integration

## Testing Examples

The server responds to various requests:

- **GET /**: Returns `index.html` (200 OK)
- **GET /hello**: Returns `hello.html` (200 OK)
- **GET /nonexistent.html**: Returns 404 Not Found
- **POST /**: Returns 404 Not Found (only GET supported for files)
- **GET /../etc/passwd**: Blocked with security warning (403 Forbidden)

## Contributing

As this is a learning project, contributions that enhance its educational value are welcome. Areas for contribution:
- Additional HTTP method implementations
- Performance improvements
- Security enhancements
- Documentation improvements
- Test coverage

Please feel free to submit a Pull Request or open an Issue for discussion.

## Acknowledgements

This project is based on the Udemy course "Learn Rust by Building Real Applications". Special thanks to the course instructor for the guidance and structure provided. The implementation has been extended beyond the course material to include complete HTTP response handling and security features.

## License

This project is open source and available under the [MIT License](LICENSE).