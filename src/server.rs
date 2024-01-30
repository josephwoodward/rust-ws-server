use std::net::TcpListener;

pub struct Server {
    addr: String,
    conn: Option<TcpListener>,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr, conn: None }
    }
}
