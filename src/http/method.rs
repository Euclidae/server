use std::str::FromStr;
// This file defines the HTTP methods used in requests.
// It provides an enum for the methods and implements FromStr for parsing strings into the enum.
// The methods include GET, POST, PUT, DELETE, HEAD, CONNECT, OPTIONS, TRACE, and PATCH.
// The MethodError is used to handle cases where the string does not match any known HTTP method.
// This is a simple implementation that can be extended with more methods or features as needed.
// This file is part of a simple HTTP server implementation in Rust.


#[derive(Debug)]
pub enum Method {
    GET,
    DELETE,
    POST,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

impl FromStr for Method {
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::GET),
            "DELETE" => Ok(Self::DELETE),
            "POST" => Ok(Self::POST),
            "PUT" => Ok(Self::PUT),
            "HEAD" => Ok(Self::HEAD),
            "CONNECT" => Ok(Self::CONNECT),
            "OPTIONS" => Ok(Self::OPTIONS),
            "TRACE" => Ok(Self::TRACE),
            "PATCH" => Ok(Self::PATCH),
            _ => Err(MethodError),
        }
    }
}

pub struct MethodError;
