use server::Server;

mod request;
mod server;

fn main() {
    let mut server = Server::new("127.0.0.1:8082".to_string());
    server.run().unwrap();
}
