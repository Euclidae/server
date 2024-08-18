use std::net::{TcpListener,TcpStream};
pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Server {
        Server { address }
    }

    pub fn run(&self) {
        println!("Listening on {}", self.address);
        let listener = TcpListener::bind(&self.address);
    }
}