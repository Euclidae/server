use super::http::{Method, Request, Response, StatusCode};
use super::server::Handler;
use std::fs;

// TODO figure out why this thing keeps failing to serve the web pages
// Done

pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }
    
    fn read_file(&self, file_path: &str) -> Option<String> {
        // Remove leading slash if present
        let file_path = file_path.strip_prefix('/').unwrap_or(file_path);
        
        let path = format!("{}/{}", self.public_path, file_path);
        
        println!("Attempting to read file: {}", path); // Debug output
        
        
        match fs::canonicalize(&self.public_path) {
            Ok(canonical_public_path) => {
                
                match fs::read_to_string(&path) {
                    Ok(contents) => {
                        match fs::canonicalize(&path) {
                            Ok(canonical_file_path) => {
                                if canonical_file_path.starts_with(&canonical_public_path) {
                                    println!("Successfully read file: {}", path);
                                    Some(contents)
                                } else {
                                    println!("Directory Traversal Attack Attempted: {}", file_path);
                                    None
                                }
                            }
                            Err(_) => {
                                // If we can't canonicalize but we read the file, it might be OK
                                // This is a fallback for when canonicalize fails
                                println!("Warning: Could not canonicalize path, but file was read: {}", path);
                                Some(contents)
                            }
                        }
                    }
                    Err(e) => {
                        println!("Failed to read file {}: {}", path, e);
                        None
                    }
                }
            }
            Err(e) => {
                println!("Failed to canonicalize public path {}: {}", self.public_path, e);
                // Fallback: try to read without security check
                match fs::read_to_string(&path) {
                    Ok(contents) => {
                        println!("Successfully read file (fallback): {}", path);
                        Some(contents)
                    }
                    Err(e) => {
                        println!("Failed to read file {}: {}", path, e);
                        None
                    }
                }
            }
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        println!("Handling request: {:?} {}", request.method(), request.path()); // Debug output
        
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/hello" => Response::new(StatusCode::Ok, self.read_file("hello.html")),
                path => {   
                    if path.ends_with(".css") || path.ends_with(".js") || path.ends_with(".html") {
                        match self.read_file(path) {
                            Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
                            None => Response::new(StatusCode::NotFound, None),
                        }
                    } else {
                        match self.read_file(path) {
                            Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
                            None => Response::new(StatusCode::NotFound, None),
                        }
                    }
                }
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}