use crate::request::Request;
use std::{
    io::{BufRead, BufReader},
    net::{TcpListener, TcpStream},
};

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(&mut self) -> Result<(), String> {
        let listener = match TcpListener::bind(&self.addr) {
            Ok(l) => l,
            Err(e) => return Err(format!("failed to bind connection: {e}")),
        };

        println!("server running and listening on: {}", &self.addr);

        for stream in listener.incoming() {
            let stream = stream.expect("failed to read from TCP stream");
            self.handle_connection(stream);
        }

        return Ok(());
    }

    // fn handle_connection(&mut self, mut stream: TcpStream) -> ! {
    //     // let reader = BufReader::new(stream.try_clone().expect("failed to clone stream"));
    //     // let buf_reader = BufReader::new(&mut stream);
    //     let mut buf = [0; 1024];
    //     loop {
    //         match stream.read(&mut buf) {
    //             Ok(_) => match Request::try_from(&buf[..]) {
    //                 Ok() => {}
    //                 Err(e) => {}
    //             },
    //             Err(e) => {
    //                 println!("err: {:?}", e);
    //             }
    //         }
    //     }
    // }

    fn handle_connection(&mut self, mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let http_request: Vec<String> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();
        // println!("Request: {:#?}", http_request);

        if http_request[0] == "GET /ws HTTP/1.1" {
            println!("Yes it does...");
        }

        // Opening handshake: https://datatracker.ietf.org/doc/html/rfc6455#section-1.3
        // GET /chat HTTP/1.1
        // Host: server.example.com
        // Upgrade: websocket
        // Connection: Upgrade
        // Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==
        // Origin: http://example.com
        // Sec-WebSocket-Protocol: chat, superchat
        // Sec-WebSocket-Version: 13
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn exploration() {
        // asse    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
        // EvenNumber::try_from(8);
        // assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
        // assert_eq!(EvenNumber::try_from(5), Err(()));
    }
}
