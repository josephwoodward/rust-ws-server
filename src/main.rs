use server::Server;

mod frame;
mod server;

fn main() {
    let server = Server::new("127.0.0.1:8082".to_string());
    server.run().unwrap();
}
