use std::convert::TryFrom;
use super::method::Method;
use std::str::FromStr;

#[derive(Debug)]
pub struct Request {
    pub path: String,
    pub query_string: Option<String>,
    pub method: Method,
}

impl TryFrom<&[u8]> for Request {
    type Error = String;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let request = String::from_utf8_lossy(buf);
        let (method, request) = get_next_word(&request).ok_or("Method not found")?;
        let (mut path, request) = get_next_word(request).ok_or("Path not found")?;
        let (protocol, _) = get_next_word(request).ok_or("Protocol not found")?;

        if protocol != "HTTP/1.1" {
            return Err("Invalid protocol".into());
        }

        let method: Method = method.parse()?;

        let mut query_string = None;
        if let Some(i) = path.find('?') {
            query_string = Some(path[i + 1..].to_string());
            path = &path[..i];
        }

        Ok(Self {
            path: path.to_string(),
            query_string,
            method,
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }
    None
}

impl FromStr for Method {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Method::GET),
            "DELETE" => Ok(Method::DELETE),
            "POST" => Ok(Method::POST),
            "HEAD" => Ok(Method::HEAD),
            "CONNECT" => Ok(Method::CONNECT),
            "OPTIONS" => Ok(Method::OPTIONS),
            "TRACE" => Ok(Method::TRACE),
            "PATCH" => Ok(Method::PATCH),
            _ => Err(format!("Invalid method: {}", s)),
        }
    }
}
