use server::Server;

mod server;

fn main() {
    let mut server = Server::new("127.0.0.1:8081".to_string());
    server.run();
}
