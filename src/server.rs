use std::net::{TcpListener,TcpStream};
use std::convert::TryFrom;
use std::convert::TryInto;
use crate::http::request;
use std::io::Read;

use crate::http;

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Server {
        Server { address }
    }

    pub fn run(&self) {
        println!("Listening on {}", self.address);

        let listener = TcpListener::bind(&self.address).unwrap();

        loop{
            match listener.accept(){
                Ok((mut stream,_))=> {
                    let mut buffer = [0;1024];
                    match stream.read(&mut buffer){
                        Ok(_)=>{
                            println!("Received a reuqest:{}", String::from_utf8_lossy(&buffer));
                            match request::Request::try_from(&buffer[..]){
                                Ok(request)=>{}
                                Err(e) => println!("Failed to parse a request {}", e)
                            }
                          

                        },
                        Err(e) => println!("Failed to establish a connection: {}",e),
                    }
                },
                Err(e)=>println!("Failed to establish a connection: {}",e)
            }
        }
    }
}