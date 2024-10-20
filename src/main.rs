use server::Server;
use http::method::Method;
use http::request::Request;

mod server;
mod http;

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}
