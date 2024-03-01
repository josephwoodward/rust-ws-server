// use crate::request::Request;
use base64::prelude::*;
use sha1::{Digest, Sha1};
use std::{
    io::{BufRead, BufReader, BufWriter, Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

pub struct Server {
    addr: String,
}

enum State {
    A,
    B,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(&mut self) -> Result<(), String> {
        let listener = match TcpListener::bind(&self.addr) {
            Ok(l) => l,
            Err(e) => return Err(e.to_string()),
        };

        println!("server running and listening on: {}", &self.addr);

        for stream in listener.incoming() {
            let s = stream.expect("failed to read from TCP stream");
            self.upgrade_connection(s);
        }

        return Ok(());
    }

    // fn handle_connection(&mut self, mut stream: TcpStream) {
    //     println!("handling ws connection");
    // }

    fn upgrade_connection(&mut self, mut stream: TcpStream) {
        loop {
            // let mut buf = vec![0u8; 2];
            // let b = buf[1];

            // let _ = stream
            //     .read_exact(&mut buf)
            //     .expect("could not read from buffer");
            // let reader = BufReader::new(&mut buf);
            // reader.read_exact(&mut buf);
            // println!("size is {b}");

            let buf_reader = BufReader::new(&mut stream);
            let http_request: Vec<String> = buf_reader
                .lines()
                .map(|result| result.unwrap())
                .take_while(|line| !line.is_empty())
                .collect();
            // println!("Request: {:#?}", http_request);

            let mut buf_writer = BufWriter::new(&mut stream);
            if http_request[0] != "GET /ws HTTP/1.1" {
                buf_writer
                    .write("HTTP/1.1 404 Not Found".as_bytes())
                    .unwrap();
            } else {
                if let Some(key) = find_websocket_key(http_request) {
                    let hash = generate_hash(key);
                    println!("switching protocols");
                    buf_writer
                .write(format!("HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: {hash}\r\n\r\n").as_bytes())
                .unwrap();
                }
            }

            buf_writer.flush().unwrap();

            let mut buf = vec![0u8; 2];
            let b = buf[1];

            let _ = stream
                .read_exact(&mut buf)
                .expect("could not read from buffer");
            // let reader = BufReader::new(&mut buf);
            // reader.read_exact(&mut buf);
            println!("val is: {b}");
        }

        // println!("done");

        // let mut buf = vec![0u8; 2];
        // let b = buf[1];

        // let _ = stream
        //     .read_exact(&mut buf)
        //     .expect("could not read from buffer");
        // let reader = BufReader::new(&mut buf);
        // reader.read_exact(&mut buf);
        // println!("val is: {b}");

        // Opening handshake: https://datatracker.ietf.org/doc/html/rfc6455#section-1.3
        // GET /chat HTTP/1.1
        // Host: server.example.com
        // Upgrade: websocket
        // Connection: Upgrade
        // Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==
        // Origin: http://example.com
        // Sec-WebSocket-Protocol: chat, superchat
        // Sec-WebSocket-Version: 13

        // HTTP/1.1 101 Switching Protocols
        // Upgrade: websocket
        // Connection: Upgrade
        // Sec-WebSocket-Accept: s3pPLMBiTxaQ9kYGzzhZRbK+xOo=
        // From https://tools.ietf.org/html/rfc6455#section-4.2.2
    }
}

fn generate_hash(key: String) -> String {
    let mut hasher = Sha1::new();
    hasher.input(key.to_owned() + "258EAFA5-E914-47DA-95CA-C5AB0DC85B11");

    return base64::encode(hasher.result());
}

fn find_websocket_key(request: Vec<String>) -> Option<String> {
    let key = "Sec-WebSocket-Key:";
    let l = key.len();
    for h in request.iter() {
        if h.contains(key) {
            let val = &h[l..].trim_start();
            return Some(val.to_string());
        }
    }
    return None;
}

#[cfg(test)]
mod tests {
    use crate::server::{find_websocket_key, generate_hash};

    #[test]
    fn generate_accept_hash() {
        assert_eq!(
            generate_hash("dGhlIHNhbXBsZSBub25jZQ==".to_string()),
            "b37a4f2cc0624f1690f64606cf385945b2bec4ea"
        )
    }

    #[test]
    fn find_websocket_key_test() {
        let request = vec![
            "GET /ws HTTP/1.1".to_string(),
            "b".into(),
            "Sec-WebSocket-Key: abcd".into(),
        ];
        assert_eq!(find_websocket_key(request).is_some(), true);
    }
}
